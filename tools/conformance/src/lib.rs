use std::slice::{from_raw_parts, from_raw_parts_mut};

use protokit::textformat::reflect::Registry;
use protokit::{binformat, textformat};

use crate::gen::conformance::conformance;
use crate::gen::conformance::conformance::{
    ConformanceRequestOneOfPayload, ConformanceResponse, ConformanceResponseOneOfResult, FailureSet, WireFormat,
};

pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use gen::protobuf_test_messages::proto2::test_messages_proto2::TestAllTypesProto2;
use gen::protobuf_test_messages::proto3::test_messages_proto3::TestAllTypesProto3;

#[derive(Debug)]
enum Output {
    Proto2(TestAllTypesProto2),
    Proto3(TestAllTypesProto3),
}

fn octal(d: &[u8]) -> String {
    d.iter().map(|v| format!("\\{v:0>3o}")).collect::<Vec<_>>().join("")
}

fn input(payload: ConformanceRequestOneOfPayload, proto3: bool) -> anyhow::Result<Output> {
    let txt;
    let out = match (&payload, proto3) {
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), false) => {
            txt = octal(pb);
            Ok(Output::Proto2(binformat::decode::<TestAllTypesProto2>(&pb)?))
        }
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), true) => {
            txt = octal(pb);
            Ok(Output::Proto3(binformat::decode::<TestAllTypesProto3>(&pb)?))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), false) => {
            txt = pb.clone();
            Ok(Output::Proto2(textformat::decode::<TestAllTypesProto2>(
                &pb,
                &Registry::init(gen::register_types),
            )?))
        }
        (ConformanceRequestOneOfPayload::TextPayload(pb), true) => {
            txt = pb.clone();

            Ok::<_, anyhow::Error>(Output::Proto3(textformat::decode::<TestAllTypesProto3>(
                &pb,
                &Registry::init(gen::register_types),
            )?))
        }
        (other, _) => panic!("Unknown payload {other:?}"),
    }?;

    // eprintln!("Req: {txt} => {out:?}");
    Ok(out)
}

fn output(r: anyhow::Result<Output>, wire: WireFormat) -> ConformanceResponseOneOfResult {
    let reg = Registry::init(gen::register_types);

    match (r, wire) {
        (Ok(Output::Proto2(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&v).unwrap())
        }
        (Ok(Output::Proto3(v)), WireFormat::PROTOBUF) => {
            ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&v).unwrap())
        }
        (Ok(Output::Proto2(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode(&v, &Registry::default()).unwrap();
            // println!("OUT: {out}");
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (Ok(Output::Proto3(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode(&v, &Registry::default()).unwrap();
            // println!("OUT: {out}");
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (_, WireFormat::JSON) => ConformanceResponseOneOfResult::Skipped("No json".to_string()),
        (Err(e), _) => ConformanceResponseOneOfResult::ParseError(e.to_string()),
        _ => panic!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn run_rust(data: *const u8, len: u32, odata: &mut u8, olen: u32) -> u32 {
    let data = from_raw_parts(data, len as usize);

    let req = protokit::binformat::decode::<conformance::ConformanceRequest>(&data).unwrap();

    let out = if let Some(ConformanceRequestOneOfPayload::JsonPayload(_)) = req.payload {
        ConformanceResponse {
            result: Some(ConformanceResponseOneOfResult::Skipped("No json support".to_string())),
            ..Default::default()
        }
    } else if req.message_type.contains("Proto3") || req.message_type.contains("Proto2") {
        let out = input(req.payload.unwrap(), req.message_type.contains("Proto3"));
        let data_out = output(out, req.requested_output_format);
        ConformanceResponse {
            result: Some(data_out),
            ..Default::default()
        }
    } else if req.message_type.contains("FailureSet") {
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
        panic!()
    };
    let out = binformat::encode(&out).unwrap();
    let outslice = from_raw_parts_mut(odata, olen as usize);
    outslice[0 .. out.len()].copy_from_slice(&out);
    return out.len() as u32;
}

#[test]
fn test1() {
    let a = binformat::decode::<TestAllTypesProto3>(&[
        0o202, 0o007, 0o014, 0o022, 0o012, 0o010, 0o001, 0o020, 0o001, 0o310, 0o005, 0o001, 0o310, 0o005, 0o001,
    ])
    .unwrap();
    let b = binformat::encode(&a).unwrap();
    // panic!("{a:#?}{b:#o}")
}
