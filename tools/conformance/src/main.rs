use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use protokit::reflect::Registry;
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

    eprintln!("Req: {txt} => {out:?}");
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
            ConformanceResponseOneOfResult::TextPayload(textformat::encode(&v, &reg).unwrap())
        }
        (Ok(Output::Proto3(v)), WireFormat::TEXT_FORMAT) => {
            ConformanceResponseOneOfResult::TextPayload(textformat::encode(&v, &reg).unwrap())
        }
        (_, WireFormat::JSON) => ConformanceResponseOneOfResult::Skipped("No json".to_string()),
        (Err(e), _) => ConformanceResponseOneOfResult::ParseError(e.to_string()),
        _ => panic!(),
    }
}

fn main() -> anyhow::Result<()> {
    for i in 0 .. {
        let len = stdin().read_u32::<LittleEndian>()?;
        let mut data = vec![0; len as usize];
        stdin().read_exact(&mut data).unwrap();

        // eprintln!("D  {}  \n", data.iter().map(|v| format!("\\{v:03o}")).collect::<Vec<_>>().join(""));
        std::fs::write(format!("target/{}.bin", i), &data).unwrap();
        let req = protokit::binformat::decode::<conformance::ConformanceRequest>(&data)?;

        let out = if let ConformanceRequestOneOfPayload::JsonPayload(_) = req.payload {
            ConformanceResponse {
                result: ConformanceResponseOneOfResult::Skipped("No json support".to_string()),
                ..Default::default()
            }
        } else if req.message_type.contains("Proto3") || req.message_type.contains("Proto2") {
            let out = input(req.payload, req.message_type.contains("Proto3"));
            let out = output(out, req.requested_output_format);
            ConformanceResponse {
                result: out,
                ..Default::default()
            }
        } else if req.message_type.contains("FailureSet") {
            let fs = FailureSet {
                failure: vec![],
                ..Default::default()
            };

            ConformanceResponse {
                result: ConformanceResponseOneOfResult::ProtobufPayload(binformat::encode(&fs).unwrap()),
                ..Default::default()
            }
        } else {
            panic!()
        };
        let out = binformat::encode(&out).unwrap();
        stdout().write_u32::<LittleEndian>(out.len() as _).unwrap();
        stdout().write(&out).unwrap();
        stdout().flush().unwrap()
    }
    Ok(())
}

// #[no_mangle]
// extern "C" fn run(data: *mut u8, len: usize, done: fn)

#[test]
fn test1() {
    let a = binformat::decode::<TestAllTypesProto3>(&[
        0o202, 0o007, 0o014, 0o022, 0o012, 0o010, 0o001, 0o020, 0o001, 0o310, 0o005, 0o001, 0o310, 0o005, 0o001,
    ])
    .unwrap();
    let b = binformat::encode(&a).unwrap();
    // panic!("{a:#?}{b:#o}")
}
