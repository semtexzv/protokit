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
    Proto2(TestAllTypesProto2),
    Proto3(TestAllTypesProto3),
}

fn input(payload: ConformanceRequestOneOfPayload, proto3: bool) -> anyhow::Result<Output> {
    let out = match (&payload, proto3) {
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), false) => {
            Ok::<_, anyhow::Error>(Output::Proto2(binformat::decode::<TestAllTypesProto2>(pb)?))
        }
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), true) => {
            Ok(Output::Proto3(binformat::decode::<TestAllTypesProto3>(pb)?))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), false) => {
            Ok(Output::Proto2(textformat::decode::<TestAllTypesProto2>(
                pb,
                &Registry::init(gen::register_types),
            )?))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), true) => {
            Ok(Output::Proto3(textformat::decode::<TestAllTypesProto3>(
                pb,
                &Registry::init(gen::register_types),
            )?))
        }
        (other, _) => bail!("Unknown payload {other:?}"),
    }?;
    Ok(out)
}

fn output(r: anyhow::Result<Output>, wire: WireFormat) -> ConformanceResponseOneOfResult {
    match (r, wire) {
        (Ok(Output::Proto2(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&v).unwrap())
        }
        (Ok(Output::Proto3(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&v).unwrap())
        }
        (Ok(Output::Proto2(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode(&v).unwrap();
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (Ok(Output::Proto3(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode(&v).unwrap();
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
        let data_out = output(out, req.requested_output_format);
        ConformanceResponse {
            result: Some(data_out),
            ..Default::default()
        }
    } else if msg_type.contains("FailureSet") {
        let fs = FailureSet {
            failure: vec![],

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
    let out = binformat::encode(&out).unwrap();
    let outslice = from_raw_parts_mut(odata, olen as usize);
    outslice[0 .. out.len()].copy_from_slice(&out);
    out.len() as u32
}
