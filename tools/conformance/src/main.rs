use std::io::{Read, stdin, stdout, Write};

use anyhow::*;
use byteorder::{ReadBytesExt, WriteBytesExt};

use ConformanceRequestOneOfPayload::*;
use gen::conformance::conformance;
use gen::protobuf_test_messages::proto3::test_messages_proto3;

use crate::gen::conformance::conformance::{ConformanceRequestOneOfPayload, WireFormat};


pub mod gen;

fn run(payload: ConformanceRequestOneOfPayload, proto3: bool, output: WireFormat) {
    // match (payload, proto3, output) {
    //     (ProtobufPayload(pb), false, WireFormat::PROTOBUF) ->
    // }
}

fn main() -> anyhow::Result<()> {
    loop {
        // let len = stdin().read_u32()?;
        // let mut data = vec![0; len as usize];
        // stdin().read_exact(&mut data)?;
        // let req = protokit::binformat::decode::<conformance::ConformanceRequest>(&data)?;
        // match (req.payload, req.message_type.as_str(), req.requested_output_format, WireFormat::PROTOBUF) {
        //     (ProtobufPayload(pb), "protobuf_test_messages.proto3.TestAllTypesProto3", ) => {
        //         let input = protokit::binformat::decode::<test_messages_proto3::TestAllTypesProto3>(&pb)?;
        //         let output = protokit::binformat::encode::<test_messages_proto3::TestAllTypesProto3>(&input)?;
        //
        //         let resp = conformance::ConformanceResponse {
        //             result: Default::default(),
        //             _unknown: (),
        //         };
        //
        //         stdout().write_u32(output.len() as _)?;
        //         stdout().write_all(&output)?;
        //     }
        //     TextPayload(_) => {}
        //     ConformanceRequestOneOfPayload::Unknown(_) => {}
        // }

    }
}