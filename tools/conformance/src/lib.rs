#![allow(clippy::manual_range_patterns)]
use core::slice::{from_raw_parts, from_raw_parts_mut};

use anyhow::bail;
use protokit::textformat::reflect::Registry;
use protokit::{binformat, textformat};

// pub mod gen {
//     include!(concat!(env!("OUT_DIR"), "/mod.rs"));
// }

pub mod gen;

use gen::protobuf_test_messages::proto2::test_messages_proto2::TestAllTypesProto2;
use gen::protobuf_test_messages::proto3::test_messages_proto3::TestAllTypesProto3;

use crate::gen::conformance::conformance::{
    self, ConformanceRequestOneOfPayload, ConformanceResponse, ConformanceResponseOneOfResult, FailureSet, WireFormat,
};

#[derive(Debug)]
enum Output {
    Proto2(Box<TestAllTypesProto2>),
    Proto3(Box<TestAllTypesProto3>),
}

struct AnyProxy;

impl textformat::TextFormatProxy for AnyProxy {
    fn merge<'buf>(
        &self,
        msg: &mut dyn textformat::TextProto<'buf>,
        stream: &mut textformat::InputStream<'buf>,
    ) -> textformat::Result<()> {
        if stream.token() == textformat::Token::Colon {
            stream.advance();
        }

        // Handle expanded format without braces: [type_url] { ... }
        if stream.token() == textformat::Token::LBracket {
            let key = stream.parse_key()?;
            // If it was LBracket, parse_key consumed it and returned the inner string.
            // We assume it is a type URL.
            return self.merge_expanded(msg, stream, key.into_owned());
        }

        // Handle braced format: { [type_url] { ... } } or { type_url: "...", value: ... }
        if stream.token() == textformat::Token::LBrace || stream.token() == textformat::Token::LAngle {
            let end_token = if stream.token() == textformat::Token::LBrace {
                textformat::Token::RBrace
            } else {
                textformat::Token::RAngle
            };
            stream.advance();

            while stream.token() != end_token && stream.token() != textformat::Token::EndOfFile {
                if stream.token() == textformat::Token::LBracket {
                    let key = stream.parse_key()?;
                    // It was [type_url].
                    self.merge_expanded(msg, stream, key.into_owned())?;
                    // Optional separator
                    if stream.token() == textformat::Token::Comma || stream.token() == textformat::Token::Semi {
                        stream.advance();
                    }
                    continue;
                }

                // Standard field
                msg.merge_field(stream)?;

                // Optional separator
                if stream.token() == textformat::Token::Comma || stream.token() == textformat::Token::Semi {
                    stream.advance();
                }
            }

            if stream.token() == end_token {
                stream.advance();
                Ok(())
            } else {
                Err(textformat::Error::Unexpected {
                    exp: textformat::Token::RBrace,
                    got: stream.token(),
                    rest: "End of Any message".to_string(),
                })
            }
        } else {
            // Fallback (should not happen for message)
            stream.message_fields(msg)
        }
    }

    fn encode<'buf>(&self, msg: &dyn textformat::TextProto<'buf>, stream: &mut textformat::OutputStream) {
        let any_msg = unsafe { &*(msg as *const dyn textformat::TextProto as *const gen::google::protobuf::any::Any) };

        if !any_msg.type_url.is_empty() {
            let type_name = if let Some(pos) = any_msg.type_url.rfind('/') {
                &any_msg.type_url[pos + 1..]
            } else {
                &any_msg.type_url
            };

            if let Some(inner_msg) = stream.reg.find(type_name) {
                let mut inner_msg = inner_msg.new();
                let mut buf_stream = binformat::InputStream::new(&any_msg.value);
                if inner_msg.as_bin_mut().merge_field(0, &mut buf_stream).is_ok() {
                    // Tag 0? No, merge whole message?
                    // BinProto::merge_field merges a single field. We need to merge the whole message.
                    // But BinProto doesn't have merge_message?
                    // It has merge_field.
                    // We need to loop over fields?
                    // Wait, InputStream for BinProto iterates over fields.
                    // So we loop until EOF.
                    while !buf_stream.is_empty() {
                        let tag: u32 = buf_stream._varint().unwrap();
                        // let wire = tag & 7; // Unused
                        // We need to rewind or pass tag?
                        // merge_field expects stream to be positioned AFTER tag?
                        // No, merge_field usually takes tag as argument.
                        // But does it consume the value? Yes.
                        // So we read tag, then call merge_field.
                        inner_msg.as_bin_mut().merge_field(tag, &mut buf_stream).unwrap();
                    }

                    stream.push("[");
                    stream.push(&any_msg.type_url);
                    stream.push("] {");
                    inner_msg.as_text().encode(stream);
                    stream.push("}");
                    return;
                }
            }
        }

        // Fallback
        msg.encode(stream)
    }
}

impl AnyProxy {
    fn merge_expanded<'buf>(
        &self,
        msg: &mut dyn textformat::TextProto<'buf>,
        stream: &mut textformat::InputStream<'buf>,
        type_url_inner: String,
    ) -> textformat::Result<()> {
        // stream.parse_key() already consumed the tokens.
        // And it stripped loops.

        let url_str = &type_url_inner;
        let type_name = if let Some(pos) = url_str.rfind('/') {
            &url_str[pos + 1..]
        } else {
            &url_str
        };

        if let Some(inner_msg) = stream.reg.find(type_name) {
            let mut inner_msg = inner_msg.new();
            inner_msg.as_text_mut().decode(stream)?;

            let mut stack = binformat::SizeStack::default();
            let size = inner_msg.as_bin().size(&mut stack);
            let mut buf = vec![0u8; size];
            let mut out_stream = binformat::OutputStream::new(stack, &mut buf);
            inner_msg.as_bin().encode(&mut out_stream);

            let any_msg =
                unsafe { &mut *(msg as *mut dyn textformat::TextProto as *mut gen::google::protobuf::any::Any) };
            any_msg.type_url = url_str.to_string();
            any_msg.value = buf;

            Ok(())
        } else {
            textformat::unknown(&type_url_inner)
        }
    }
}

fn register_proxies(reg: &mut Registry) {
    gen::register_types(reg);
    reg.register_proxy("google.protobuf.Any", Box::new(AnyProxy));
}

fn input(payload: ConformanceRequestOneOfPayload, proto3: bool) -> anyhow::Result<Output> {
    match (&payload, proto3) {
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), false) => {
            let msg = binformat::decode::<TestAllTypesProto2>(pb)?;
            Ok(Output::Proto2(Box::new(msg)))
        }
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), true) => {
            let msg = binformat::decode::<TestAllTypesProto3>(pb)?;
            Ok(Output::Proto3(Box::new(msg)))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), false) => {
            let msg = textformat::decode::<TestAllTypesProto2>(pb, &Registry::init(register_proxies));
            let msg = msg?;
            Ok(Output::Proto2(Box::new(msg)))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), true) => {
            let msg = textformat::decode::<TestAllTypesProto3>(pb, &Registry::init(register_proxies))?;
            Ok(Output::Proto3(Box::new(msg)))
        }
        (other, _) => bail!("Unknown payload {other:?}"),
    }
}

fn output(r: anyhow::Result<Output>, wire: WireFormat, print_unknown_fields: bool) -> ConformanceResponseOneOfResult {
    let options = textformat::EncodeOptions { print_unknown_fields };
    match (r, wire) {
        (Ok(Output::Proto2(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&*v).unwrap())
        }
        (Ok(Output::Proto3(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&*v).unwrap())
        }
        (Ok(Output::Proto2(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode_with_options(&*v, &Registry::default(), options).unwrap();
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (Ok(Output::Proto3(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode_with_options(&*v, &Registry::default(), options).unwrap();
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (_, WireFormat::JSON) => ConformanceResponseOneOfResult::Skipped("No json".to_string()),
        (Err(e), _) => ConformanceResponseOneOfResult::ParseError(e.to_string()),
        (Ok(other), _) => ConformanceResponseOneOfResult::ParseError(format!("{:?}", other)),
    }
}

/// Run a rust test case
///
/// # Safety
/// Trust me
#[no_mangle]

pub unsafe extern "C" fn run_rust(data: *const u8, len: u32, odata: &mut u8, olen: u32) -> u32 {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let data = from_raw_parts(data, len as usize);

        let req = protokit::binformat::decode::<conformance::ConformanceRequest>(data).unwrap();
        let msg_type = req.message_type;
        let out = if let Some(ConformanceRequestOneOfPayload::JsonPayload(_)) = req.payload {
            ConformanceResponse {
                result: Some(ConformanceResponseOneOfResult::Skipped("No json support".to_string())),
                ..Default::default()
            }
        } else if msg_type.contains("Proto3") || msg_type.contains("Proto2") {
            let out = input(req.payload.unwrap(), msg_type.contains("Proto3"));
            let data_out = output(out, req.requested_output_format, req.print_unknown_fields);
            ConformanceResponse {
                result: Some(data_out),
                ..Default::default()
            }
        } else if msg_type.contains("FailureSet") {
            let fs = FailureSet {
                test: vec![],

                ..Default::default()
            };

            ConformanceResponse {
                result: Some(ConformanceResponseOneOfResult::ProtobufPayload(
                    binformat::encode(&fs).unwrap(),
                )),
                ..Default::default()
            }
        } else {
            ConformanceResponse {
                result: Some(ConformanceResponseOneOfResult::RuntimeError(format!(
                    "Unknown req: {:?}",
                    msg_type
                ))),
                ..Default::default()
            }
        };
        binformat::encode(&out).unwrap()
    }));

    match result {
        Ok(out) => {
            let outslice = from_raw_parts_mut(odata, olen as usize);
            if out.len() > outslice.len() {
                eprintln!("Response buffer too small!");
                return 0;
            }
            outslice[0..out.len()].copy_from_slice(&out);
            out.len() as u32
        }
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                format!("Panic: {}", s)
            } else if let Some(s) = e.downcast_ref::<String>() {
                format!("Panic: {}", s)
            } else {
                "Panic: unknown".to_string()
            };
            eprintln!("{}", msg);
            0
        }
    }
}
