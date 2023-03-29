#![feature(prelude_import)]

#[prelude_import]
use std::prelude::rust_2021::*;

#[macro_use]
extern crate std;

use std::slice::{from_raw_parts, from_raw_parts_mut};
use protokit::textformat::reflect::Registry;
use protokit::{binformat, textformat};
use crate::gen::conformance::conformance;
use crate::gen::conformance::conformance::{
    ConformanceRequestOneOfPayload, ConformanceResponse, ConformanceResponseOneOfResult,
    FailureSet, WireFormat,
};

pub mod gen {
    pub mod conformance {
        pub mod conformance {
            use ::protokit::*;

            pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

            pub struct WireFormat(pub u32);

            #[automatically_derived]
            impl ::core::default::Default for WireFormat {
                #[inline]
                fn default() -> WireFormat {
                    WireFormat(::core::default::Default::default())
                }
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for WireFormat {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WireFormat", &&self.0)
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for WireFormat {
                #[inline]
                fn clone(&self) -> WireFormat {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }

            #[automatically_derived]
            impl ::core::marker::Copy for WireFormat {}

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for WireFormat {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for WireFormat {
                #[inline]
                fn eq(&self, other: &WireFormat) -> bool {
                    self.0 == other.0
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralEq for WireFormat {}

            #[automatically_derived]
            impl ::core::cmp::Eq for WireFormat {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }

            #[automatically_derived]
            impl ::core::cmp::PartialOrd for WireFormat {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &WireFormat,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }

            #[automatically_derived]
            impl ::core::cmp::Ord for WireFormat {
                #[inline]
                fn cmp(&self, other: &WireFormat) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }

            impl WireFormat {
                pub const UNSPECIFIED: WireFormat = WireFormat(0u32);
                pub const PROTOBUF: WireFormat = WireFormat(1u32);
                pub const JSON: WireFormat = WireFormat(2u32);
                pub const JSPB: WireFormat = WireFormat(3u32);
                pub const TEXT_FORMAT: WireFormat = WireFormat(4u32);
            }

            impl From<u32> for WireFormat {
                fn from(v: u32) -> Self {
                    Self(v)
                }
            }

            impl From<WireFormat> for u32 {
                fn from(v: WireFormat) -> Self {
                    v.0
                }
            }

            impl textformat::TextField for WireFormat {
                fn merge_value(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "UNSPECIFIED" => *self = Self::from(0u32),
                        "PROTOBUF" => *self = Self::from(1u32),
                        "JSON" => *self = Self::from(2u32),
                        "JSPB" => *self = Self::from(3u32),
                        "TEXT_FORMAT" => *self = Self::from(4u32),
                        name => return textformat::unknown(name),
                    }
                    Ok(())
                }
                fn emit_value(&self, stream: &mut textformat::OutputStream) {
                    match self.0 {
                        0u32 => stream.ident("UNSPECIFIED"),
                        1u32 => stream.ident("PROTOBUF"),
                        2u32 => stream.ident("JSON"),
                        3u32 => stream.ident("JSPB"),
                        4u32 => stream.ident("TEXT_FORMAT"),
                        other => stream.disp(&other),
                    }
                }
            }

            pub struct TestCategory(pub u32);

            #[automatically_derived]
            impl ::core::default::Default for TestCategory {
                #[inline]
                fn default() -> TestCategory {
                    TestCategory(::core::default::Default::default())
                }
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for TestCategory {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TestCategory", &&self.0)
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for TestCategory {
                #[inline]
                fn clone(&self) -> TestCategory {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }

            #[automatically_derived]
            impl ::core::marker::Copy for TestCategory {}

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TestCategory {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for TestCategory {
                #[inline]
                fn eq(&self, other: &TestCategory) -> bool {
                    self.0 == other.0
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralEq for TestCategory {}

            #[automatically_derived]
            impl ::core::cmp::Eq for TestCategory {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }

            #[automatically_derived]
            impl ::core::cmp::PartialOrd for TestCategory {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &TestCategory,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                }
            }

            #[automatically_derived]
            impl ::core::cmp::Ord for TestCategory {
                #[inline]
                fn cmp(&self, other: &TestCategory) -> ::core::cmp::Ordering {
                    ::core::cmp::Ord::cmp(&self.0, &other.0)
                }
            }

            impl TestCategory {
                pub const UNSPECIFIED_TEST: TestCategory = TestCategory(0u32);
                pub const BINARY_TEST: TestCategory = TestCategory(1u32);
                pub const JSON_TEST: TestCategory = TestCategory(2u32);
                pub const JSON_IGNORE_UNKNOWN_PARSING_TEST: TestCategory = TestCategory(3u32);
                pub const JSPB_TEST: TestCategory = TestCategory(4u32);
                pub const TEXT_FORMAT_TEST: TestCategory = TestCategory(5u32);
            }

            impl From<u32> for TestCategory {
                fn from(v: u32) -> Self {
                    Self(v)
                }
            }

            impl From<TestCategory> for u32 {
                fn from(v: TestCategory) -> Self {
                    v.0
                }
            }

            impl textformat::TextField for TestCategory {
                fn merge_value(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "UNSPECIFIED_TEST" => *self = Self::from(0u32),
                        "BINARY_TEST" => *self = Self::from(1u32),
                        "JSON_TEST" => *self = Self::from(2u32),
                        "JSON_IGNORE_UNKNOWN_PARSING_TEST" => *self = Self::from(3u32),
                        "JSPB_TEST" => *self = Self::from(4u32),
                        "TEXT_FORMAT_TEST" => *self = Self::from(5u32),
                        name => return textformat::unknown(name),
                    }
                    Ok(())
                }
                fn emit_value(&self, stream: &mut textformat::OutputStream) {
                    match self.0 {
                        0u32 => stream.ident("UNSPECIFIED_TEST"),
                        1u32 => stream.ident("BINARY_TEST"),
                        2u32 => stream.ident("JSON_TEST"),
                        3u32 => stream.ident("JSON_IGNORE_UNKNOWN_PARSING_TEST"),
                        4u32 => stream.ident("JSPB_TEST"),
                        5u32 => stream.ident("TEXT_FORMAT_TEST"),
                        other => stream.disp(&other),
                    }
                }
            }

            pub struct FailureSet {
                #[field(1u32, "failure", string, repeated)]
                pub failure: Vec<String>,
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for FailureSet {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailureSet",
                        "failure",
                        &&self.failure,
                    )
                }
            }

            #[automatically_derived]
            impl ::core::default::Default for FailureSet {
                #[inline]
                fn default() -> FailureSet {
                    FailureSet {
                        failure: ::core::default::Default::default(),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for FailureSet {
                #[inline]
                fn clone(&self) -> FailureSet {
                    FailureSet {
                        failure: ::core::clone::Clone::clone(&self.failure),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for FailureSet {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for FailureSet {
                #[inline]
                fn eq(&self, other: &FailureSet) -> bool {
                    self.failure == other.failure
                }
            }

            impl binformat::BinProto for FailureSet {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        10u32 => binformat::merge_repeated(
                            &mut self.failure,
                            stream,
                            binformat::InputStream::string,
                        ),
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    binformat::emit_repeated(
                        &self.failure,
                        10u32,
                        stream,
                        binformat::OutputStream::string,
                    );
                }
            }

            impl textformat::TextProto for FailureSet {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "failure" => self.failure.merge_text(stream),
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    stream.emit_field("failure", &self.failure);
                }
            }

            pub enum ConformanceRequestOneOfPayload {
                #[field(1u32, "protobuf_payload", bytes, raw)]
                ProtobufPayload(Vec<u8>),
                #[field(2u32, "json_payload", string, raw)]
                JsonPayload(String),
                #[field(7u32, "jspb_payload", string, raw)]
                JspbPayload(String),
                #[field(8u32, "text_payload", string, raw)]
                TextPayload(String),
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for ConformanceRequestOneOfPayload {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ConformanceRequestOneOfPayload::ProtobufPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "ProtobufPayload",
                                &__self_0,
                            )
                        }
                        ConformanceRequestOneOfPayload::JsonPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "JsonPayload",
                                &__self_0,
                            )
                        }
                        ConformanceRequestOneOfPayload::JspbPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "JspbPayload",
                                &__self_0,
                            )
                        }
                        ConformanceRequestOneOfPayload::TextPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "TextPayload",
                                &__self_0,
                            )
                        }
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for ConformanceRequestOneOfPayload {
                #[inline]
                fn clone(&self) -> ConformanceRequestOneOfPayload {
                    match self {
                        ConformanceRequestOneOfPayload::ProtobufPayload(__self_0) => {
                            ConformanceRequestOneOfPayload::ProtobufPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceRequestOneOfPayload::JsonPayload(__self_0) => {
                            ConformanceRequestOneOfPayload::JsonPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceRequestOneOfPayload::JspbPayload(__self_0) => {
                            ConformanceRequestOneOfPayload::JspbPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceRequestOneOfPayload::TextPayload(__self_0) => {
                            ConformanceRequestOneOfPayload::TextPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ConformanceRequestOneOfPayload {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for ConformanceRequestOneOfPayload {
                #[inline]
                fn eq(&self, other: &ConformanceRequestOneOfPayload) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                        (
                            ConformanceRequestOneOfPayload::ProtobufPayload(__self_0),
                            ConformanceRequestOneOfPayload::ProtobufPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceRequestOneOfPayload::JsonPayload(__self_0),
                            ConformanceRequestOneOfPayload::JsonPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceRequestOneOfPayload::JspbPayload(__self_0),
                            ConformanceRequestOneOfPayload::JspbPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceRequestOneOfPayload::TextPayload(__self_0),
                            ConformanceRequestOneOfPayload::TextPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                }
            }

            impl ConformanceRequestOneOfPayload {
                fn make_ProtobufPayload_mut(&mut self) -> &mut Vec<u8> {
                    loop {
                        match self {
                            Self::ProtobufPayload(v) => return v,
                            _ => *self = Self::ProtobufPayload(Default::default()),
                        }
                    }
                }
                fn make_JsonPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::JsonPayload(v) => return v,
                            _ => *self = Self::JsonPayload(Default::default()),
                        }
                    }
                }
                fn make_JspbPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::JspbPayload(v) => return v,
                            _ => *self = Self::JspbPayload(Default::default()),
                        }
                    }
                }
                fn make_TextPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::TextPayload(v) => return v,
                            _ => *self = Self::TextPayload(Default::default()),
                        }
                    }
                }
            }

            impl binformat::BinProto for ConformanceRequestOneOfPayload {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        10u32 => binformat::merge_single(
                            self.make_ProtobufPayload_mut(),
                            stream,
                            binformat::InputStream::bytes,
                        ),
                        18u32 => binformat::merge_single(
                            self.make_JsonPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        58u32 => binformat::merge_single(
                            self.make_JspbPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        66u32 => binformat::merge_single(
                            self.make_TextPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    match self {
                        Self::ProtobufPayload(v) => {
                            binformat::emit_raw(v, 10u32, stream, binformat::OutputStream::bytes);
                        }
                        Self::JsonPayload(v) => {
                            binformat::emit_raw(v, 18u32, stream, binformat::OutputStream::string);
                        }
                        Self::JspbPayload(v) => {
                            binformat::emit_raw(v, 58u32, stream, binformat::OutputStream::string);
                        }
                        Self::TextPayload(v) => {
                            binformat::emit_raw(v, 66u32, stream, binformat::OutputStream::string);
                        }
                    }
                }
            }

            impl textformat::TextProto for ConformanceRequestOneOfPayload {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "protobuf_payload" => self.make_ProtobufPayload_mut().merge_text(stream),
                        "json_payload" => self.make_JsonPayload_mut().merge_text(stream),
                        "jspb_payload" => self.make_JspbPayload_mut().merge_text(stream),
                        "text_payload" => self.make_TextPayload_mut().merge_text(stream),
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    match self {
                        Self::ProtobufPayload(v) => stream.emit_field("protobuf_payload", v),
                        Self::JsonPayload(v) => stream.emit_field("json_payload", v),
                        Self::JspbPayload(v) => stream.emit_field("jspb_payload", v),
                        Self::TextPayload(v) => stream.emit_field("text_payload", v),
                    }
                }
            }

            impl Default for ConformanceRequestOneOfPayload {
                fn default() -> Self {
                    Self::ProtobufPayload(Default::default())
                }
            }

            pub struct ConformanceRequest {
                #[field(3u32, "requested_output_format", protoenum, singular)]
                pub requested_output_format: WireFormat,
                #[field(4u32, "message_type", string, singular)]
                pub message_type: String,
                #[field(5u32, "test_category", protoenum, singular)]
                pub test_category: TestCategory,
                #[field(6u32, "jspb_encoding_options", nested, optional)]
                pub jspb_encoding_options: Option<Box<JspbEncodingConfig>>,
                #[field(9u32, "print_unknown_fields", bool, singular)]
                pub print_unknown_fields: bool,
                #[oneof([1u32, 2u32, 7u32, 8u32, ], ["protobuf_payload", "json_payload", "jspb_payload", "text_payload",])]
                pub payload: Option<ConformanceRequestOneOfPayload>,
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for ConformanceRequest {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "requested_output_format",
                        "message_type",
                        "test_category",
                        "jspb_encoding_options",
                        "print_unknown_fields",
                        "payload",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &&self.requested_output_format,
                        &&self.message_type,
                        &&self.test_category,
                        &&self.jspb_encoding_options,
                        &&self.print_unknown_fields,
                        &&self.payload,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "ConformanceRequest",
                        names,
                        values,
                    )
                }
            }

            #[automatically_derived]
            impl ::core::default::Default for ConformanceRequest {
                #[inline]
                fn default() -> ConformanceRequest {
                    ConformanceRequest {
                        requested_output_format: ::core::default::Default::default(),
                        message_type: ::core::default::Default::default(),
                        test_category: ::core::default::Default::default(),
                        jspb_encoding_options: ::core::default::Default::default(),
                        print_unknown_fields: ::core::default::Default::default(),
                        payload: ::core::default::Default::default(),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for ConformanceRequest {
                #[inline]
                fn clone(&self) -> ConformanceRequest {
                    ConformanceRequest {
                        requested_output_format: ::core::clone::Clone::clone(
                            &self.requested_output_format,
                        ),
                        message_type: ::core::clone::Clone::clone(&self.message_type),
                        test_category: ::core::clone::Clone::clone(&self.test_category),
                        jspb_encoding_options: ::core::clone::Clone::clone(
                            &self.jspb_encoding_options,
                        ),
                        print_unknown_fields: ::core::clone::Clone::clone(
                            &self.print_unknown_fields,
                        ),
                        payload: ::core::clone::Clone::clone(&self.payload),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ConformanceRequest {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for ConformanceRequest {
                #[inline]
                fn eq(&self, other: &ConformanceRequest) -> bool {
                    self.requested_output_format == other.requested_output_format
                        & &self.message_type == other.message_type
                        & &self.test_category == other.test_category
                        & &self.jspb_encoding_options == other.jspb_encoding_options
                        & &self.print_unknown_fields == other.print_unknown_fields
                        & &self.payload == other.payload
                }
            }

            impl binformat::BinProto for ConformanceRequest {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        24u32 => binformat::merge_single(
                            &mut self.requested_output_format,
                            stream,
                            binformat::InputStream::protoenum,
                        ),
                        34u32 => binformat::merge_single(
                            &mut self.message_type,
                            stream,
                            binformat::InputStream::string,
                        ),
                        40u32 => binformat::merge_single(
                            &mut self.test_category,
                            stream,
                            binformat::InputStream::protoenum,
                        ),
                        50u32 => binformat::merge_optional(
                            &mut self.jspb_encoding_options,
                            stream,
                            binformat::InputStream::nested,
                        ),
                        72u32 => binformat::merge_single(
                            &mut self.print_unknown_fields,
                            stream,
                            binformat::InputStream::bool,
                        ),
                        8u32 | 9u32 | 10u32 | 11u32 | 12u32 | 13u32 | 14u32 | 15u32 | 16u32
                        | 17u32 | 18u32 | 19u32 | 20u32 | 21u32 | 22u32 | 23u32 | 56u32 | 57u32
                        | 58u32 | 59u32 | 60u32 | 61u32 | 62u32 | 63u32 | 64u32 | 65u32 | 66u32
                        | 67u32 | 68u32 | 69u32 | 70u32 | 71u32 => {
                            binformat::merge_oneof(&mut self.payload, tag, stream)
                        }
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    binformat::emit_single(
                        &self.requested_output_format,
                        24u32,
                        stream,
                        binformat::OutputStream::protoenum,
                    );
                    binformat::emit_single(
                        &self.message_type,
                        34u32,
                        stream,
                        binformat::OutputStream::string,
                    );
                    binformat::emit_single(
                        &self.test_category,
                        40u32,
                        stream,
                        binformat::OutputStream::protoenum,
                    );
                    binformat::emit_optional(
                        &self.jspb_encoding_options,
                        50u32,
                        stream,
                        binformat::OutputStream::nested,
                    );
                    binformat::emit_single(
                        &self.print_unknown_fields,
                        72u32,
                        stream,
                        binformat::OutputStream::bool,
                    );
                    binformat::emit_oneof(&self.payload, stream);
                }
            }

            impl textformat::TextProto for ConformanceRequest {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "requested_output_format" => {
                            self.requested_output_format.merge_text(stream)
                        }
                        "message_type" => self.message_type.merge_text(stream),
                        "test_category" => self.test_category.merge_text(stream),
                        "jspb_encoding_options" => self.jspb_encoding_options.merge_text(stream),
                        "print_unknown_fields" => self.print_unknown_fields.merge_text(stream),
                        "protobuf_payload" | "json_payload" | "jspb_payload" | "text_payload" => {
                            self.payload.merge_text(stream)
                        }
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    stream.emit_field("requested_output_format", &self.requested_output_format);
                    stream.emit_field("message_type", &self.message_type);
                    stream.emit_field("test_category", &self.test_category);
                    stream.emit_field("jspb_encoding_options", &self.jspb_encoding_options);
                    stream.emit_field("print_unknown_fields", &self.print_unknown_fields);
                    stream.emit_oneof(&self.payload);
                }
            }

            pub enum ConformanceResponseOneOfResult {
                #[field(1u32, "parse_error", string, raw)]
                ParseError(String),
                #[field(6u32, "serialize_error", string, raw)]
                SerializeError(String),
                #[field(9u32, "timeout_error", string, raw)]
                TimeoutError(String),
                #[field(2u32, "runtime_error", string, raw)]
                RuntimeError(String),
                #[field(3u32, "protobuf_payload", bytes, raw)]
                ProtobufPayload(Vec<u8>),
                #[field(4u32, "json_payload", string, raw)]
                JsonPayload(String),
                #[field(5u32, "skipped", string, raw)]
                Skipped(String),
                #[field(7u32, "jspb_payload", string, raw)]
                JspbPayload(String),
                #[field(8u32, "text_payload", string, raw)]
                TextPayload(String),
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for ConformanceResponseOneOfResult {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ConformanceResponseOneOfResult::ParseError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "ParseError",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::SerializeError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "SerializeError",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::TimeoutError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "TimeoutError",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::RuntimeError(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "RuntimeError",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::ProtobufPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "ProtobufPayload",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::JsonPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "JsonPayload",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::Skipped(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Skipped", &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::JspbPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "JspbPayload",
                                &__self_0,
                            )
                        }
                        ConformanceResponseOneOfResult::TextPayload(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "TextPayload",
                                &__self_0,
                            )
                        }
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for ConformanceResponseOneOfResult {
                #[inline]
                fn clone(&self) -> ConformanceResponseOneOfResult {
                    match self {
                        ConformanceResponseOneOfResult::ParseError(__self_0) => {
                            ConformanceResponseOneOfResult::ParseError(::core::clone::Clone::clone(
                                __self_0,
                            ))
                        }
                        ConformanceResponseOneOfResult::SerializeError(__self_0) => {
                            ConformanceResponseOneOfResult::SerializeError(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::TimeoutError(__self_0) => {
                            ConformanceResponseOneOfResult::TimeoutError(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::RuntimeError(__self_0) => {
                            ConformanceResponseOneOfResult::RuntimeError(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::ProtobufPayload(__self_0) => {
                            ConformanceResponseOneOfResult::ProtobufPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::JsonPayload(__self_0) => {
                            ConformanceResponseOneOfResult::JsonPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::Skipped(__self_0) => {
                            ConformanceResponseOneOfResult::Skipped(::core::clone::Clone::clone(
                                __self_0,
                            ))
                        }
                        ConformanceResponseOneOfResult::JspbPayload(__self_0) => {
                            ConformanceResponseOneOfResult::JspbPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ConformanceResponseOneOfResult::TextPayload(__self_0) => {
                            ConformanceResponseOneOfResult::TextPayload(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ConformanceResponseOneOfResult {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for ConformanceResponseOneOfResult {
                #[inline]
                fn eq(&self, other: &ConformanceResponseOneOfResult) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                        && match (self, other) {
                        (
                            ConformanceResponseOneOfResult::ParseError(__self_0),
                            ConformanceResponseOneOfResult::ParseError(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::SerializeError(__self_0),
                            ConformanceResponseOneOfResult::SerializeError(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::TimeoutError(__self_0),
                            ConformanceResponseOneOfResult::TimeoutError(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::RuntimeError(__self_0),
                            ConformanceResponseOneOfResult::RuntimeError(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::ProtobufPayload(__self_0),
                            ConformanceResponseOneOfResult::ProtobufPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::JsonPayload(__self_0),
                            ConformanceResponseOneOfResult::JsonPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::Skipped(__self_0),
                            ConformanceResponseOneOfResult::Skipped(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::JspbPayload(__self_0),
                            ConformanceResponseOneOfResult::JspbPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConformanceResponseOneOfResult::TextPayload(__self_0),
                            ConformanceResponseOneOfResult::TextPayload(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                }
            }

            impl ConformanceResponseOneOfResult {
                fn make_ParseError_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::ParseError(v) => return v,
                            _ => *self = Self::ParseError(Default::default()),
                        }
                    }
                }
                fn make_SerializeError_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::SerializeError(v) => return v,
                            _ => *self = Self::SerializeError(Default::default()),
                        }
                    }
                }
                fn make_TimeoutError_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::TimeoutError(v) => return v,
                            _ => *self = Self::TimeoutError(Default::default()),
                        }
                    }
                }
                fn make_RuntimeError_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::RuntimeError(v) => return v,
                            _ => *self = Self::RuntimeError(Default::default()),
                        }
                    }
                }
                fn make_ProtobufPayload_mut(&mut self) -> &mut Vec<u8> {
                    loop {
                        match self {
                            Self::ProtobufPayload(v) => return v,
                            _ => *self = Self::ProtobufPayload(Default::default()),
                        }
                    }
                }
                fn make_JsonPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::JsonPayload(v) => return v,
                            _ => *self = Self::JsonPayload(Default::default()),
                        }
                    }
                }
                fn make_Skipped_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::Skipped(v) => return v,
                            _ => *self = Self::Skipped(Default::default()),
                        }
                    }
                }
                fn make_JspbPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::JspbPayload(v) => return v,
                            _ => *self = Self::JspbPayload(Default::default()),
                        }
                    }
                }
                fn make_TextPayload_mut(&mut self) -> &mut String {
                    loop {
                        match self {
                            Self::TextPayload(v) => return v,
                            _ => *self = Self::TextPayload(Default::default()),
                        }
                    }
                }
            }

            impl binformat::BinProto for ConformanceResponseOneOfResult {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        10u32 => binformat::merge_single(
                            self.make_ParseError_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        50u32 => binformat::merge_single(
                            self.make_SerializeError_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        74u32 => binformat::merge_single(
                            self.make_TimeoutError_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        18u32 => binformat::merge_single(
                            self.make_RuntimeError_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        26u32 => binformat::merge_single(
                            self.make_ProtobufPayload_mut(),
                            stream,
                            binformat::InputStream::bytes,
                        ),
                        34u32 => binformat::merge_single(
                            self.make_JsonPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        42u32 => binformat::merge_single(
                            self.make_Skipped_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        58u32 => binformat::merge_single(
                            self.make_JspbPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        66u32 => binformat::merge_single(
                            self.make_TextPayload_mut(),
                            stream,
                            binformat::InputStream::string,
                        ),
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    match self {
                        Self::ParseError(v) => {
                            binformat::emit_raw(v, 10u32, stream, binformat::OutputStream::string);
                        }
                        Self::SerializeError(v) => {
                            binformat::emit_raw(v, 50u32, stream, binformat::OutputStream::string);
                        }
                        Self::TimeoutError(v) => {
                            binformat::emit_raw(v, 74u32, stream, binformat::OutputStream::string);
                        }
                        Self::RuntimeError(v) => {
                            binformat::emit_raw(v, 18u32, stream, binformat::OutputStream::string);
                        }
                        Self::ProtobufPayload(v) => {
                            binformat::emit_raw(v, 26u32, stream, binformat::OutputStream::bytes);
                        }
                        Self::JsonPayload(v) => {
                            binformat::emit_raw(v, 34u32, stream, binformat::OutputStream::string);
                        }
                        Self::Skipped(v) => {
                            binformat::emit_raw(v, 42u32, stream, binformat::OutputStream::string);
                        }
                        Self::JspbPayload(v) => {
                            binformat::emit_raw(v, 58u32, stream, binformat::OutputStream::string);
                        }
                        Self::TextPayload(v) => {
                            binformat::emit_raw(v, 66u32, stream, binformat::OutputStream::string);
                        }
                    }
                }
            }

            impl textformat::TextProto for ConformanceResponseOneOfResult {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "parse_error" => self.make_ParseError_mut().merge_text(stream),
                        "serialize_error" => self.make_SerializeError_mut().merge_text(stream),
                        "timeout_error" => self.make_TimeoutError_mut().merge_text(stream),
                        "runtime_error" => self.make_RuntimeError_mut().merge_text(stream),
                        "protobuf_payload" => self.make_ProtobufPayload_mut().merge_text(stream),
                        "json_payload" => self.make_JsonPayload_mut().merge_text(stream),
                        "skipped" => self.make_Skipped_mut().merge_text(stream),
                        "jspb_payload" => self.make_JspbPayload_mut().merge_text(stream),
                        "text_payload" => self.make_TextPayload_mut().merge_text(stream),
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    match self {
                        Self::ParseError(v) => stream.emit_field("parse_error", v),
                        Self::SerializeError(v) => stream.emit_field("serialize_error", v),
                        Self::TimeoutError(v) => stream.emit_field("timeout_error", v),
                        Self::RuntimeError(v) => stream.emit_field("runtime_error", v),
                        Self::ProtobufPayload(v) => stream.emit_field("protobuf_payload", v),
                        Self::JsonPayload(v) => stream.emit_field("json_payload", v),
                        Self::Skipped(v) => stream.emit_field("skipped", v),
                        Self::JspbPayload(v) => stream.emit_field("jspb_payload", v),
                        Self::TextPayload(v) => stream.emit_field("text_payload", v),
                    }
                }
            }

            impl Default for ConformanceResponseOneOfResult {
                fn default() -> Self {
                    Self::ParseError(Default::default())
                }
            }

            pub struct ConformanceResponse {
                #[oneof([1u32, 6u32, 9u32, 2u32, 3u32, 4u32, 5u32, 7u32, 8u32, ], ["parse_error", "serialize_error", "timeout_error", "runtime_error", "protobuf_payload", "json_payload", "skipped", "jspb_payload", "text_payload", ])]
                pub result: Option<ConformanceResponseOneOfResult>,
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for ConformanceResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ConformanceResponse",
                        "result",
                        &&self.result,
                    )
                }
            }

            #[automatically_derived]
            impl ::core::default::Default for ConformanceResponse {
                #[inline]
                fn default() -> ConformanceResponse {
                    ConformanceResponse {
                        result: ::core::default::Default::default(),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for ConformanceResponse {
                #[inline]
                fn clone(&self) -> ConformanceResponse {
                    ConformanceResponse {
                        result: ::core::clone::Clone::clone(&self.result),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ConformanceResponse {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for ConformanceResponse {
                #[inline]
                fn eq(&self, other: &ConformanceResponse) -> bool {
                    self.result == other.result
                }
            }

            impl binformat::BinProto for ConformanceResponse {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        8u32 | 9u32 | 10u32 | 11u32 | 12u32 | 13u32 | 14u32 | 15u32 | 48u32
                        | 49u32 | 50u32 | 51u32 | 52u32 | 53u32 | 54u32 | 55u32 | 72u32 | 73u32
                        | 74u32 | 75u32 | 76u32 | 77u32 | 78u32 | 79u32 | 16u32 | 17u32 | 18u32
                        | 19u32 | 20u32 | 21u32 | 22u32 | 23u32 | 24u32 | 25u32 | 26u32 | 27u32
                        | 28u32 | 29u32 | 30u32 | 31u32 | 32u32 | 33u32 | 34u32 | 35u32 | 36u32
                        | 37u32 | 38u32 | 39u32 | 40u32 | 41u32 | 42u32 | 43u32 | 44u32 | 45u32
                        | 46u32 | 47u32 | 56u32 | 57u32 | 58u32 | 59u32 | 60u32 | 61u32 | 62u32
                        | 63u32 | 64u32 | 65u32 | 66u32 | 67u32 | 68u32 | 69u32 | 70u32 | 71u32 => {
                            binformat::merge_oneof(&mut self.result, tag, stream)
                        }
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    binformat::emit_oneof(&self.result, stream);
                }
            }

            impl textformat::TextProto for ConformanceResponse {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "parse_error" | "serialize_error" | "timeout_error" | "runtime_error"
                        | "protobuf_payload" | "json_payload" | "skipped" | "jspb_payload"
                        | "text_payload" => self.result.merge_text(stream),
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    stream.emit_oneof(&self.result);
                }
            }

            pub struct JspbEncodingConfig {
                #[field(1u32, "use_jspb_array_any_format", bool, singular)]
                pub use_jspb_array_any_format: bool,
            }

            #[automatically_derived]
            impl ::core::fmt::Debug for JspbEncodingConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "JspbEncodingConfig",
                        "use_jspb_array_any_format",
                        &&self.use_jspb_array_any_format,
                    )
                }
            }

            #[automatically_derived]
            impl ::core::default::Default for JspbEncodingConfig {
                #[inline]
                fn default() -> JspbEncodingConfig {
                    JspbEncodingConfig {
                        use_jspb_array_any_format: ::core::default::Default::default(),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::clone::Clone for JspbEncodingConfig {
                #[inline]
                fn clone(&self) -> JspbEncodingConfig {
                    JspbEncodingConfig {
                        use_jspb_array_any_format: ::core::clone::Clone::clone(
                            &self.use_jspb_array_any_format,
                        ),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for JspbEncodingConfig {}

            #[automatically_derived]
            impl ::core::cmp::PartialEq for JspbEncodingConfig {
                #[inline]
                fn eq(&self, other: &JspbEncodingConfig) -> bool {
                    self.use_jspb_array_any_format == other.use_jspb_array_any_format
                }
            }

            impl binformat::BinProto for JspbEncodingConfig {
                fn merge_field(
                    &mut self,
                    tag: u32,
                    stream: &mut binformat::InputStream,
                ) -> binformat::Result<()> {
                    match tag {
                        8u32 => binformat::merge_single(
                            &mut self.use_jspb_array_any_format,
                            stream,
                            binformat::InputStream::bool,
                        ),
                        other => stream.skip(other),
                    }
                }
                fn encode(&self, stream: &mut binformat::OutputStream) {
                    binformat::emit_single(
                        &self.use_jspb_array_any_format,
                        8u32,
                        stream,
                        binformat::OutputStream::bool,
                    );
                }
            }

            impl textformat::TextProto for JspbEncodingConfig {
                fn merge_field(
                    &mut self,
                    stream: &mut textformat::InputStream,
                ) -> textformat::Result<()> {
                    match stream.field() {
                        "use_jspb_array_any_format" => {
                            self.use_jspb_array_any_format.merge_text(stream)
                        }
                        name => textformat::unknown(name),
                    }
                }
                fn encode(&self, stream: &mut textformat::OutputStream) {
                    stream.emit_field("use_jspb_array_any_format", &self.use_jspb_array_any_format);
                }
            }
        }

        pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
            conformance::register_types(registry);
        }
    }

    pub mod protobuf_test_messages {
        pub mod proto2 {
            pub mod test_messages_proto2 {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct TestAllTypesProto2NestedEnum(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2NestedEnum {
                    #[inline]
                    fn default() -> TestAllTypesProto2NestedEnum {
                        TestAllTypesProto2NestedEnum(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2NestedEnum {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TestAllTypesProto2NestedEnum",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2NestedEnum {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2NestedEnum {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for TestAllTypesProto2NestedEnum {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2NestedEnum {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2NestedEnum {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2NestedEnum) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for TestAllTypesProto2NestedEnum {}

                #[automatically_derived]
                impl ::core::cmp::Eq for TestAllTypesProto2NestedEnum {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for TestAllTypesProto2NestedEnum {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &TestAllTypesProto2NestedEnum,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for TestAllTypesProto2NestedEnum {
                    #[inline]
                    fn cmp(&self, other: &TestAllTypesProto2NestedEnum) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl TestAllTypesProto2NestedEnum {
                    pub const FOO: TestAllTypesProto2NestedEnum =
                        TestAllTypesProto2NestedEnum(0u32);
                    pub const BAR: TestAllTypesProto2NestedEnum =
                        TestAllTypesProto2NestedEnum(1u32);
                    pub const BAZ: TestAllTypesProto2NestedEnum =
                        TestAllTypesProto2NestedEnum(2u32);
                    pub const NEG: TestAllTypesProto2NestedEnum =
                        TestAllTypesProto2NestedEnum(4294967295u32);
                }

                impl From<u32> for TestAllTypesProto2NestedEnum {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<TestAllTypesProto2NestedEnum> for u32 {
                    fn from(v: TestAllTypesProto2NestedEnum) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for TestAllTypesProto2NestedEnum {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "FOO" => *self = Self::from(0u32),
                            "BAR" => *self = Self::from(1u32),
                            "BAZ" => *self = Self::from(2u32),
                            "NEG" => *self = Self::from(4294967295u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("FOO"),
                            1u32 => stream.ident("BAR"),
                            2u32 => stream.ident("BAZ"),
                            4294967295u32 => stream.ident("NEG"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct ForeignEnumProto2(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for ForeignEnumProto2 {
                    #[inline]
                    fn default() -> ForeignEnumProto2 {
                        ForeignEnumProto2(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ForeignEnumProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ForeignEnumProto2",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ForeignEnumProto2 {
                    #[inline]
                    fn clone(&self) -> ForeignEnumProto2 {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for ForeignEnumProto2 {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ForeignEnumProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ForeignEnumProto2 {
                    #[inline]
                    fn eq(&self, other: &ForeignEnumProto2) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for ForeignEnumProto2 {}

                #[automatically_derived]
                impl ::core::cmp::Eq for ForeignEnumProto2 {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for ForeignEnumProto2 {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &ForeignEnumProto2,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for ForeignEnumProto2 {
                    #[inline]
                    fn cmp(&self, other: &ForeignEnumProto2) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl ForeignEnumProto2 {
                    pub const FOREIGN_FOO: ForeignEnumProto2 = ForeignEnumProto2(0u32);
                    pub const FOREIGN_BAR: ForeignEnumProto2 = ForeignEnumProto2(1u32);
                    pub const FOREIGN_BAZ: ForeignEnumProto2 = ForeignEnumProto2(2u32);
                }

                impl From<u32> for ForeignEnumProto2 {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<ForeignEnumProto2> for u32 {
                    fn from(v: ForeignEnumProto2) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for ForeignEnumProto2 {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "FOREIGN_FOO" => *self = Self::from(0u32),
                            "FOREIGN_BAR" => *self = Self::from(1u32),
                            "FOREIGN_BAZ" => *self = Self::from(2u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("FOREIGN_FOO"),
                            1u32 => stream.ident("FOREIGN_BAR"),
                            2u32 => stream.ident("FOREIGN_BAZ"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct EnumOnlyProto2Bool(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for EnumOnlyProto2Bool {
                    #[inline]
                    fn default() -> EnumOnlyProto2Bool {
                        EnumOnlyProto2Bool(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for EnumOnlyProto2Bool {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "EnumOnlyProto2Bool",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for EnumOnlyProto2Bool {
                    #[inline]
                    fn clone(&self) -> EnumOnlyProto2Bool {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for EnumOnlyProto2Bool {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for EnumOnlyProto2Bool {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for EnumOnlyProto2Bool {
                    #[inline]
                    fn eq(&self, other: &EnumOnlyProto2Bool) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for EnumOnlyProto2Bool {}

                #[automatically_derived]
                impl ::core::cmp::Eq for EnumOnlyProto2Bool {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for EnumOnlyProto2Bool {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &EnumOnlyProto2Bool,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for EnumOnlyProto2Bool {
                    #[inline]
                    fn cmp(&self, other: &EnumOnlyProto2Bool) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl EnumOnlyProto2Bool {
                    pub const kFalse: EnumOnlyProto2Bool = EnumOnlyProto2Bool(0u32);
                    pub const kTrue: EnumOnlyProto2Bool = EnumOnlyProto2Bool(1u32);
                }

                impl From<u32> for EnumOnlyProto2Bool {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<EnumOnlyProto2Bool> for u32 {
                    fn from(v: EnumOnlyProto2Bool) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for EnumOnlyProto2Bool {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "kFalse" => *self = Self::from(0u32),
                            "kTrue" => *self = Self::from(1u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("kFalse"),
                            1u32 => stream.ident("kTrue"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub enum TestAllTypesProto2OneOfOneofField {
                    #[field(111u32, "oneof_uint32", varint, raw)]
                    OneofUint32(u32),
                    #[field(112u32, "oneof_nested_message", nested, raw)]
                    OneofNestedMessage(TestAllTypesProto2NestedMessage),
                    #[field(113u32, "oneof_string", string, raw)]
                    OneofString(String),
                    #[field(114u32, "oneof_bytes", bytes, raw)]
                    OneofBytes(Vec<u8>),
                    #[field(115u32, "oneof_bool", bool, raw)]
                    OneofBool(bool),
                    #[field(116u32, "oneof_uint64", varint, raw)]
                    OneofUint64(u64),
                    #[field(117u32, "oneof_float", fixed32, raw)]
                    OneofFloat(f32),
                    #[field(118u32, "oneof_double", fixed64, raw)]
                    OneofDouble(f64),
                    #[field(119u32, "oneof_enum", protoenum, raw)]
                    OneofEnum(TestAllTypesProto2NestedEnum),
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2OneOfOneofField {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match self {
                            TestAllTypesProto2OneOfOneofField::OneofUint32(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofUint32",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofNestedMessage(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofNestedMessage",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofString(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofString",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofBytes(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofBytes",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofBool(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofBool",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofUint64(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofUint64",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofFloat(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofFloat",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofDouble(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofDouble",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofEnum(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofEnum",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2OneOfOneofField {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2OneOfOneofField {
                        match self {
                            TestAllTypesProto2OneOfOneofField::OneofUint32(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofUint32(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofNestedMessage(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofNestedMessage(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofString(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofString(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofBytes(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofBytes(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofBool(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofBool(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofUint64(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofUint64(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofFloat(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofFloat(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofDouble(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofDouble(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto2OneOfOneofField::OneofEnum(__self_0) => {
                                TestAllTypesProto2OneOfOneofField::OneofEnum(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2OneOfOneofField {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2OneOfOneofField {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2OneOfOneofField) -> bool {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                        __self_tag == __arg1_tag
                            && match (self, other) {
                            (
                                TestAllTypesProto2OneOfOneofField::OneofUint32(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofUint32(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofNestedMessage(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofNestedMessage(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofString(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofString(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofBytes(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofBytes(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofBool(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofBool(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofUint64(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofUint64(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofFloat(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofFloat(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofDouble(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofDouble(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto2OneOfOneofField::OneofEnum(__self_0),
                                TestAllTypesProto2OneOfOneofField::OneofEnum(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    }
                }

                impl TestAllTypesProto2OneOfOneofField {
                    fn make_OneofUint32_mut(&mut self) -> &mut u32 {
                        loop {
                            match self {
                                Self::OneofUint32(v) => return v,
                                _ => *self = Self::OneofUint32(Default::default()),
                            }
                        }
                    }
                    fn make_OneofNestedMessage_mut(
                        &mut self,
                    ) -> &mut TestAllTypesProto2NestedMessage {
                        loop {
                            match self {
                                Self::OneofNestedMessage(v) => return v,
                                _ => *self = Self::OneofNestedMessage(Default::default()),
                            }
                        }
                    }
                    fn make_OneofString_mut(&mut self) -> &mut String {
                        loop {
                            match self {
                                Self::OneofString(v) => return v,
                                _ => *self = Self::OneofString(Default::default()),
                            }
                        }
                    }
                    fn make_OneofBytes_mut(&mut self) -> &mut Vec<u8> {
                        loop {
                            match self {
                                Self::OneofBytes(v) => return v,
                                _ => *self = Self::OneofBytes(Default::default()),
                            }
                        }
                    }
                    fn make_OneofBool_mut(&mut self) -> &mut bool {
                        loop {
                            match self {
                                Self::OneofBool(v) => return v,
                                _ => *self = Self::OneofBool(Default::default()),
                            }
                        }
                    }
                    fn make_OneofUint64_mut(&mut self) -> &mut u64 {
                        loop {
                            match self {
                                Self::OneofUint64(v) => return v,
                                _ => *self = Self::OneofUint64(Default::default()),
                            }
                        }
                    }
                    fn make_OneofFloat_mut(&mut self) -> &mut f32 {
                        loop {
                            match self {
                                Self::OneofFloat(v) => return v,
                                _ => *self = Self::OneofFloat(Default::default()),
                            }
                        }
                    }
                    fn make_OneofDouble_mut(&mut self) -> &mut f64 {
                        loop {
                            match self {
                                Self::OneofDouble(v) => return v,
                                _ => *self = Self::OneofDouble(Default::default()),
                            }
                        }
                    }
                    fn make_OneofEnum_mut(&mut self) -> &mut TestAllTypesProto2NestedEnum {
                        loop {
                            match self {
                                Self::OneofEnum(v) => return v,
                                _ => *self = Self::OneofEnum(Default::default()),
                            }
                        }
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2OneOfOneofField {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            888u32 => binformat::merge_single(
                                self.make_OneofUint32_mut(),
                                stream,
                                binformat::InputStream::varint,
                            ),
                            898u32 => binformat::merge_single(
                                self.make_OneofNestedMessage_mut(),
                                stream,
                                binformat::InputStream::nested,
                            ),
                            906u32 => binformat::merge_single(
                                self.make_OneofString_mut(),
                                stream,
                                binformat::InputStream::string,
                            ),
                            914u32 => binformat::merge_single(
                                self.make_OneofBytes_mut(),
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            920u32 => binformat::merge_single(
                                self.make_OneofBool_mut(),
                                stream,
                                binformat::InputStream::bool,
                            ),
                            928u32 => binformat::merge_single(
                                self.make_OneofUint64_mut(),
                                stream,
                                binformat::InputStream::varint,
                            ),
                            941u32 => binformat::merge_single(
                                self.make_OneofFloat_mut(),
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            945u32 => binformat::merge_single(
                                self.make_OneofDouble_mut(),
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            952u32 => binformat::merge_single(
                                self.make_OneofEnum_mut(),
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        match self {
                            Self::OneofUint32(v) => {
                                binformat::emit_raw(
                                    v,
                                    888u32,
                                    stream,
                                    binformat::OutputStream::varint,
                                );
                            }
                            Self::OneofNestedMessage(v) => {
                                binformat::emit_raw(
                                    v,
                                    898u32,
                                    stream,
                                    binformat::OutputStream::nested,
                                );
                            }
                            Self::OneofString(v) => {
                                binformat::emit_raw(
                                    v,
                                    906u32,
                                    stream,
                                    binformat::OutputStream::string,
                                );
                            }
                            Self::OneofBytes(v) => {
                                binformat::emit_raw(
                                    v,
                                    914u32,
                                    stream,
                                    binformat::OutputStream::bytes,
                                );
                            }
                            Self::OneofBool(v) => {
                                binformat::emit_raw(
                                    v,
                                    920u32,
                                    stream,
                                    binformat::OutputStream::bool,
                                );
                            }
                            Self::OneofUint64(v) => {
                                binformat::emit_raw(
                                    v,
                                    928u32,
                                    stream,
                                    binformat::OutputStream::varint,
                                );
                            }
                            Self::OneofFloat(v) => {
                                binformat::emit_raw(
                                    v,
                                    941u32,
                                    stream,
                                    binformat::OutputStream::fixed32,
                                );
                            }
                            Self::OneofDouble(v) => {
                                binformat::emit_raw(
                                    v,
                                    945u32,
                                    stream,
                                    binformat::OutputStream::fixed64,
                                );
                            }
                            Self::OneofEnum(v) => {
                                binformat::emit_raw(
                                    v,
                                    952u32,
                                    stream,
                                    binformat::OutputStream::protoenum,
                                );
                            }
                        }
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2OneOfOneofField {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "oneof_uint32" => self.make_OneofUint32_mut().merge_text(stream),
                            "oneof_nested_message" => {
                                self.make_OneofNestedMessage_mut().merge_text(stream)
                            }
                            "oneof_string" => self.make_OneofString_mut().merge_text(stream),
                            "oneof_bytes" => self.make_OneofBytes_mut().merge_text(stream),
                            "oneof_bool" => self.make_OneofBool_mut().merge_text(stream),
                            "oneof_uint64" => self.make_OneofUint64_mut().merge_text(stream),
                            "oneof_float" => self.make_OneofFloat_mut().merge_text(stream),
                            "oneof_double" => self.make_OneofDouble_mut().merge_text(stream),
                            "oneof_enum" => self.make_OneofEnum_mut().merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        match self {
                            Self::OneofUint32(v) => stream.emit_field("oneof_uint32", v),
                            Self::OneofNestedMessage(v) => {
                                stream.emit_field("oneof_nested_message", v)
                            }
                            Self::OneofString(v) => stream.emit_field("oneof_string", v),
                            Self::OneofBytes(v) => stream.emit_field("oneof_bytes", v),
                            Self::OneofBool(v) => stream.emit_field("oneof_bool", v),
                            Self::OneofUint64(v) => stream.emit_field("oneof_uint64", v),
                            Self::OneofFloat(v) => stream.emit_field("oneof_float", v),
                            Self::OneofDouble(v) => stream.emit_field("oneof_double", v),
                            Self::OneofEnum(v) => stream.emit_field("oneof_enum", v),
                        }
                    }
                }

                impl Default for TestAllTypesProto2OneOfOneofField {
                    fn default() -> Self {
                        Self::OneofUint32(Default::default())
                    }
                }

                pub struct TestAllTypesProto2 {
                    #[field(1u32, "optional_int32", varint, optional)]
                    pub optional_int32: Option<i32>,
                    #[field(2u32, "optional_int64", varint, optional)]
                    pub optional_int64: Option<i64>,
                    #[field(3u32, "optional_uint32", varint, optional)]
                    pub optional_uint32: Option<u32>,
                    #[field(4u32, "optional_uint64", varint, optional)]
                    pub optional_uint64: Option<u64>,
                    #[field(5u32, "optional_sint32", sigint, optional)]
                    pub optional_sint32: Option<i32>,
                    #[field(6u32, "optional_sint64", sigint, optional)]
                    pub optional_sint64: Option<i64>,
                    #[field(7u32, "optional_fixed32", fixed32, optional)]
                    pub optional_fixed32: Option<u32>,
                    #[field(8u32, "optional_fixed64", fixed64, optional)]
                    pub optional_fixed64: Option<u64>,
                    #[field(9u32, "optional_sfixed32", fixed32, optional)]
                    pub optional_sfixed32: Option<i32>,
                    #[field(10u32, "optional_sfixed64", fixed64, optional)]
                    pub optional_sfixed64: Option<i64>,
                    #[field(11u32, "optional_float", fixed32, optional)]
                    pub optional_float: Option<f32>,
                    #[field(12u32, "optional_double", fixed64, optional)]
                    pub optional_double: Option<f64>,
                    #[field(13u32, "optional_bool", bool, optional)]
                    pub optional_bool: Option<bool>,
                    #[field(14u32, "optional_string", string, optional)]
                    pub optional_string: Option<String>,
                    #[field(15u32, "optional_bytes", bytes, optional)]
                    pub optional_bytes: Option<Vec<u8>>,
                    #[field(18u32, "optional_nested_message", nested, optional)]
                    pub optional_nested_message: Option<Box<TestAllTypesProto2NestedMessage>>,
                    #[field(19u32, "optional_foreign_message", nested, optional)]
                    pub optional_foreign_message: Option<Box<ForeignMessageProto2>>,
                    #[field(21u32, "optional_nested_enum", protoenum, optional)]
                    pub optional_nested_enum: Option<TestAllTypesProto2NestedEnum>,
                    #[field(22u32, "optional_foreign_enum", protoenum, optional)]
                    pub optional_foreign_enum: Option<ForeignEnumProto2>,
                    #[field(24u32, "optional_string_piece", string, optional)]
                    pub optional_string_piece: Option<String>,
                    #[field(25u32, "optional_cord", string, optional)]
                    pub optional_cord: Option<String>,
                    #[field(27u32, "recursive_message", nested, optional)]
                    pub recursive_message: Option<Box<TestAllTypesProto2>>,
                    #[field(31u32, "repeated_int32", varint, repeated)]
                    pub repeated_int32: Vec<i32>,
                    #[field(32u32, "repeated_int64", varint, repeated)]
                    pub repeated_int64: Vec<i64>,
                    #[field(33u32, "repeated_uint32", varint, repeated)]
                    pub repeated_uint32: Vec<u32>,
                    #[field(34u32, "repeated_uint64", varint, repeated)]
                    pub repeated_uint64: Vec<u64>,
                    #[field(35u32, "repeated_sint32", sigint, repeated)]
                    pub repeated_sint32: Vec<i32>,
                    #[field(36u32, "repeated_sint64", sigint, repeated)]
                    pub repeated_sint64: Vec<i64>,
                    #[field(37u32, "repeated_fixed32", fixed32, repeated)]
                    pub repeated_fixed32: Vec<u32>,
                    #[field(38u32, "repeated_fixed64", fixed64, repeated)]
                    pub repeated_fixed64: Vec<u64>,
                    #[field(39u32, "repeated_sfixed32", fixed32, repeated)]
                    pub repeated_sfixed32: Vec<i32>,
                    #[field(40u32, "repeated_sfixed64", fixed64, repeated)]
                    pub repeated_sfixed64: Vec<i64>,
                    #[field(41u32, "repeated_float", fixed32, repeated)]
                    pub repeated_float: Vec<f32>,
                    #[field(42u32, "repeated_double", fixed64, repeated)]
                    pub repeated_double: Vec<f64>,
                    #[field(43u32, "repeated_bool", bool, repeated)]
                    pub repeated_bool: Vec<bool>,
                    #[field(44u32, "repeated_string", string, repeated)]
                    pub repeated_string: Vec<String>,
                    #[field(45u32, "repeated_bytes", bytes, repeated)]
                    pub repeated_bytes: Vec<Vec<u8>>,
                    #[field(48u32, "repeated_nested_message", nested, repeated)]
                    pub repeated_nested_message: Vec<TestAllTypesProto2NestedMessage>,
                    #[field(49u32, "repeated_foreign_message", nested, repeated)]
                    pub repeated_foreign_message: Vec<ForeignMessageProto2>,
                    #[field(51u32, "repeated_nested_enum", protoenum, repeated)]
                    pub repeated_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
                    #[field(52u32, "repeated_foreign_enum", protoenum, repeated)]
                    pub repeated_foreign_enum: Vec<ForeignEnumProto2>,
                    #[field(54u32, "repeated_string_piece", string, repeated)]
                    pub repeated_string_piece: Vec<String>,
                    #[field(55u32, "repeated_cord", string, repeated)]
                    pub repeated_cord: Vec<String>,
                    #[field(75u32, "packed_int32", varint, repeated)]
                    pub packed_int32: Vec<i32>,
                    #[field(76u32, "packed_int64", varint, repeated)]
                    pub packed_int64: Vec<i64>,
                    #[field(77u32, "packed_uint32", varint, repeated)]
                    pub packed_uint32: Vec<u32>,
                    #[field(78u32, "packed_uint64", varint, repeated)]
                    pub packed_uint64: Vec<u64>,
                    #[field(79u32, "packed_sint32", sigint, repeated)]
                    pub packed_sint32: Vec<i32>,
                    #[field(80u32, "packed_sint64", sigint, repeated)]
                    pub packed_sint64: Vec<i64>,
                    #[field(81u32, "packed_fixed32", fixed32, repeated)]
                    pub packed_fixed32: Vec<u32>,
                    #[field(82u32, "packed_fixed64", fixed64, repeated)]
                    pub packed_fixed64: Vec<u64>,
                    #[field(83u32, "packed_sfixed32", fixed32, repeated)]
                    pub packed_sfixed32: Vec<i32>,
                    #[field(84u32, "packed_sfixed64", fixed64, repeated)]
                    pub packed_sfixed64: Vec<i64>,
                    #[field(85u32, "packed_float", fixed32, repeated)]
                    pub packed_float: Vec<f32>,
                    #[field(86u32, "packed_double", fixed64, repeated)]
                    pub packed_double: Vec<f64>,
                    #[field(87u32, "packed_bool", bool, repeated)]
                    pub packed_bool: Vec<bool>,
                    #[field(88u32, "packed_nested_enum", protoenum, repeated)]
                    pub packed_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
                    #[field(89u32, "unpacked_int32", varint, repeated)]
                    pub unpacked_int32: Vec<i32>,
                    #[field(90u32, "unpacked_int64", varint, repeated)]
                    pub unpacked_int64: Vec<i64>,
                    #[field(91u32, "unpacked_uint32", varint, repeated)]
                    pub unpacked_uint32: Vec<u32>,
                    #[field(92u32, "unpacked_uint64", varint, repeated)]
                    pub unpacked_uint64: Vec<u64>,
                    #[field(93u32, "unpacked_sint32", sigint, repeated)]
                    pub unpacked_sint32: Vec<i32>,
                    #[field(94u32, "unpacked_sint64", sigint, repeated)]
                    pub unpacked_sint64: Vec<i64>,
                    #[field(95u32, "unpacked_fixed32", fixed32, repeated)]
                    pub unpacked_fixed32: Vec<u32>,
                    #[field(96u32, "unpacked_fixed64", fixed64, repeated)]
                    pub unpacked_fixed64: Vec<u64>,
                    #[field(97u32, "unpacked_sfixed32", fixed32, repeated)]
                    pub unpacked_sfixed32: Vec<i32>,
                    #[field(98u32, "unpacked_sfixed64", fixed64, repeated)]
                    pub unpacked_sfixed64: Vec<i64>,
                    #[field(99u32, "unpacked_float", fixed32, repeated)]
                    pub unpacked_float: Vec<f32>,
                    #[field(100u32, "unpacked_double", fixed64, repeated)]
                    pub unpacked_double: Vec<f64>,
                    #[field(101u32, "unpacked_bool", bool, repeated)]
                    pub unpacked_bool: Vec<bool>,
                    #[field(102u32, "unpacked_nested_enum", protoenum, repeated)]
                    pub unpacked_nested_enum: Vec<TestAllTypesProto2NestedEnum>,
                    #[field(56u32, "map_int32_int32", map(varint, varint), singular)]
                    pub map_int32_int32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(57u32, "map_int64_int64", map(varint, varint), singular)]
                    pub map_int64_int64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(58u32, "map_uint32_uint32", map(varint, varint), singular)]
                    pub map_uint32_uint32: ::std::collections::BTreeMap<u32, u32>,
                    #[field(59u32, "map_uint64_uint64", map(varint, varint), singular)]
                    pub map_uint64_uint64: ::std::collections::BTreeMap<u64, u64>,
                    #[field(60u32, "map_sint32_sint32", map(sigint, sigint), singular)]
                    pub map_sint32_sint32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(61u32, "map_sint64_sint64", map(sigint, sigint), singular)]
                    pub map_sint64_sint64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(62u32, "map_fixed32_fixed32", map(fixed32, fixed32), singular)]
                    pub map_fixed32_fixed32: ::std::collections::BTreeMap<u32, u32>,
                    #[field(63u32, "map_fixed64_fixed64", map(fixed64, fixed64), singular)]
                    pub map_fixed64_fixed64: ::std::collections::BTreeMap<u64, u64>,
                    #[field(64u32, "map_sfixed32_sfixed32", map(fixed32, fixed32), singular)]
                    pub map_sfixed32_sfixed32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(65u32, "map_sfixed64_sfixed64", map(fixed64, fixed64), singular)]
                    pub map_sfixed64_sfixed64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(66u32, "map_int32_float", map(varint, fixed32), singular)]
                    pub map_int32_float: ::std::collections::BTreeMap<i32, f32>,
                    #[field(67u32, "map_int32_double", map(varint, fixed64), singular)]
                    pub map_int32_double: ::std::collections::BTreeMap<i32, f64>,
                    #[field(68u32, "map_bool_bool", map(bool, bool), singular)]
                    pub map_bool_bool: ::std::collections::BTreeMap<bool, bool>,
                    #[field(69u32, "map_string_string", map(string, string), singular)]
                    pub map_string_string: ::std::collections::BTreeMap<String, String>,
                    #[field(70u32, "map_string_bytes", map(string, bytes), singular)]
                    pub map_string_bytes: ::std::collections::BTreeMap<String, Vec<u8>>,
                    #[field(71u32, "map_string_nested_message", map(string, nested), singular)]
                    pub map_string_nested_message:
                    ::std::collections::BTreeMap<String, TestAllTypesProto2NestedMessage>,
                    #[field(72u32, "map_string_foreign_message", map(string, nested), singular)]
                    pub map_string_foreign_message:
                    ::std::collections::BTreeMap<String, ForeignMessageProto2>,
                    #[field(73u32, "map_string_nested_enum", map(string, protoenum), singular)]
                    pub map_string_nested_enum:
                    ::std::collections::BTreeMap<String, TestAllTypesProto2NestedEnum>,
                    #[field(74u32, "map_string_foreign_enum", map(string, protoenum), singular)]
                    pub map_string_foreign_enum:
                    ::std::collections::BTreeMap<String, ForeignEnumProto2>,
                    #[field(201u32, "Data", nested, optional)]
                    pub Data: Option<Box<TestAllTypesProto2Data>>,
                    #[field(241u32, "default_int32", varint, optional)]
                    pub default_int32: Option<i32>,
                    #[field(242u32, "default_int64", varint, optional)]
                    pub default_int64: Option<i64>,
                    #[field(243u32, "default_uint32", varint, optional)]
                    pub default_uint32: Option<u32>,
                    #[field(244u32, "default_uint64", varint, optional)]
                    pub default_uint64: Option<u64>,
                    #[field(245u32, "default_sint32", sigint, optional)]
                    pub default_sint32: Option<i32>,
                    #[field(246u32, "default_sint64", sigint, optional)]
                    pub default_sint64: Option<i64>,
                    #[field(247u32, "default_fixed32", fixed32, optional)]
                    pub default_fixed32: Option<u32>,
                    #[field(248u32, "default_fixed64", fixed64, optional)]
                    pub default_fixed64: Option<u64>,
                    #[field(249u32, "default_sfixed32", fixed32, optional)]
                    pub default_sfixed32: Option<i32>,
                    #[field(250u32, "default_sfixed64", fixed64, optional)]
                    pub default_sfixed64: Option<i64>,
                    #[field(251u32, "default_float", fixed32, optional)]
                    pub default_float: Option<f32>,
                    #[field(252u32, "default_double", fixed64, optional)]
                    pub default_double: Option<f64>,
                    #[field(253u32, "default_bool", bool, optional)]
                    pub default_bool: Option<bool>,
                    #[field(254u32, "default_string", string, optional)]
                    pub default_string: Option<String>,
                    #[field(255u32, "default_bytes", bytes, optional)]
                    pub default_bytes: Option<Vec<u8>>,
                    #[field(401u32, "fieldname1", varint, optional)]
                    pub fieldname1: Option<i32>,
                    #[field(402u32, "field_name2", varint, optional)]
                    pub field_name2: Option<i32>,
                    #[field(403u32, "_field_name3", varint, optional)]
                    pub _field_name3: Option<i32>,
                    #[field(404u32, "field__name4_", varint, optional)]
                    pub field__name4_: Option<i32>,
                    #[field(405u32, "field0name5", varint, optional)]
                    pub field0name5: Option<i32>,
                    #[field(406u32, "field_0_name6", varint, optional)]
                    pub field_0_name6: Option<i32>,
                    #[field(407u32, "fieldName7", varint, optional)]
                    pub fieldName7: Option<i32>,
                    #[field(408u32, "FieldName8", varint, optional)]
                    pub FieldName8: Option<i32>,
                    #[field(409u32, "field_Name9", varint, optional)]
                    pub field_Name9: Option<i32>,
                    #[field(410u32, "Field_Name10", varint, optional)]
                    pub Field_Name10: Option<i32>,
                    #[field(411u32, "FIELD_NAME11", varint, optional)]
                    pub FIELD_NAME11: Option<i32>,
                    #[field(412u32, "FIELD_name12", varint, optional)]
                    pub FIELD_name12: Option<i32>,
                    #[field(413u32, "__field_name13", varint, optional)]
                    pub __field_name13: Option<i32>,
                    #[field(414u32, "__Field_name14", varint, optional)]
                    pub __Field_name14: Option<i32>,
                    #[field(415u32, "field__name15", varint, optional)]
                    pub field__name15: Option<i32>,
                    #[field(416u32, "field__Name16", varint, optional)]
                    pub field__Name16: Option<i32>,
                    #[field(417u32, "field_name17__", varint, optional)]
                    pub field_name17__: Option<i32>,
                    #[field(418u32, "Field_name18__", varint, optional)]
                    pub Field_name18__: Option<i32>,
                    #[oneof([111u32, 112u32, 113u32, 114u32, 115u32, 116u32, 117u32, 118u32, 119u32, ], ["oneof_uint32", "oneof_nested_message", "oneof_string", "oneof_bytes", "oneof_bool", "oneof_uint64", "oneof_float", "oneof_double", "oneof_enum",])]
                    pub oneof_field: Option<TestAllTypesProto2OneOfOneofField>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "optional_int32",
                            "optional_int64",
                            "optional_uint32",
                            "optional_uint64",
                            "optional_sint32",
                            "optional_sint64",
                            "optional_fixed32",
                            "optional_fixed64",
                            "optional_sfixed32",
                            "optional_sfixed64",
                            "optional_float",
                            "optional_double",
                            "optional_bool",
                            "optional_string",
                            "optional_bytes",
                            "optional_nested_message",
                            "optional_foreign_message",
                            "optional_nested_enum",
                            "optional_foreign_enum",
                            "optional_string_piece",
                            "optional_cord",
                            "recursive_message",
                            "repeated_int32",
                            "repeated_int64",
                            "repeated_uint32",
                            "repeated_uint64",
                            "repeated_sint32",
                            "repeated_sint64",
                            "repeated_fixed32",
                            "repeated_fixed64",
                            "repeated_sfixed32",
                            "repeated_sfixed64",
                            "repeated_float",
                            "repeated_double",
                            "repeated_bool",
                            "repeated_string",
                            "repeated_bytes",
                            "repeated_nested_message",
                            "repeated_foreign_message",
                            "repeated_nested_enum",
                            "repeated_foreign_enum",
                            "repeated_string_piece",
                            "repeated_cord",
                            "packed_int32",
                            "packed_int64",
                            "packed_uint32",
                            "packed_uint64",
                            "packed_sint32",
                            "packed_sint64",
                            "packed_fixed32",
                            "packed_fixed64",
                            "packed_sfixed32",
                            "packed_sfixed64",
                            "packed_float",
                            "packed_double",
                            "packed_bool",
                            "packed_nested_enum",
                            "unpacked_int32",
                            "unpacked_int64",
                            "unpacked_uint32",
                            "unpacked_uint64",
                            "unpacked_sint32",
                            "unpacked_sint64",
                            "unpacked_fixed32",
                            "unpacked_fixed64",
                            "unpacked_sfixed32",
                            "unpacked_sfixed64",
                            "unpacked_float",
                            "unpacked_double",
                            "unpacked_bool",
                            "unpacked_nested_enum",
                            "map_int32_int32",
                            "map_int64_int64",
                            "map_uint32_uint32",
                            "map_uint64_uint64",
                            "map_sint32_sint32",
                            "map_sint64_sint64",
                            "map_fixed32_fixed32",
                            "map_fixed64_fixed64",
                            "map_sfixed32_sfixed32",
                            "map_sfixed64_sfixed64",
                            "map_int32_float",
                            "map_int32_double",
                            "map_bool_bool",
                            "map_string_string",
                            "map_string_bytes",
                            "map_string_nested_message",
                            "map_string_foreign_message",
                            "map_string_nested_enum",
                            "map_string_foreign_enum",
                            "Data",
                            "default_int32",
                            "default_int64",
                            "default_uint32",
                            "default_uint64",
                            "default_sint32",
                            "default_sint64",
                            "default_fixed32",
                            "default_fixed64",
                            "default_sfixed32",
                            "default_sfixed64",
                            "default_float",
                            "default_double",
                            "default_bool",
                            "default_string",
                            "default_bytes",
                            "fieldname1",
                            "field_name2",
                            "_field_name3",
                            "field__name4_",
                            "field0name5",
                            "field_0_name6",
                            "fieldName7",
                            "FieldName8",
                            "field_Name9",
                            "Field_Name10",
                            "FIELD_NAME11",
                            "FIELD_name12",
                            "__field_name13",
                            "__Field_name14",
                            "field__name15",
                            "field__Name16",
                            "field_name17__",
                            "Field_name18__",
                            "oneof_field",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &&self.optional_int32,
                            &&self.optional_int64,
                            &&self.optional_uint32,
                            &&self.optional_uint64,
                            &&self.optional_sint32,
                            &&self.optional_sint64,
                            &&self.optional_fixed32,
                            &&self.optional_fixed64,
                            &&self.optional_sfixed32,
                            &&self.optional_sfixed64,
                            &&self.optional_float,
                            &&self.optional_double,
                            &&self.optional_bool,
                            &&self.optional_string,
                            &&self.optional_bytes,
                            &&self.optional_nested_message,
                            &&self.optional_foreign_message,
                            &&self.optional_nested_enum,
                            &&self.optional_foreign_enum,
                            &&self.optional_string_piece,
                            &&self.optional_cord,
                            &&self.recursive_message,
                            &&self.repeated_int32,
                            &&self.repeated_int64,
                            &&self.repeated_uint32,
                            &&self.repeated_uint64,
                            &&self.repeated_sint32,
                            &&self.repeated_sint64,
                            &&self.repeated_fixed32,
                            &&self.repeated_fixed64,
                            &&self.repeated_sfixed32,
                            &&self.repeated_sfixed64,
                            &&self.repeated_float,
                            &&self.repeated_double,
                            &&self.repeated_bool,
                            &&self.repeated_string,
                            &&self.repeated_bytes,
                            &&self.repeated_nested_message,
                            &&self.repeated_foreign_message,
                            &&self.repeated_nested_enum,
                            &&self.repeated_foreign_enum,
                            &&self.repeated_string_piece,
                            &&self.repeated_cord,
                            &&self.packed_int32,
                            &&self.packed_int64,
                            &&self.packed_uint32,
                            &&self.packed_uint64,
                            &&self.packed_sint32,
                            &&self.packed_sint64,
                            &&self.packed_fixed32,
                            &&self.packed_fixed64,
                            &&self.packed_sfixed32,
                            &&self.packed_sfixed64,
                            &&self.packed_float,
                            &&self.packed_double,
                            &&self.packed_bool,
                            &&self.packed_nested_enum,
                            &&self.unpacked_int32,
                            &&self.unpacked_int64,
                            &&self.unpacked_uint32,
                            &&self.unpacked_uint64,
                            &&self.unpacked_sint32,
                            &&self.unpacked_sint64,
                            &&self.unpacked_fixed32,
                            &&self.unpacked_fixed64,
                            &&self.unpacked_sfixed32,
                            &&self.unpacked_sfixed64,
                            &&self.unpacked_float,
                            &&self.unpacked_double,
                            &&self.unpacked_bool,
                            &&self.unpacked_nested_enum,
                            &&self.map_int32_int32,
                            &&self.map_int64_int64,
                            &&self.map_uint32_uint32,
                            &&self.map_uint64_uint64,
                            &&self.map_sint32_sint32,
                            &&self.map_sint64_sint64,
                            &&self.map_fixed32_fixed32,
                            &&self.map_fixed64_fixed64,
                            &&self.map_sfixed32_sfixed32,
                            &&self.map_sfixed64_sfixed64,
                            &&self.map_int32_float,
                            &&self.map_int32_double,
                            &&self.map_bool_bool,
                            &&self.map_string_string,
                            &&self.map_string_bytes,
                            &&self.map_string_nested_message,
                            &&self.map_string_foreign_message,
                            &&self.map_string_nested_enum,
                            &&self.map_string_foreign_enum,
                            &&self.Data,
                            &&self.default_int32,
                            &&self.default_int64,
                            &&self.default_uint32,
                            &&self.default_uint64,
                            &&self.default_sint32,
                            &&self.default_sint64,
                            &&self.default_fixed32,
                            &&self.default_fixed64,
                            &&self.default_sfixed32,
                            &&self.default_sfixed64,
                            &&self.default_float,
                            &&self.default_double,
                            &&self.default_bool,
                            &&self.default_string,
                            &&self.default_bytes,
                            &&self.fieldname1,
                            &&self.field_name2,
                            &&self._field_name3,
                            &&self.field__name4_,
                            &&self.field0name5,
                            &&self.field_0_name6,
                            &&self.fieldName7,
                            &&self.FieldName8,
                            &&self.field_Name9,
                            &&self.Field_Name10,
                            &&self.FIELD_NAME11,
                            &&self.FIELD_name12,
                            &&self.__field_name13,
                            &&self.__Field_name14,
                            &&self.field__name15,
                            &&self.field__Name16,
                            &&self.field_name17__,
                            &&self.Field_name18__,
                            &&self.oneof_field,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "TestAllTypesProto2",
                            names,
                            values,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2 {
                    #[inline]
                    fn default() -> TestAllTypesProto2 {
                        TestAllTypesProto2 {
                            optional_int32: ::core::default::Default::default(),
                            optional_int64: ::core::default::Default::default(),
                            optional_uint32: ::core::default::Default::default(),
                            optional_uint64: ::core::default::Default::default(),
                            optional_sint32: ::core::default::Default::default(),
                            optional_sint64: ::core::default::Default::default(),
                            optional_fixed32: ::core::default::Default::default(),
                            optional_fixed64: ::core::default::Default::default(),
                            optional_sfixed32: ::core::default::Default::default(),
                            optional_sfixed64: ::core::default::Default::default(),
                            optional_float: ::core::default::Default::default(),
                            optional_double: ::core::default::Default::default(),
                            optional_bool: ::core::default::Default::default(),
                            optional_string: ::core::default::Default::default(),
                            optional_bytes: ::core::default::Default::default(),
                            optional_nested_message: ::core::default::Default::default(),
                            optional_foreign_message: ::core::default::Default::default(),
                            optional_nested_enum: ::core::default::Default::default(),
                            optional_foreign_enum: ::core::default::Default::default(),
                            optional_string_piece: ::core::default::Default::default(),
                            optional_cord: ::core::default::Default::default(),
                            recursive_message: ::core::default::Default::default(),
                            repeated_int32: ::core::default::Default::default(),
                            repeated_int64: ::core::default::Default::default(),
                            repeated_uint32: ::core::default::Default::default(),
                            repeated_uint64: ::core::default::Default::default(),
                            repeated_sint32: ::core::default::Default::default(),
                            repeated_sint64: ::core::default::Default::default(),
                            repeated_fixed32: ::core::default::Default::default(),
                            repeated_fixed64: ::core::default::Default::default(),
                            repeated_sfixed32: ::core::default::Default::default(),
                            repeated_sfixed64: ::core::default::Default::default(),
                            repeated_float: ::core::default::Default::default(),
                            repeated_double: ::core::default::Default::default(),
                            repeated_bool: ::core::default::Default::default(),
                            repeated_string: ::core::default::Default::default(),
                            repeated_bytes: ::core::default::Default::default(),
                            repeated_nested_message: ::core::default::Default::default(),
                            repeated_foreign_message: ::core::default::Default::default(),
                            repeated_nested_enum: ::core::default::Default::default(),
                            repeated_foreign_enum: ::core::default::Default::default(),
                            repeated_string_piece: ::core::default::Default::default(),
                            repeated_cord: ::core::default::Default::default(),
                            packed_int32: ::core::default::Default::default(),
                            packed_int64: ::core::default::Default::default(),
                            packed_uint32: ::core::default::Default::default(),
                            packed_uint64: ::core::default::Default::default(),
                            packed_sint32: ::core::default::Default::default(),
                            packed_sint64: ::core::default::Default::default(),
                            packed_fixed32: ::core::default::Default::default(),
                            packed_fixed64: ::core::default::Default::default(),
                            packed_sfixed32: ::core::default::Default::default(),
                            packed_sfixed64: ::core::default::Default::default(),
                            packed_float: ::core::default::Default::default(),
                            packed_double: ::core::default::Default::default(),
                            packed_bool: ::core::default::Default::default(),
                            packed_nested_enum: ::core::default::Default::default(),
                            unpacked_int32: ::core::default::Default::default(),
                            unpacked_int64: ::core::default::Default::default(),
                            unpacked_uint32: ::core::default::Default::default(),
                            unpacked_uint64: ::core::default::Default::default(),
                            unpacked_sint32: ::core::default::Default::default(),
                            unpacked_sint64: ::core::default::Default::default(),
                            unpacked_fixed32: ::core::default::Default::default(),
                            unpacked_fixed64: ::core::default::Default::default(),
                            unpacked_sfixed32: ::core::default::Default::default(),
                            unpacked_sfixed64: ::core::default::Default::default(),
                            unpacked_float: ::core::default::Default::default(),
                            unpacked_double: ::core::default::Default::default(),
                            unpacked_bool: ::core::default::Default::default(),
                            unpacked_nested_enum: ::core::default::Default::default(),
                            map_int32_int32: ::core::default::Default::default(),
                            map_int64_int64: ::core::default::Default::default(),
                            map_uint32_uint32: ::core::default::Default::default(),
                            map_uint64_uint64: ::core::default::Default::default(),
                            map_sint32_sint32: ::core::default::Default::default(),
                            map_sint64_sint64: ::core::default::Default::default(),
                            map_fixed32_fixed32: ::core::default::Default::default(),
                            map_fixed64_fixed64: ::core::default::Default::default(),
                            map_sfixed32_sfixed32: ::core::default::Default::default(),
                            map_sfixed64_sfixed64: ::core::default::Default::default(),
                            map_int32_float: ::core::default::Default::default(),
                            map_int32_double: ::core::default::Default::default(),
                            map_bool_bool: ::core::default::Default::default(),
                            map_string_string: ::core::default::Default::default(),
                            map_string_bytes: ::core::default::Default::default(),
                            map_string_nested_message: ::core::default::Default::default(),
                            map_string_foreign_message: ::core::default::Default::default(),
                            map_string_nested_enum: ::core::default::Default::default(),
                            map_string_foreign_enum: ::core::default::Default::default(),
                            Data: ::core::default::Default::default(),
                            default_int32: ::core::default::Default::default(),
                            default_int64: ::core::default::Default::default(),
                            default_uint32: ::core::default::Default::default(),
                            default_uint64: ::core::default::Default::default(),
                            default_sint32: ::core::default::Default::default(),
                            default_sint64: ::core::default::Default::default(),
                            default_fixed32: ::core::default::Default::default(),
                            default_fixed64: ::core::default::Default::default(),
                            default_sfixed32: ::core::default::Default::default(),
                            default_sfixed64: ::core::default::Default::default(),
                            default_float: ::core::default::Default::default(),
                            default_double: ::core::default::Default::default(),
                            default_bool: ::core::default::Default::default(),
                            default_string: ::core::default::Default::default(),
                            default_bytes: ::core::default::Default::default(),
                            fieldname1: ::core::default::Default::default(),
                            field_name2: ::core::default::Default::default(),
                            _field_name3: ::core::default::Default::default(),
                            field__name4_: ::core::default::Default::default(),
                            field0name5: ::core::default::Default::default(),
                            field_0_name6: ::core::default::Default::default(),
                            fieldName7: ::core::default::Default::default(),
                            FieldName8: ::core::default::Default::default(),
                            field_Name9: ::core::default::Default::default(),
                            Field_Name10: ::core::default::Default::default(),
                            FIELD_NAME11: ::core::default::Default::default(),
                            FIELD_name12: ::core::default::Default::default(),
                            __field_name13: ::core::default::Default::default(),
                            __Field_name14: ::core::default::Default::default(),
                            field__name15: ::core::default::Default::default(),
                            field__Name16: ::core::default::Default::default(),
                            field_name17__: ::core::default::Default::default(),
                            Field_name18__: ::core::default::Default::default(),
                            oneof_field: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2 {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2 {
                        TestAllTypesProto2 {
                            optional_int32: ::core::clone::Clone::clone(&self.optional_int32),
                            optional_int64: ::core::clone::Clone::clone(&self.optional_int64),
                            optional_uint32: ::core::clone::Clone::clone(&self.optional_uint32),
                            optional_uint64: ::core::clone::Clone::clone(&self.optional_uint64),
                            optional_sint32: ::core::clone::Clone::clone(&self.optional_sint32),
                            optional_sint64: ::core::clone::Clone::clone(&self.optional_sint64),
                            optional_fixed32: ::core::clone::Clone::clone(&self.optional_fixed32),
                            optional_fixed64: ::core::clone::Clone::clone(&self.optional_fixed64),
                            optional_sfixed32: ::core::clone::Clone::clone(&self.optional_sfixed32),
                            optional_sfixed64: ::core::clone::Clone::clone(&self.optional_sfixed64),
                            optional_float: ::core::clone::Clone::clone(&self.optional_float),
                            optional_double: ::core::clone::Clone::clone(&self.optional_double),
                            optional_bool: ::core::clone::Clone::clone(&self.optional_bool),
                            optional_string: ::core::clone::Clone::clone(&self.optional_string),
                            optional_bytes: ::core::clone::Clone::clone(&self.optional_bytes),
                            optional_nested_message: ::core::clone::Clone::clone(
                                &self.optional_nested_message,
                            ),
                            optional_foreign_message: ::core::clone::Clone::clone(
                                &self.optional_foreign_message,
                            ),
                            optional_nested_enum: ::core::clone::Clone::clone(
                                &self.optional_nested_enum,
                            ),
                            optional_foreign_enum: ::core::clone::Clone::clone(
                                &self.optional_foreign_enum,
                            ),
                            optional_string_piece: ::core::clone::Clone::clone(
                                &self.optional_string_piece,
                            ),
                            optional_cord: ::core::clone::Clone::clone(&self.optional_cord),
                            recursive_message: ::core::clone::Clone::clone(&self.recursive_message),
                            repeated_int32: ::core::clone::Clone::clone(&self.repeated_int32),
                            repeated_int64: ::core::clone::Clone::clone(&self.repeated_int64),
                            repeated_uint32: ::core::clone::Clone::clone(&self.repeated_uint32),
                            repeated_uint64: ::core::clone::Clone::clone(&self.repeated_uint64),
                            repeated_sint32: ::core::clone::Clone::clone(&self.repeated_sint32),
                            repeated_sint64: ::core::clone::Clone::clone(&self.repeated_sint64),
                            repeated_fixed32: ::core::clone::Clone::clone(&self.repeated_fixed32),
                            repeated_fixed64: ::core::clone::Clone::clone(&self.repeated_fixed64),
                            repeated_sfixed32: ::core::clone::Clone::clone(&self.repeated_sfixed32),
                            repeated_sfixed64: ::core::clone::Clone::clone(&self.repeated_sfixed64),
                            repeated_float: ::core::clone::Clone::clone(&self.repeated_float),
                            repeated_double: ::core::clone::Clone::clone(&self.repeated_double),
                            repeated_bool: ::core::clone::Clone::clone(&self.repeated_bool),
                            repeated_string: ::core::clone::Clone::clone(&self.repeated_string),
                            repeated_bytes: ::core::clone::Clone::clone(&self.repeated_bytes),
                            repeated_nested_message: ::core::clone::Clone::clone(
                                &self.repeated_nested_message,
                            ),
                            repeated_foreign_message: ::core::clone::Clone::clone(
                                &self.repeated_foreign_message,
                            ),
                            repeated_nested_enum: ::core::clone::Clone::clone(
                                &self.repeated_nested_enum,
                            ),
                            repeated_foreign_enum: ::core::clone::Clone::clone(
                                &self.repeated_foreign_enum,
                            ),
                            repeated_string_piece: ::core::clone::Clone::clone(
                                &self.repeated_string_piece,
                            ),
                            repeated_cord: ::core::clone::Clone::clone(&self.repeated_cord),
                            packed_int32: ::core::clone::Clone::clone(&self.packed_int32),
                            packed_int64: ::core::clone::Clone::clone(&self.packed_int64),
                            packed_uint32: ::core::clone::Clone::clone(&self.packed_uint32),
                            packed_uint64: ::core::clone::Clone::clone(&self.packed_uint64),
                            packed_sint32: ::core::clone::Clone::clone(&self.packed_sint32),
                            packed_sint64: ::core::clone::Clone::clone(&self.packed_sint64),
                            packed_fixed32: ::core::clone::Clone::clone(&self.packed_fixed32),
                            packed_fixed64: ::core::clone::Clone::clone(&self.packed_fixed64),
                            packed_sfixed32: ::core::clone::Clone::clone(&self.packed_sfixed32),
                            packed_sfixed64: ::core::clone::Clone::clone(&self.packed_sfixed64),
                            packed_float: ::core::clone::Clone::clone(&self.packed_float),
                            packed_double: ::core::clone::Clone::clone(&self.packed_double),
                            packed_bool: ::core::clone::Clone::clone(&self.packed_bool),
                            packed_nested_enum: ::core::clone::Clone::clone(
                                &self.packed_nested_enum,
                            ),
                            unpacked_int32: ::core::clone::Clone::clone(&self.unpacked_int32),
                            unpacked_int64: ::core::clone::Clone::clone(&self.unpacked_int64),
                            unpacked_uint32: ::core::clone::Clone::clone(&self.unpacked_uint32),
                            unpacked_uint64: ::core::clone::Clone::clone(&self.unpacked_uint64),
                            unpacked_sint32: ::core::clone::Clone::clone(&self.unpacked_sint32),
                            unpacked_sint64: ::core::clone::Clone::clone(&self.unpacked_sint64),
                            unpacked_fixed32: ::core::clone::Clone::clone(&self.unpacked_fixed32),
                            unpacked_fixed64: ::core::clone::Clone::clone(&self.unpacked_fixed64),
                            unpacked_sfixed32: ::core::clone::Clone::clone(&self.unpacked_sfixed32),
                            unpacked_sfixed64: ::core::clone::Clone::clone(&self.unpacked_sfixed64),
                            unpacked_float: ::core::clone::Clone::clone(&self.unpacked_float),
                            unpacked_double: ::core::clone::Clone::clone(&self.unpacked_double),
                            unpacked_bool: ::core::clone::Clone::clone(&self.unpacked_bool),
                            unpacked_nested_enum: ::core::clone::Clone::clone(
                                &self.unpacked_nested_enum,
                            ),
                            map_int32_int32: ::core::clone::Clone::clone(&self.map_int32_int32),
                            map_int64_int64: ::core::clone::Clone::clone(&self.map_int64_int64),
                            map_uint32_uint32: ::core::clone::Clone::clone(&self.map_uint32_uint32),
                            map_uint64_uint64: ::core::clone::Clone::clone(&self.map_uint64_uint64),
                            map_sint32_sint32: ::core::clone::Clone::clone(&self.map_sint32_sint32),
                            map_sint64_sint64: ::core::clone::Clone::clone(&self.map_sint64_sint64),
                            map_fixed32_fixed32: ::core::clone::Clone::clone(
                                &self.map_fixed32_fixed32,
                            ),
                            map_fixed64_fixed64: ::core::clone::Clone::clone(
                                &self.map_fixed64_fixed64,
                            ),
                            map_sfixed32_sfixed32: ::core::clone::Clone::clone(
                                &self.map_sfixed32_sfixed32,
                            ),
                            map_sfixed64_sfixed64: ::core::clone::Clone::clone(
                                &self.map_sfixed64_sfixed64,
                            ),
                            map_int32_float: ::core::clone::Clone::clone(&self.map_int32_float),
                            map_int32_double: ::core::clone::Clone::clone(&self.map_int32_double),
                            map_bool_bool: ::core::clone::Clone::clone(&self.map_bool_bool),
                            map_string_string: ::core::clone::Clone::clone(&self.map_string_string),
                            map_string_bytes: ::core::clone::Clone::clone(&self.map_string_bytes),
                            map_string_nested_message: ::core::clone::Clone::clone(
                                &self.map_string_nested_message,
                            ),
                            map_string_foreign_message: ::core::clone::Clone::clone(
                                &self.map_string_foreign_message,
                            ),
                            map_string_nested_enum: ::core::clone::Clone::clone(
                                &self.map_string_nested_enum,
                            ),
                            map_string_foreign_enum: ::core::clone::Clone::clone(
                                &self.map_string_foreign_enum,
                            ),
                            Data: ::core::clone::Clone::clone(&self.Data),
                            default_int32: ::core::clone::Clone::clone(&self.default_int32),
                            default_int64: ::core::clone::Clone::clone(&self.default_int64),
                            default_uint32: ::core::clone::Clone::clone(&self.default_uint32),
                            default_uint64: ::core::clone::Clone::clone(&self.default_uint64),
                            default_sint32: ::core::clone::Clone::clone(&self.default_sint32),
                            default_sint64: ::core::clone::Clone::clone(&self.default_sint64),
                            default_fixed32: ::core::clone::Clone::clone(&self.default_fixed32),
                            default_fixed64: ::core::clone::Clone::clone(&self.default_fixed64),
                            default_sfixed32: ::core::clone::Clone::clone(&self.default_sfixed32),
                            default_sfixed64: ::core::clone::Clone::clone(&self.default_sfixed64),
                            default_float: ::core::clone::Clone::clone(&self.default_float),
                            default_double: ::core::clone::Clone::clone(&self.default_double),
                            default_bool: ::core::clone::Clone::clone(&self.default_bool),
                            default_string: ::core::clone::Clone::clone(&self.default_string),
                            default_bytes: ::core::clone::Clone::clone(&self.default_bytes),
                            fieldname1: ::core::clone::Clone::clone(&self.fieldname1),
                            field_name2: ::core::clone::Clone::clone(&self.field_name2),
                            _field_name3: ::core::clone::Clone::clone(&self._field_name3),
                            field__name4_: ::core::clone::Clone::clone(&self.field__name4_),
                            field0name5: ::core::clone::Clone::clone(&self.field0name5),
                            field_0_name6: ::core::clone::Clone::clone(&self.field_0_name6),
                            fieldName7: ::core::clone::Clone::clone(&self.fieldName7),
                            FieldName8: ::core::clone::Clone::clone(&self.FieldName8),
                            field_Name9: ::core::clone::Clone::clone(&self.field_Name9),
                            Field_Name10: ::core::clone::Clone::clone(&self.Field_Name10),
                            FIELD_NAME11: ::core::clone::Clone::clone(&self.FIELD_NAME11),
                            FIELD_name12: ::core::clone::Clone::clone(&self.FIELD_name12),
                            __field_name13: ::core::clone::Clone::clone(&self.__field_name13),
                            __Field_name14: ::core::clone::Clone::clone(&self.__Field_name14),
                            field__name15: ::core::clone::Clone::clone(&self.field__name15),
                            field__Name16: ::core::clone::Clone::clone(&self.field__Name16),
                            field_name17__: ::core::clone::Clone::clone(&self.field_name17__),
                            Field_name18__: ::core::clone::Clone::clone(&self.Field_name18__),
                            oneof_field: ::core::clone::Clone::clone(&self.oneof_field),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2 {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2) -> bool {
                        self.optional_int32 == other.optional_int32
                            & &self.optional_int64 == other.optional_int64
                            & &self.optional_uint32 == other.optional_uint32
                            & &self.optional_uint64 == other.optional_uint64
                            & &self.optional_sint32 == other.optional_sint32
                            & &self.optional_sint64 == other.optional_sint64
                            & &self.optional_fixed32 == other.optional_fixed32
                            & &self.optional_fixed64 == other.optional_fixed64
                            & &self.optional_sfixed32 == other.optional_sfixed32
                            && self.optional_sfixed64 == other.optional_sfixed64
                            & &self.optional_float == other.optional_float
                            & &self.optional_double == other.optional_double
                            & &self.optional_bool == other.optional_bool
                            & &self.optional_string == other.optional_string
                            & &self.optional_bytes == other.optional_bytes
                            & &self.optional_nested_message == other.optional_nested_message
                            & &self.optional_foreign_message == other.optional_foreign_message
                            & &self.optional_nested_enum == other.optional_nested_enum
                            & &self.optional_foreign_enum == other.optional_foreign_enum
                            && self.optional_string_piece == other.optional_string_piece
                            & &self.optional_cord == other.optional_cord
                            & &self.recursive_message == other.recursive_message
                            & &self.repeated_int32 == other.repeated_int32
                            & &self.repeated_int64 == other.repeated_int64
                            & &self.repeated_uint32 == other.repeated_uint32
                            & &self.repeated_uint64 == other.repeated_uint64
                            & &self.repeated_sint32 == other.repeated_sint32
                            & &self.repeated_sint64 == other.repeated_sint64
                            & &self.repeated_fixed32 == other.repeated_fixed32
                            && self.repeated_fixed64 == other.repeated_fixed64
                            & &self.repeated_sfixed32 == other.repeated_sfixed32
                            & &self.repeated_sfixed64 == other.repeated_sfixed64
                            & &self.repeated_float == other.repeated_float
                            & &self.repeated_double == other.repeated_double
                            & &self.repeated_bool == other.repeated_bool
                            & &self.repeated_string == other.repeated_string
                            & &self.repeated_bytes == other.repeated_bytes
                            & &self.repeated_nested_message == other.repeated_nested_message
                            & &self.repeated_foreign_message == other.repeated_foreign_message
                            && self.repeated_nested_enum == other.repeated_nested_enum
                            & &self.repeated_foreign_enum == other.repeated_foreign_enum
                            & &self.repeated_string_piece == other.repeated_string_piece
                            & &self.repeated_cord == other.repeated_cord
                            & &self.packed_int32 == other.packed_int32
                            & &self.packed_int64 == other.packed_int64
                            & &self.packed_uint32 == other.packed_uint32
                            & &self.packed_uint64 == other.packed_uint64
                            & &self.packed_sint32 == other.packed_sint32
                            & &self.packed_sint64 == other.packed_sint64
                            && self.packed_fixed32 == other.packed_fixed32
                            & &self.packed_fixed64 == other.packed_fixed64
                            & &self.packed_sfixed32 == other.packed_sfixed32
                            & &self.packed_sfixed64 == other.packed_sfixed64
                            & &self.packed_float == other.packed_float
                            & &self.packed_double == other.packed_double
                            & &self.packed_bool == other.packed_bool
                            & &self.packed_nested_enum == other.packed_nested_enum
                            & &self.unpacked_int32 == other.unpacked_int32
                            & &self.unpacked_int64 == other.unpacked_int64
                            && self.unpacked_uint32 == other.unpacked_uint32
                            & &self.unpacked_uint64 == other.unpacked_uint64
                            & &self.unpacked_sint32 == other.unpacked_sint32
                            & &self.unpacked_sint64 == other.unpacked_sint64
                            & &self.unpacked_fixed32 == other.unpacked_fixed32
                            & &self.unpacked_fixed64 == other.unpacked_fixed64
                            & &self.unpacked_sfixed32 == other.unpacked_sfixed32
                            & &self.unpacked_sfixed64 == other.unpacked_sfixed64
                            & &self.unpacked_float == other.unpacked_float
                            & &self.unpacked_double == other.unpacked_double
                            && self.unpacked_bool == other.unpacked_bool
                            & &self.unpacked_nested_enum == other.unpacked_nested_enum
                            & &self.map_int32_int32 == other.map_int32_int32
                            & &self.map_int64_int64 == other.map_int64_int64
                            & &self.map_uint32_uint32 == other.map_uint32_uint32
                            & &self.map_uint64_uint64 == other.map_uint64_uint64
                            & &self.map_sint32_sint32 == other.map_sint32_sint32
                            & &self.map_sint64_sint64 == other.map_sint64_sint64
                            & &self.map_fixed32_fixed32 == other.map_fixed32_fixed32
                            & &self.map_fixed64_fixed64 == other.map_fixed64_fixed64
                            && self.map_sfixed32_sfixed32 == other.map_sfixed32_sfixed32
                            & &self.map_sfixed64_sfixed64 == other.map_sfixed64_sfixed64
                            & &self.map_int32_float == other.map_int32_float
                            & &self.map_int32_double == other.map_int32_double
                            & &self.map_bool_bool == other.map_bool_bool
                            & &self.map_string_string == other.map_string_string
                            & &self.map_string_bytes == other.map_string_bytes
                            & &self.map_string_nested_message == other.map_string_nested_message
                            & &self.map_string_foreign_message == other.map_string_foreign_message
                            & &self.map_string_nested_enum == other.map_string_nested_enum
                            && self.map_string_foreign_enum == other.map_string_foreign_enum
                            & &self.Data == other.Data
                            & &self.default_int32 == other.default_int32
                            & &self.default_int64 == other.default_int64
                            & &self.default_uint32 == other.default_uint32
                            & &self.default_uint64 == other.default_uint64
                            & &self.default_sint32 == other.default_sint32
                            & &self.default_sint64 == other.default_sint64
                            & &self.default_fixed32 == other.default_fixed32
                            & &self.default_fixed64 == other.default_fixed64
                            && self.default_sfixed32 == other.default_sfixed32
                            & &self.default_sfixed64 == other.default_sfixed64
                            & &self.default_float == other.default_float
                            & &self.default_double == other.default_double
                            & &self.default_bool == other.default_bool
                            & &self.default_string == other.default_string
                            & &self.default_bytes == other.default_bytes
                            & &self.fieldname1 == other.fieldname1
                            & &self.field_name2 == other.field_name2
                            & &self._field_name3 == other._field_name3
                            && self.field__name4_ == other.field__name4_
                            & &self.field0name5 == other.field0name5
                            & &self.field_0_name6 == other.field_0_name6
                            & &self.fieldName7 == other.fieldName7
                            & &self.FieldName8 == other.FieldName8
                            & &self.field_Name9 == other.field_Name9
                            & &self.Field_Name10 == other.Field_Name10
                            & &self.FIELD_NAME11 == other.FIELD_NAME11
                            & &self.FIELD_name12 == other.FIELD_name12
                            & &self.__field_name13 == other.__field_name13
                            && self.__Field_name14 == other.__Field_name14
                            & &self.field__name15 == other.field__name15
                            & &self.field__Name16 == other.field__Name16
                            & &self.field_name17__ == other.field_name17__
                            & &self.Field_name18__ == other.Field_name18__
                            & &self.oneof_field == other.oneof_field
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_optional(
                                &mut self.optional_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            16u32 => binformat::merge_optional(
                                &mut self.optional_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            24u32 => binformat::merge_optional(
                                &mut self.optional_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            32u32 => binformat::merge_optional(
                                &mut self.optional_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            40u32 => binformat::merge_optional(
                                &mut self.optional_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            48u32 => binformat::merge_optional(
                                &mut self.optional_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            61u32 => binformat::merge_optional(
                                &mut self.optional_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            65u32 => binformat::merge_optional(
                                &mut self.optional_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            77u32 => binformat::merge_optional(
                                &mut self.optional_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            81u32 => binformat::merge_optional(
                                &mut self.optional_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            93u32 => binformat::merge_optional(
                                &mut self.optional_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            97u32 => binformat::merge_optional(
                                &mut self.optional_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            104u32 => binformat::merge_optional(
                                &mut self.optional_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            114u32 => binformat::merge_optional(
                                &mut self.optional_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            122u32 => binformat::merge_optional(
                                &mut self.optional_bytes,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            146u32 => binformat::merge_optional(
                                &mut self.optional_nested_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            154u32 => binformat::merge_optional(
                                &mut self.optional_foreign_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            168u32 => binformat::merge_optional(
                                &mut self.optional_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            176u32 => binformat::merge_optional(
                                &mut self.optional_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            194u32 => binformat::merge_optional(
                                &mut self.optional_string_piece,
                                stream,
                                binformat::InputStream::string,
                            ),
                            202u32 => binformat::merge_optional(
                                &mut self.optional_cord,
                                stream,
                                binformat::InputStream::string,
                            ),
                            218u32 => binformat::merge_optional(
                                &mut self.recursive_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            248u32 => binformat::merge_repeated(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            250u32 => binformat::merge_packed(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            256u32 => binformat::merge_repeated(
                                &mut self.repeated_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            258u32 => binformat::merge_packed(
                                &mut self.repeated_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            264u32 => binformat::merge_repeated(
                                &mut self.repeated_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            266u32 => binformat::merge_packed(
                                &mut self.repeated_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            272u32 => binformat::merge_repeated(
                                &mut self.repeated_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            274u32 => binformat::merge_packed(
                                &mut self.repeated_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            280u32 => binformat::merge_repeated(
                                &mut self.repeated_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            282u32 => binformat::merge_packed(
                                &mut self.repeated_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            288u32 => binformat::merge_repeated(
                                &mut self.repeated_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            290u32 => binformat::merge_packed(
                                &mut self.repeated_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            301u32 => binformat::merge_repeated(
                                &mut self.repeated_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            298u32 => binformat::merge_packed(
                                &mut self.repeated_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            305u32 => binformat::merge_repeated(
                                &mut self.repeated_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            306u32 => binformat::merge_packed(
                                &mut self.repeated_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            317u32 => binformat::merge_repeated(
                                &mut self.repeated_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            314u32 => binformat::merge_packed(
                                &mut self.repeated_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            321u32 => binformat::merge_repeated(
                                &mut self.repeated_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            322u32 => binformat::merge_packed(
                                &mut self.repeated_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            333u32 => binformat::merge_repeated(
                                &mut self.repeated_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            330u32 => binformat::merge_packed(
                                &mut self.repeated_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            337u32 => binformat::merge_repeated(
                                &mut self.repeated_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            338u32 => binformat::merge_packed(
                                &mut self.repeated_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            344u32 => binformat::merge_repeated(
                                &mut self.repeated_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            346u32 => binformat::merge_packed(
                                &mut self.repeated_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            354u32 => binformat::merge_repeated(
                                &mut self.repeated_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            362u32 => binformat::merge_repeated(
                                &mut self.repeated_bytes,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            386u32 => binformat::merge_repeated(
                                &mut self.repeated_nested_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            394u32 => binformat::merge_repeated(
                                &mut self.repeated_foreign_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            408u32 => binformat::merge_repeated(
                                &mut self.repeated_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            410u32 => binformat::merge_packed(
                                &mut self.repeated_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            416u32 => binformat::merge_repeated(
                                &mut self.repeated_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            418u32 => binformat::merge_packed(
                                &mut self.repeated_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            434u32 => binformat::merge_repeated(
                                &mut self.repeated_string_piece,
                                stream,
                                binformat::InputStream::string,
                            ),
                            442u32 => binformat::merge_repeated(
                                &mut self.repeated_cord,
                                stream,
                                binformat::InputStream::string,
                            ),
                            600u32 => binformat::merge_repeated(
                                &mut self.packed_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            602u32 => binformat::merge_packed(
                                &mut self.packed_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            608u32 => binformat::merge_repeated(
                                &mut self.packed_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            610u32 => binformat::merge_packed(
                                &mut self.packed_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            616u32 => binformat::merge_repeated(
                                &mut self.packed_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            618u32 => binformat::merge_packed(
                                &mut self.packed_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            624u32 => binformat::merge_repeated(
                                &mut self.packed_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            626u32 => binformat::merge_packed(
                                &mut self.packed_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            632u32 => binformat::merge_repeated(
                                &mut self.packed_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            634u32 => binformat::merge_packed(
                                &mut self.packed_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            640u32 => binformat::merge_repeated(
                                &mut self.packed_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            642u32 => binformat::merge_packed(
                                &mut self.packed_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            653u32 => binformat::merge_repeated(
                                &mut self.packed_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            650u32 => binformat::merge_packed(
                                &mut self.packed_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            657u32 => binformat::merge_repeated(
                                &mut self.packed_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            658u32 => binformat::merge_packed(
                                &mut self.packed_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            669u32 => binformat::merge_repeated(
                                &mut self.packed_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            666u32 => binformat::merge_packed(
                                &mut self.packed_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            673u32 => binformat::merge_repeated(
                                &mut self.packed_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            674u32 => binformat::merge_packed(
                                &mut self.packed_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            685u32 => binformat::merge_repeated(
                                &mut self.packed_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            682u32 => binformat::merge_packed(
                                &mut self.packed_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            689u32 => binformat::merge_repeated(
                                &mut self.packed_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            690u32 => binformat::merge_packed(
                                &mut self.packed_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            696u32 => binformat::merge_repeated(
                                &mut self.packed_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            698u32 => binformat::merge_packed(
                                &mut self.packed_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            704u32 => binformat::merge_repeated(
                                &mut self.packed_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            706u32 => binformat::merge_packed(
                                &mut self.packed_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            712u32 => binformat::merge_repeated(
                                &mut self.unpacked_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            714u32 => binformat::merge_packed(
                                &mut self.unpacked_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            720u32 => binformat::merge_repeated(
                                &mut self.unpacked_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            722u32 => binformat::merge_packed(
                                &mut self.unpacked_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            728u32 => binformat::merge_repeated(
                                &mut self.unpacked_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            730u32 => binformat::merge_packed(
                                &mut self.unpacked_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            736u32 => binformat::merge_repeated(
                                &mut self.unpacked_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            738u32 => binformat::merge_packed(
                                &mut self.unpacked_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            744u32 => binformat::merge_repeated(
                                &mut self.unpacked_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            746u32 => binformat::merge_packed(
                                &mut self.unpacked_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            752u32 => binformat::merge_repeated(
                                &mut self.unpacked_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            754u32 => binformat::merge_packed(
                                &mut self.unpacked_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            765u32 => binformat::merge_repeated(
                                &mut self.unpacked_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            762u32 => binformat::merge_packed(
                                &mut self.unpacked_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            769u32 => binformat::merge_repeated(
                                &mut self.unpacked_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            770u32 => binformat::merge_packed(
                                &mut self.unpacked_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            781u32 => binformat::merge_repeated(
                                &mut self.unpacked_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            778u32 => binformat::merge_packed(
                                &mut self.unpacked_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            785u32 => binformat::merge_repeated(
                                &mut self.unpacked_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            786u32 => binformat::merge_packed(
                                &mut self.unpacked_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            797u32 => binformat::merge_repeated(
                                &mut self.unpacked_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            794u32 => binformat::merge_packed(
                                &mut self.unpacked_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            801u32 => binformat::merge_repeated(
                                &mut self.unpacked_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            802u32 => binformat::merge_packed(
                                &mut self.unpacked_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            808u32 => binformat::merge_repeated(
                                &mut self.unpacked_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            810u32 => binformat::merge_packed(
                                &mut self.unpacked_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            816u32 => binformat::merge_repeated(
                                &mut self.unpacked_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            818u32 => binformat::merge_packed(
                                &mut self.unpacked_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            450u32 => binformat::merge_map(
                                &mut self.map_int32_int32,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            458u32 => binformat::merge_map(
                                &mut self.map_int64_int64,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            466u32 => binformat::merge_map(
                                &mut self.map_uint32_uint32,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            474u32 => binformat::merge_map(
                                &mut self.map_uint64_uint64,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            482u32 => binformat::merge_map(
                                &mut self.map_sint32_sint32,
                                stream,
                                binformat::InputStream::sigint,
                                binformat::InputStream::sigint,
                            ),
                            490u32 => binformat::merge_map(
                                &mut self.map_sint64_sint64,
                                stream,
                                binformat::InputStream::sigint,
                                binformat::InputStream::sigint,
                            ),
                            498u32 => binformat::merge_map(
                                &mut self.map_fixed32_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                                binformat::InputStream::fixed32,
                            ),
                            506u32 => binformat::merge_map(
                                &mut self.map_fixed64_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                                binformat::InputStream::fixed64,
                            ),
                            514u32 => binformat::merge_map(
                                &mut self.map_sfixed32_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                                binformat::InputStream::fixed32,
                            ),
                            522u32 => binformat::merge_map(
                                &mut self.map_sfixed64_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                                binformat::InputStream::fixed64,
                            ),
                            530u32 => binformat::merge_map(
                                &mut self.map_int32_float,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::fixed32,
                            ),
                            538u32 => binformat::merge_map(
                                &mut self.map_int32_double,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::fixed64,
                            ),
                            546u32 => binformat::merge_map(
                                &mut self.map_bool_bool,
                                stream,
                                binformat::InputStream::bool,
                                binformat::InputStream::bool,
                            ),
                            554u32 => binformat::merge_map(
                                &mut self.map_string_string,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::string,
                            ),
                            562u32 => binformat::merge_map(
                                &mut self.map_string_bytes,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::bytes,
                            ),
                            570u32 => binformat::merge_map(
                                &mut self.map_string_nested_message,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::nested,
                            ),
                            578u32 => binformat::merge_map(
                                &mut self.map_string_foreign_message,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::nested,
                            ),
                            586u32 => binformat::merge_map(
                                &mut self.map_string_nested_enum,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::protoenum,
                            ),
                            594u32 => binformat::merge_map(
                                &mut self.map_string_foreign_enum,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::protoenum,
                            ),
                            1610u32 => binformat::merge_optional(
                                &mut self.Data,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1928u32 => binformat::merge_optional(
                                &mut self.default_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            1936u32 => binformat::merge_optional(
                                &mut self.default_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            1944u32 => binformat::merge_optional(
                                &mut self.default_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            1952u32 => binformat::merge_optional(
                                &mut self.default_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            1960u32 => binformat::merge_optional(
                                &mut self.default_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            1968u32 => binformat::merge_optional(
                                &mut self.default_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            1981u32 => binformat::merge_optional(
                                &mut self.default_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            1985u32 => binformat::merge_optional(
                                &mut self.default_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            1997u32 => binformat::merge_optional(
                                &mut self.default_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            2001u32 => binformat::merge_optional(
                                &mut self.default_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            2013u32 => binformat::merge_optional(
                                &mut self.default_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            2017u32 => binformat::merge_optional(
                                &mut self.default_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            2024u32 => binformat::merge_optional(
                                &mut self.default_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            2034u32 => binformat::merge_optional(
                                &mut self.default_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            2042u32 => binformat::merge_optional(
                                &mut self.default_bytes,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            3208u32 => binformat::merge_optional(
                                &mut self.fieldname1,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3216u32 => binformat::merge_optional(
                                &mut self.field_name2,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3224u32 => binformat::merge_optional(
                                &mut self._field_name3,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3232u32 => binformat::merge_optional(
                                &mut self.field__name4_,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3240u32 => binformat::merge_optional(
                                &mut self.field0name5,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3248u32 => binformat::merge_optional(
                                &mut self.field_0_name6,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3256u32 => binformat::merge_optional(
                                &mut self.fieldName7,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3264u32 => binformat::merge_optional(
                                &mut self.FieldName8,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3272u32 => binformat::merge_optional(
                                &mut self.field_Name9,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3280u32 => binformat::merge_optional(
                                &mut self.Field_Name10,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3288u32 => binformat::merge_optional(
                                &mut self.FIELD_NAME11,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3296u32 => binformat::merge_optional(
                                &mut self.FIELD_name12,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3304u32 => binformat::merge_optional(
                                &mut self.__field_name13,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3312u32 => binformat::merge_optional(
                                &mut self.__Field_name14,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3320u32 => binformat::merge_optional(
                                &mut self.field__name15,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3328u32 => binformat::merge_optional(
                                &mut self.field__Name16,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3336u32 => binformat::merge_optional(
                                &mut self.field_name17__,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3344u32 => binformat::merge_optional(
                                &mut self.Field_name18__,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            888u32 | 889u32 | 890u32 | 891u32 | 892u32 | 893u32 | 894u32
                            | 895u32 | 896u32 | 897u32 | 898u32 | 899u32 | 900u32 | 901u32
                            | 902u32 | 903u32 | 904u32 | 905u32 | 906u32 | 907u32 | 908u32
                            | 909u32 | 910u32 | 911u32 | 912u32 | 913u32 | 914u32 | 915u32
                            | 916u32 | 917u32 | 918u32 | 919u32 | 920u32 | 921u32 | 922u32
                            | 923u32 | 924u32 | 925u32 | 926u32 | 927u32 | 928u32 | 929u32
                            | 930u32 | 931u32 | 932u32 | 933u32 | 934u32 | 935u32 | 936u32
                            | 937u32 | 938u32 | 939u32 | 940u32 | 941u32 | 942u32 | 943u32
                            | 944u32 | 945u32 | 946u32 | 947u32 | 948u32 | 949u32 | 950u32
                            | 951u32 | 952u32 | 953u32 | 954u32 | 955u32 | 956u32 | 957u32
                            | 958u32 | 959u32 => {
                                binformat::merge_oneof(&mut self.oneof_field, tag, stream)
                            }
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.optional_int32,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.optional_int64,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.optional_uint32,
                            24u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.optional_uint64,
                            32u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.optional_sint32,
                            40u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_optional(
                            &self.optional_sint64,
                            48u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_optional(
                            &self.optional_fixed32,
                            61u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.optional_fixed64,
                            65u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.optional_sfixed32,
                            77u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.optional_sfixed64,
                            81u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.optional_float,
                            93u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.optional_double,
                            97u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.optional_bool,
                            104u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_optional(
                            &self.optional_string,
                            114u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.optional_bytes,
                            122u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_optional(
                            &self.optional_nested_message,
                            146u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_foreign_message,
                            154u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_nested_enum,
                            168u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_optional(
                            &self.optional_foreign_enum,
                            176u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_optional(
                            &self.optional_string_piece,
                            194u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.optional_cord,
                            202u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.recursive_message,
                            218u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_int32,
                            248u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_int64,
                            256u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_uint32,
                            264u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_uint64,
                            272u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_sint32,
                            280u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_sint64,
                            288u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.repeated_fixed32,
                            301u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.repeated_fixed64,
                            305u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.repeated_sfixed32,
                            317u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.repeated_sfixed64,
                            321u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.repeated_float,
                            333u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.repeated_double,
                            337u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.repeated_bool,
                            344u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_repeated(
                            &self.repeated_string,
                            354u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.repeated_bytes,
                            362u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_repeated(
                            &self.repeated_nested_message,
                            386u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_foreign_message,
                            394u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_nested_enum,
                            408u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_repeated(
                            &self.repeated_foreign_enum,
                            416u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_repeated(
                            &self.repeated_string_piece,
                            434u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.repeated_cord,
                            442u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.packed_int32,
                            600u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.packed_int64,
                            608u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.packed_uint32,
                            616u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.packed_uint64,
                            624u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.packed_sint32,
                            632u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.packed_sint64,
                            640u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.packed_fixed32,
                            653u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.packed_fixed64,
                            657u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.packed_sfixed32,
                            669u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.packed_sfixed64,
                            673u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.packed_float,
                            685u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.packed_double,
                            689u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.packed_bool,
                            696u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_repeated(
                            &self.packed_nested_enum,
                            704u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_int32,
                            712u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_int64,
                            720u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_uint32,
                            728u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_uint64,
                            736u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_sint32,
                            744u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_sint64,
                            752u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_fixed32,
                            765u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_fixed64,
                            769u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_sfixed32,
                            781u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_sfixed64,
                            785u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_float,
                            797u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_double,
                            801u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_bool,
                            808u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_repeated(
                            &self.unpacked_nested_enum,
                            816u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_map(
                            &self.map_int32_int32,
                            450u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_int64_int64,
                            458u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_uint32_uint32,
                            466u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_uint64_uint64,
                            474u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_sint32_sint32,
                            482u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::sigint,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_map(
                            &self.map_sint64_sint64,
                            490u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::sigint,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_map(
                            &self.map_fixed32_fixed32,
                            498u32,
                            13u32,
                            21u32,
                            stream,
                            binformat::OutputStream::fixed32,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_fixed64_fixed64,
                            506u32,
                            9u32,
                            17u32,
                            stream,
                            binformat::OutputStream::fixed64,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_sfixed32_sfixed32,
                            514u32,
                            13u32,
                            21u32,
                            stream,
                            binformat::OutputStream::fixed32,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_sfixed64_sfixed64,
                            522u32,
                            9u32,
                            17u32,
                            stream,
                            binformat::OutputStream::fixed64,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_int32_float,
                            530u32,
                            8u32,
                            21u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_int32_double,
                            538u32,
                            8u32,
                            17u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_bool_bool,
                            546u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::bool,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_map(
                            &self.map_string_string,
                            554u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_map(
                            &self.map_string_bytes,
                            562u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_map(
                            &self.map_string_nested_message,
                            570u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_map(
                            &self.map_string_foreign_message,
                            578u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_map(
                            &self.map_string_nested_enum,
                            586u32,
                            10u32,
                            16u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_map(
                            &self.map_string_foreign_enum,
                            594u32,
                            10u32,
                            16u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_optional(
                            &self.Data,
                            1610u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.default_int32,
                            1928u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.default_int64,
                            1936u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.default_uint32,
                            1944u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.default_uint64,
                            1952u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.default_sint32,
                            1960u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_optional(
                            &self.default_sint64,
                            1968u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_optional(
                            &self.default_fixed32,
                            1981u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.default_fixed64,
                            1985u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.default_sfixed32,
                            1997u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.default_sfixed64,
                            2001u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.default_float,
                            2013u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_optional(
                            &self.default_double,
                            2017u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_optional(
                            &self.default_bool,
                            2024u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_optional(
                            &self.default_string,
                            2034u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.default_bytes,
                            2042u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_optional(
                            &self.fieldname1,
                            3208u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field_name2,
                            3216u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self._field_name3,
                            3224u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field__name4_,
                            3232u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field0name5,
                            3240u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field_0_name6,
                            3248u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.fieldName7,
                            3256u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.FieldName8,
                            3264u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field_Name9,
                            3272u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.Field_Name10,
                            3280u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.FIELD_NAME11,
                            3288u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.FIELD_name12,
                            3296u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.__field_name13,
                            3304u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.__Field_name14,
                            3312u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field__name15,
                            3320u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field__Name16,
                            3328u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.field_name17__,
                            3336u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.Field_name18__,
                            3344u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_oneof(&self.oneof_field, stream);
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "optional_int32" => self.optional_int32.merge_text(stream),
                            "optional_int64" => self.optional_int64.merge_text(stream),
                            "optional_uint32" => self.optional_uint32.merge_text(stream),
                            "optional_uint64" => self.optional_uint64.merge_text(stream),
                            "optional_sint32" => self.optional_sint32.merge_text(stream),
                            "optional_sint64" => self.optional_sint64.merge_text(stream),
                            "optional_fixed32" => self.optional_fixed32.merge_text(stream),
                            "optional_fixed64" => self.optional_fixed64.merge_text(stream),
                            "optional_sfixed32" => self.optional_sfixed32.merge_text(stream),
                            "optional_sfixed64" => self.optional_sfixed64.merge_text(stream),
                            "optional_float" => self.optional_float.merge_text(stream),
                            "optional_double" => self.optional_double.merge_text(stream),
                            "optional_bool" => self.optional_bool.merge_text(stream),
                            "optional_string" => self.optional_string.merge_text(stream),
                            "optional_bytes" => self.optional_bytes.merge_text(stream),
                            "optional_nested_message" => {
                                self.optional_nested_message.merge_text(stream)
                            }
                            "optional_foreign_message" => {
                                self.optional_foreign_message.merge_text(stream)
                            }
                            "optional_nested_enum" => self.optional_nested_enum.merge_text(stream),
                            "optional_foreign_enum" => {
                                self.optional_foreign_enum.merge_text(stream)
                            }
                            "optional_string_piece" => {
                                self.optional_string_piece.merge_text(stream)
                            }
                            "optional_cord" => self.optional_cord.merge_text(stream),
                            "recursive_message" => self.recursive_message.merge_text(stream),
                            "repeated_int32" => self.repeated_int32.merge_text(stream),
                            "repeated_int64" => self.repeated_int64.merge_text(stream),
                            "repeated_uint32" => self.repeated_uint32.merge_text(stream),
                            "repeated_uint64" => self.repeated_uint64.merge_text(stream),
                            "repeated_sint32" => self.repeated_sint32.merge_text(stream),
                            "repeated_sint64" => self.repeated_sint64.merge_text(stream),
                            "repeated_fixed32" => self.repeated_fixed32.merge_text(stream),
                            "repeated_fixed64" => self.repeated_fixed64.merge_text(stream),
                            "repeated_sfixed32" => self.repeated_sfixed32.merge_text(stream),
                            "repeated_sfixed64" => self.repeated_sfixed64.merge_text(stream),
                            "repeated_float" => self.repeated_float.merge_text(stream),
                            "repeated_double" => self.repeated_double.merge_text(stream),
                            "repeated_bool" => self.repeated_bool.merge_text(stream),
                            "repeated_string" => self.repeated_string.merge_text(stream),
                            "repeated_bytes" => self.repeated_bytes.merge_text(stream),
                            "repeated_nested_message" => {
                                self.repeated_nested_message.merge_text(stream)
                            }
                            "repeated_foreign_message" => {
                                self.repeated_foreign_message.merge_text(stream)
                            }
                            "repeated_nested_enum" => self.repeated_nested_enum.merge_text(stream),
                            "repeated_foreign_enum" => {
                                self.repeated_foreign_enum.merge_text(stream)
                            }
                            "repeated_string_piece" => {
                                self.repeated_string_piece.merge_text(stream)
                            }
                            "repeated_cord" => self.repeated_cord.merge_text(stream),
                            "packed_int32" => self.packed_int32.merge_text(stream),
                            "packed_int64" => self.packed_int64.merge_text(stream),
                            "packed_uint32" => self.packed_uint32.merge_text(stream),
                            "packed_uint64" => self.packed_uint64.merge_text(stream),
                            "packed_sint32" => self.packed_sint32.merge_text(stream),
                            "packed_sint64" => self.packed_sint64.merge_text(stream),
                            "packed_fixed32" => self.packed_fixed32.merge_text(stream),
                            "packed_fixed64" => self.packed_fixed64.merge_text(stream),
                            "packed_sfixed32" => self.packed_sfixed32.merge_text(stream),
                            "packed_sfixed64" => self.packed_sfixed64.merge_text(stream),
                            "packed_float" => self.packed_float.merge_text(stream),
                            "packed_double" => self.packed_double.merge_text(stream),
                            "packed_bool" => self.packed_bool.merge_text(stream),
                            "packed_nested_enum" => self.packed_nested_enum.merge_text(stream),
                            "unpacked_int32" => self.unpacked_int32.merge_text(stream),
                            "unpacked_int64" => self.unpacked_int64.merge_text(stream),
                            "unpacked_uint32" => self.unpacked_uint32.merge_text(stream),
                            "unpacked_uint64" => self.unpacked_uint64.merge_text(stream),
                            "unpacked_sint32" => self.unpacked_sint32.merge_text(stream),
                            "unpacked_sint64" => self.unpacked_sint64.merge_text(stream),
                            "unpacked_fixed32" => self.unpacked_fixed32.merge_text(stream),
                            "unpacked_fixed64" => self.unpacked_fixed64.merge_text(stream),
                            "unpacked_sfixed32" => self.unpacked_sfixed32.merge_text(stream),
                            "unpacked_sfixed64" => self.unpacked_sfixed64.merge_text(stream),
                            "unpacked_float" => self.unpacked_float.merge_text(stream),
                            "unpacked_double" => self.unpacked_double.merge_text(stream),
                            "unpacked_bool" => self.unpacked_bool.merge_text(stream),
                            "unpacked_nested_enum" => self.unpacked_nested_enum.merge_text(stream),
                            "map_int32_int32" => self.map_int32_int32.merge_text(stream),
                            "map_int64_int64" => self.map_int64_int64.merge_text(stream),
                            "map_uint32_uint32" => self.map_uint32_uint32.merge_text(stream),
                            "map_uint64_uint64" => self.map_uint64_uint64.merge_text(stream),
                            "map_sint32_sint32" => self.map_sint32_sint32.merge_text(stream),
                            "map_sint64_sint64" => self.map_sint64_sint64.merge_text(stream),
                            "map_fixed32_fixed32" => self.map_fixed32_fixed32.merge_text(stream),
                            "map_fixed64_fixed64" => self.map_fixed64_fixed64.merge_text(stream),
                            "map_sfixed32_sfixed32" => {
                                self.map_sfixed32_sfixed32.merge_text(stream)
                            }
                            "map_sfixed64_sfixed64" => {
                                self.map_sfixed64_sfixed64.merge_text(stream)
                            }
                            "map_int32_float" => self.map_int32_float.merge_text(stream),
                            "map_int32_double" => self.map_int32_double.merge_text(stream),
                            "map_bool_bool" => self.map_bool_bool.merge_text(stream),
                            "map_string_string" => self.map_string_string.merge_text(stream),
                            "map_string_bytes" => self.map_string_bytes.merge_text(stream),
                            "map_string_nested_message" => {
                                self.map_string_nested_message.merge_text(stream)
                            }
                            "map_string_foreign_message" => {
                                self.map_string_foreign_message.merge_text(stream)
                            }
                            "map_string_nested_enum" => {
                                self.map_string_nested_enum.merge_text(stream)
                            }
                            "map_string_foreign_enum" => {
                                self.map_string_foreign_enum.merge_text(stream)
                            }
                            "Data" => self.Data.merge_text(stream),
                            "default_int32" => self.default_int32.merge_text(stream),
                            "default_int64" => self.default_int64.merge_text(stream),
                            "default_uint32" => self.default_uint32.merge_text(stream),
                            "default_uint64" => self.default_uint64.merge_text(stream),
                            "default_sint32" => self.default_sint32.merge_text(stream),
                            "default_sint64" => self.default_sint64.merge_text(stream),
                            "default_fixed32" => self.default_fixed32.merge_text(stream),
                            "default_fixed64" => self.default_fixed64.merge_text(stream),
                            "default_sfixed32" => self.default_sfixed32.merge_text(stream),
                            "default_sfixed64" => self.default_sfixed64.merge_text(stream),
                            "default_float" => self.default_float.merge_text(stream),
                            "default_double" => self.default_double.merge_text(stream),
                            "default_bool" => self.default_bool.merge_text(stream),
                            "default_string" => self.default_string.merge_text(stream),
                            "default_bytes" => self.default_bytes.merge_text(stream),
                            "fieldname1" => self.fieldname1.merge_text(stream),
                            "field_name2" => self.field_name2.merge_text(stream),
                            "_field_name3" => self._field_name3.merge_text(stream),
                            "field__name4_" => self.field__name4_.merge_text(stream),
                            "field0name5" => self.field0name5.merge_text(stream),
                            "field_0_name6" => self.field_0_name6.merge_text(stream),
                            "fieldName7" => self.fieldName7.merge_text(stream),
                            "FieldName8" => self.FieldName8.merge_text(stream),
                            "field_Name9" => self.field_Name9.merge_text(stream),
                            "Field_Name10" => self.Field_Name10.merge_text(stream),
                            "FIELD_NAME11" => self.FIELD_NAME11.merge_text(stream),
                            "FIELD_name12" => self.FIELD_name12.merge_text(stream),
                            "__field_name13" => self.__field_name13.merge_text(stream),
                            "__Field_name14" => self.__Field_name14.merge_text(stream),
                            "field__name15" => self.field__name15.merge_text(stream),
                            "field__Name16" => self.field__Name16.merge_text(stream),
                            "field_name17__" => self.field_name17__.merge_text(stream),
                            "Field_name18__" => self.Field_name18__.merge_text(stream),
                            "oneof_uint32"
                            | "oneof_nested_message"
                            | "oneof_string"
                            | "oneof_bytes"
                            | "oneof_bool"
                            | "oneof_uint64"
                            | "oneof_float"
                            | "oneof_double"
                            | "oneof_enum" => self.oneof_field.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("optional_int32", &self.optional_int32);
                        stream.emit_field("optional_int64", &self.optional_int64);
                        stream.emit_field("optional_uint32", &self.optional_uint32);
                        stream.emit_field("optional_uint64", &self.optional_uint64);
                        stream.emit_field("optional_sint32", &self.optional_sint32);
                        stream.emit_field("optional_sint64", &self.optional_sint64);
                        stream.emit_field("optional_fixed32", &self.optional_fixed32);
                        stream.emit_field("optional_fixed64", &self.optional_fixed64);
                        stream.emit_field("optional_sfixed32", &self.optional_sfixed32);
                        stream.emit_field("optional_sfixed64", &self.optional_sfixed64);
                        stream.emit_field("optional_float", &self.optional_float);
                        stream.emit_field("optional_double", &self.optional_double);
                        stream.emit_field("optional_bool", &self.optional_bool);
                        stream.emit_field("optional_string", &self.optional_string);
                        stream.emit_field("optional_bytes", &self.optional_bytes);
                        stream.emit_field("optional_nested_message", &self.optional_nested_message);
                        stream
                            .emit_field("optional_foreign_message", &self.optional_foreign_message);
                        stream.emit_field("optional_nested_enum", &self.optional_nested_enum);
                        stream.emit_field("optional_foreign_enum", &self.optional_foreign_enum);
                        stream.emit_field("optional_string_piece", &self.optional_string_piece);
                        stream.emit_field("optional_cord", &self.optional_cord);
                        stream.emit_field("recursive_message", &self.recursive_message);
                        stream.emit_field("repeated_int32", &self.repeated_int32);
                        stream.emit_field("repeated_int64", &self.repeated_int64);
                        stream.emit_field("repeated_uint32", &self.repeated_uint32);
                        stream.emit_field("repeated_uint64", &self.repeated_uint64);
                        stream.emit_field("repeated_sint32", &self.repeated_sint32);
                        stream.emit_field("repeated_sint64", &self.repeated_sint64);
                        stream.emit_field("repeated_fixed32", &self.repeated_fixed32);
                        stream.emit_field("repeated_fixed64", &self.repeated_fixed64);
                        stream.emit_field("repeated_sfixed32", &self.repeated_sfixed32);
                        stream.emit_field("repeated_sfixed64", &self.repeated_sfixed64);
                        stream.emit_field("repeated_float", &self.repeated_float);
                        stream.emit_field("repeated_double", &self.repeated_double);
                        stream.emit_field("repeated_bool", &self.repeated_bool);
                        stream.emit_field("repeated_string", &self.repeated_string);
                        stream.emit_field("repeated_bytes", &self.repeated_bytes);
                        stream.emit_field("repeated_nested_message", &self.repeated_nested_message);
                        stream
                            .emit_field("repeated_foreign_message", &self.repeated_foreign_message);
                        stream.emit_field("repeated_nested_enum", &self.repeated_nested_enum);
                        stream.emit_field("repeated_foreign_enum", &self.repeated_foreign_enum);
                        stream.emit_field("repeated_string_piece", &self.repeated_string_piece);
                        stream.emit_field("repeated_cord", &self.repeated_cord);
                        stream.emit_field("packed_int32", &self.packed_int32);
                        stream.emit_field("packed_int64", &self.packed_int64);
                        stream.emit_field("packed_uint32", &self.packed_uint32);
                        stream.emit_field("packed_uint64", &self.packed_uint64);
                        stream.emit_field("packed_sint32", &self.packed_sint32);
                        stream.emit_field("packed_sint64", &self.packed_sint64);
                        stream.emit_field("packed_fixed32", &self.packed_fixed32);
                        stream.emit_field("packed_fixed64", &self.packed_fixed64);
                        stream.emit_field("packed_sfixed32", &self.packed_sfixed32);
                        stream.emit_field("packed_sfixed64", &self.packed_sfixed64);
                        stream.emit_field("packed_float", &self.packed_float);
                        stream.emit_field("packed_double", &self.packed_double);
                        stream.emit_field("packed_bool", &self.packed_bool);
                        stream.emit_field("packed_nested_enum", &self.packed_nested_enum);
                        stream.emit_field("unpacked_int32", &self.unpacked_int32);
                        stream.emit_field("unpacked_int64", &self.unpacked_int64);
                        stream.emit_field("unpacked_uint32", &self.unpacked_uint32);
                        stream.emit_field("unpacked_uint64", &self.unpacked_uint64);
                        stream.emit_field("unpacked_sint32", &self.unpacked_sint32);
                        stream.emit_field("unpacked_sint64", &self.unpacked_sint64);
                        stream.emit_field("unpacked_fixed32", &self.unpacked_fixed32);
                        stream.emit_field("unpacked_fixed64", &self.unpacked_fixed64);
                        stream.emit_field("unpacked_sfixed32", &self.unpacked_sfixed32);
                        stream.emit_field("unpacked_sfixed64", &self.unpacked_sfixed64);
                        stream.emit_field("unpacked_float", &self.unpacked_float);
                        stream.emit_field("unpacked_double", &self.unpacked_double);
                        stream.emit_field("unpacked_bool", &self.unpacked_bool);
                        stream.emit_field("unpacked_nested_enum", &self.unpacked_nested_enum);
                        stream.emit_field("map_int32_int32", &self.map_int32_int32);
                        stream.emit_field("map_int64_int64", &self.map_int64_int64);
                        stream.emit_field("map_uint32_uint32", &self.map_uint32_uint32);
                        stream.emit_field("map_uint64_uint64", &self.map_uint64_uint64);
                        stream.emit_field("map_sint32_sint32", &self.map_sint32_sint32);
                        stream.emit_field("map_sint64_sint64", &self.map_sint64_sint64);
                        stream.emit_field("map_fixed32_fixed32", &self.map_fixed32_fixed32);
                        stream.emit_field("map_fixed64_fixed64", &self.map_fixed64_fixed64);
                        stream.emit_field("map_sfixed32_sfixed32", &self.map_sfixed32_sfixed32);
                        stream.emit_field("map_sfixed64_sfixed64", &self.map_sfixed64_sfixed64);
                        stream.emit_field("map_int32_float", &self.map_int32_float);
                        stream.emit_field("map_int32_double", &self.map_int32_double);
                        stream.emit_field("map_bool_bool", &self.map_bool_bool);
                        stream.emit_field("map_string_string", &self.map_string_string);
                        stream.emit_field("map_string_bytes", &self.map_string_bytes);
                        stream.emit_field(
                            "map_string_nested_message",
                            &self.map_string_nested_message,
                        );
                        stream.emit_field(
                            "map_string_foreign_message",
                            &self.map_string_foreign_message,
                        );
                        stream.emit_field("map_string_nested_enum", &self.map_string_nested_enum);
                        stream.emit_field("map_string_foreign_enum", &self.map_string_foreign_enum);
                        stream.emit_field("Data", &self.Data);
                        stream.emit_field("default_int32", &self.default_int32);
                        stream.emit_field("default_int64", &self.default_int64);
                        stream.emit_field("default_uint32", &self.default_uint32);
                        stream.emit_field("default_uint64", &self.default_uint64);
                        stream.emit_field("default_sint32", &self.default_sint32);
                        stream.emit_field("default_sint64", &self.default_sint64);
                        stream.emit_field("default_fixed32", &self.default_fixed32);
                        stream.emit_field("default_fixed64", &self.default_fixed64);
                        stream.emit_field("default_sfixed32", &self.default_sfixed32);
                        stream.emit_field("default_sfixed64", &self.default_sfixed64);
                        stream.emit_field("default_float", &self.default_float);
                        stream.emit_field("default_double", &self.default_double);
                        stream.emit_field("default_bool", &self.default_bool);
                        stream.emit_field("default_string", &self.default_string);
                        stream.emit_field("default_bytes", &self.default_bytes);
                        stream.emit_field("fieldname1", &self.fieldname1);
                        stream.emit_field("field_name2", &self.field_name2);
                        stream.emit_field("_field_name3", &self._field_name3);
                        stream.emit_field("field__name4_", &self.field__name4_);
                        stream.emit_field("field0name5", &self.field0name5);
                        stream.emit_field("field_0_name6", &self.field_0_name6);
                        stream.emit_field("fieldName7", &self.fieldName7);
                        stream.emit_field("FieldName8", &self.FieldName8);
                        stream.emit_field("field_Name9", &self.field_Name9);
                        stream.emit_field("Field_Name10", &self.Field_Name10);
                        stream.emit_field("FIELD_NAME11", &self.FIELD_NAME11);
                        stream.emit_field("FIELD_name12", &self.FIELD_name12);
                        stream.emit_field("__field_name13", &self.__field_name13);
                        stream.emit_field("__Field_name14", &self.__Field_name14);
                        stream.emit_field("field__name15", &self.field__name15);
                        stream.emit_field("field__Name16", &self.field__Name16);
                        stream.emit_field("field_name17__", &self.field_name17__);
                        stream.emit_field("Field_name18__", &self.Field_name18__);
                        stream.emit_oneof(&self.oneof_field);
                    }
                }

                pub struct TestAllTypesProto2NestedMessage {
                    #[field(1u32, "a", varint, optional)]
                    pub a: Option<i32>,
                    #[field(2u32, "corecursive", nested, optional)]
                    pub corecursive: Option<Box<TestAllTypesProto2>>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2NestedMessage {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TestAllTypesProto2NestedMessage",
                            "a",
                            &&self.a,
                            "corecursive",
                            &&self.corecursive,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2NestedMessage {
                    #[inline]
                    fn default() -> TestAllTypesProto2NestedMessage {
                        TestAllTypesProto2NestedMessage {
                            a: ::core::default::Default::default(),
                            corecursive: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2NestedMessage {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2NestedMessage {
                        TestAllTypesProto2NestedMessage {
                            a: ::core::clone::Clone::clone(&self.a),
                            corecursive: ::core::clone::Clone::clone(&self.corecursive),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2NestedMessage {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2NestedMessage {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2NestedMessage) -> bool {
                        self.a == other.a & &self.corecursive == other.corecursive
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2NestedMessage {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_optional(
                                &mut self.a,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            18u32 => binformat::merge_optional(
                                &mut self.corecursive,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.a,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.corecursive,
                            18u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2NestedMessage {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "a" => self.a.merge_text(stream),
                            "corecursive" => self.corecursive.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("a", &self.a);
                        stream.emit_field("corecursive", &self.corecursive);
                    }
                }

                pub struct TestAllTypesProto2Data {
                    #[field(202u32, "group_int32", varint, optional)]
                    pub group_int32: Option<i32>,
                    #[field(203u32, "group_uint32", varint, optional)]
                    pub group_uint32: Option<u32>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2Data {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TestAllTypesProto2Data",
                            "group_int32",
                            &&self.group_int32,
                            "group_uint32",
                            &&self.group_uint32,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2Data {
                    #[inline]
                    fn default() -> TestAllTypesProto2Data {
                        TestAllTypesProto2Data {
                            group_int32: ::core::default::Default::default(),
                            group_uint32: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2Data {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2Data {
                        TestAllTypesProto2Data {
                            group_int32: ::core::clone::Clone::clone(&self.group_int32),
                            group_uint32: ::core::clone::Clone::clone(&self.group_uint32),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2Data {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2Data {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2Data) -> bool {
                        self.group_int32 == other.group_int32
                            & &self.group_uint32 == other.group_uint32
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2Data {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            1616u32 => binformat::merge_optional(
                                &mut self.group_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            1624u32 => binformat::merge_optional(
                                &mut self.group_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.group_int32,
                            1616u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.group_uint32,
                            1624u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2Data {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "group_int32" => self.group_int32.merge_text(stream),
                            "group_uint32" => self.group_uint32.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("group_int32", &self.group_int32);
                        stream.emit_field("group_uint32", &self.group_uint32);
                    }
                }

                pub struct TestAllTypesProto2MessageSetCorrect {}

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2MessageSetCorrect {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "TestAllTypesProto2MessageSetCorrect")
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2MessageSetCorrect {
                    #[inline]
                    fn default() -> TestAllTypesProto2MessageSetCorrect {
                        TestAllTypesProto2MessageSetCorrect {}
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2MessageSetCorrect {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2MessageSetCorrect {
                        TestAllTypesProto2MessageSetCorrect {}
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2MessageSetCorrect {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2MessageSetCorrect {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2MessageSetCorrect) -> bool {
                        true
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2MessageSetCorrect {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {}
                }

                impl textformat::TextProto for TestAllTypesProto2MessageSetCorrect {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {}
                }

                pub struct TestAllTypesProto2MessageSetCorrectExtension1 {
                    #[field(1547769u32, "message_set_extension", nested, optional)]
                    pub message_set_extension:
                    Option<Box<TestAllTypesProto2MessageSetCorrectExtension1>>,
                    #[field(25u32, "str", string, optional)]
                    pub str: Option<String>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2MessageSetCorrectExtension1 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TestAllTypesProto2MessageSetCorrectExtension1",
                            "message_set_extension",
                            &&self.message_set_extension,
                            "str",
                            &&self.str,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2MessageSetCorrectExtension1 {
                    #[inline]
                    fn default() -> TestAllTypesProto2MessageSetCorrectExtension1 {
                        TestAllTypesProto2MessageSetCorrectExtension1 {
                            message_set_extension: ::core::default::Default::default(),
                            str: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2MessageSetCorrectExtension1 {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2MessageSetCorrectExtension1 {
                        TestAllTypesProto2MessageSetCorrectExtension1 {
                            message_set_extension: ::core::clone::Clone::clone(
                                &self.message_set_extension,
                            ),
                            str: ::core::clone::Clone::clone(&self.str),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2MessageSetCorrectExtension1 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2MessageSetCorrectExtension1 {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2MessageSetCorrectExtension1) -> bool {
                        self.message_set_extension == other.message_set_extension
                            & &self.str == other.str
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2MessageSetCorrectExtension1 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            12382154u32 => binformat::merge_optional(
                                &mut self.message_set_extension,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            202u32 => binformat::merge_optional(
                                &mut self.str,
                                stream,
                                binformat::InputStream::string,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.message_set_extension,
                            12382154u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.str,
                            202u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2MessageSetCorrectExtension1 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "message_set_extension" => {
                                self.message_set_extension.merge_text(stream)
                            }
                            "str" => self.str.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("message_set_extension", &self.message_set_extension);
                        stream.emit_field("str", &self.str);
                    }
                }

                pub struct TestAllTypesProto2MessageSetCorrectExtension2 {
                    #[field(4135312u32, "message_set_extension", nested, optional)]
                    pub message_set_extension:
                    Option<Box<TestAllTypesProto2MessageSetCorrectExtension2>>,
                    #[field(9u32, "i", varint, optional)]
                    pub i: Option<i32>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto2MessageSetCorrectExtension2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TestAllTypesProto2MessageSetCorrectExtension2",
                            "message_set_extension",
                            &&self.message_set_extension,
                            "i",
                            &&self.i,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto2MessageSetCorrectExtension2 {
                    #[inline]
                    fn default() -> TestAllTypesProto2MessageSetCorrectExtension2 {
                        TestAllTypesProto2MessageSetCorrectExtension2 {
                            message_set_extension: ::core::default::Default::default(),
                            i: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto2MessageSetCorrectExtension2 {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto2MessageSetCorrectExtension2 {
                        TestAllTypesProto2MessageSetCorrectExtension2 {
                            message_set_extension: ::core::clone::Clone::clone(
                                &self.message_set_extension,
                            ),
                            i: ::core::clone::Clone::clone(&self.i),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto2MessageSetCorrectExtension2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto2MessageSetCorrectExtension2 {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto2MessageSetCorrectExtension2) -> bool {
                        self.message_set_extension == other.message_set_extension
                            & &self.i == other.i
                    }
                }

                impl binformat::BinProto for TestAllTypesProto2MessageSetCorrectExtension2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            33082498u32 => binformat::merge_optional(
                                &mut self.message_set_extension,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            72u32 => binformat::merge_optional(
                                &mut self.i,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.message_set_extension,
                            33082498u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.i,
                            72u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for TestAllTypesProto2MessageSetCorrectExtension2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "message_set_extension" => {
                                self.message_set_extension.merge_text(stream)
                            }
                            "i" => self.i.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("message_set_extension", &self.message_set_extension);
                        stream.emit_field("i", &self.i);
                    }
                }

                pub struct ForeignMessageProto2 {
                    #[field(1u32, "c", varint, optional)]
                    pub c: Option<i32>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ForeignMessageProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ForeignMessageProto2",
                            "c",
                            &&self.c,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for ForeignMessageProto2 {
                    #[inline]
                    fn default() -> ForeignMessageProto2 {
                        ForeignMessageProto2 {
                            c: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ForeignMessageProto2 {
                    #[inline]
                    fn clone(&self) -> ForeignMessageProto2 {
                        ForeignMessageProto2 {
                            c: ::core::clone::Clone::clone(&self.c),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ForeignMessageProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ForeignMessageProto2 {
                    #[inline]
                    fn eq(&self, other: &ForeignMessageProto2) -> bool {
                        self.c == other.c
                    }
                }

                impl binformat::BinProto for ForeignMessageProto2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_optional(
                                &mut self.c,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.c,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for ForeignMessageProto2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "c" => self.c.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("c", &self.c);
                    }
                }

                pub struct UnknownToTestAllTypes {
                    #[field(1001u32, "optional_int32", varint, optional)]
                    pub optional_int32: Option<i32>,
                    #[field(1002u32, "optional_string", string, optional)]
                    pub optional_string: Option<String>,
                    #[field(1003u32, "nested_message", nested, optional)]
                    pub nested_message: Option<Box<ForeignMessageProto2>>,
                    #[field(1004u32, "OptionalGroup", nested, optional)]
                    pub OptionalGroup: Option<Box<UnknownToTestAllTypesOptionalGroup>>,
                    #[field(1006u32, "optional_bool", bool, optional)]
                    pub optional_bool: Option<bool>,
                    #[field(1011u32, "repeated_int32", varint, repeated)]
                    pub repeated_int32: Vec<i32>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for UnknownToTestAllTypes {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "optional_int32",
                            "optional_string",
                            "nested_message",
                            "OptionalGroup",
                            "optional_bool",
                            "repeated_int32",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &&self.optional_int32,
                            &&self.optional_string,
                            &&self.nested_message,
                            &&self.OptionalGroup,
                            &&self.optional_bool,
                            &&self.repeated_int32,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "UnknownToTestAllTypes",
                            names,
                            values,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for UnknownToTestAllTypes {
                    #[inline]
                    fn default() -> UnknownToTestAllTypes {
                        UnknownToTestAllTypes {
                            optional_int32: ::core::default::Default::default(),
                            optional_string: ::core::default::Default::default(),
                            nested_message: ::core::default::Default::default(),
                            OptionalGroup: ::core::default::Default::default(),
                            optional_bool: ::core::default::Default::default(),
                            repeated_int32: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for UnknownToTestAllTypes {
                    #[inline]
                    fn clone(&self) -> UnknownToTestAllTypes {
                        UnknownToTestAllTypes {
                            optional_int32: ::core::clone::Clone::clone(&self.optional_int32),
                            optional_string: ::core::clone::Clone::clone(&self.optional_string),
                            nested_message: ::core::clone::Clone::clone(&self.nested_message),
                            OptionalGroup: ::core::clone::Clone::clone(&self.OptionalGroup),
                            optional_bool: ::core::clone::Clone::clone(&self.optional_bool),
                            repeated_int32: ::core::clone::Clone::clone(&self.repeated_int32),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for UnknownToTestAllTypes {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for UnknownToTestAllTypes {
                    #[inline]
                    fn eq(&self, other: &UnknownToTestAllTypes) -> bool {
                        self.optional_int32 == other.optional_int32
                            & &self.optional_string == other.optional_string
                            & &self.nested_message == other.nested_message
                            & &self.OptionalGroup == other.OptionalGroup
                            & &self.optional_bool == other.optional_bool
                            & &self.repeated_int32 == other.repeated_int32
                    }
                }

                impl binformat::BinProto for UnknownToTestAllTypes {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8008u32 => binformat::merge_optional(
                                &mut self.optional_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            8018u32 => binformat::merge_optional(
                                &mut self.optional_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            8026u32 => binformat::merge_optional(
                                &mut self.nested_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            8034u32 => binformat::merge_optional(
                                &mut self.OptionalGroup,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            8048u32 => binformat::merge_optional(
                                &mut self.optional_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            8088u32 => binformat::merge_repeated(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            8090u32 => binformat::merge_packed(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.optional_int32,
                            8008u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.optional_string,
                            8018u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.nested_message,
                            8026u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.OptionalGroup,
                            8034u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_bool,
                            8048u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_repeated(
                            &self.repeated_int32,
                            8088u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for UnknownToTestAllTypes {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "optional_int32" => self.optional_int32.merge_text(stream),
                            "optional_string" => self.optional_string.merge_text(stream),
                            "nested_message" => self.nested_message.merge_text(stream),
                            "OptionalGroup" => self.OptionalGroup.merge_text(stream),
                            "optional_bool" => self.optional_bool.merge_text(stream),
                            "repeated_int32" => self.repeated_int32.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("optional_int32", &self.optional_int32);
                        stream.emit_field("optional_string", &self.optional_string);
                        stream.emit_field("nested_message", &self.nested_message);
                        stream.emit_field("OptionalGroup", &self.OptionalGroup);
                        stream.emit_field("optional_bool", &self.optional_bool);
                        stream.emit_field("repeated_int32", &self.repeated_int32);
                    }
                }

                pub struct UnknownToTestAllTypesOptionalGroup {
                    #[field(1u32, "a", varint, optional)]
                    pub a: Option<i32>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for UnknownToTestAllTypesOptionalGroup {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "UnknownToTestAllTypesOptionalGroup",
                            "a",
                            &&self.a,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for UnknownToTestAllTypesOptionalGroup {
                    #[inline]
                    fn default() -> UnknownToTestAllTypesOptionalGroup {
                        UnknownToTestAllTypesOptionalGroup {
                            a: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for UnknownToTestAllTypesOptionalGroup {
                    #[inline]
                    fn clone(&self) -> UnknownToTestAllTypesOptionalGroup {
                        UnknownToTestAllTypesOptionalGroup {
                            a: ::core::clone::Clone::clone(&self.a),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for UnknownToTestAllTypesOptionalGroup {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for UnknownToTestAllTypesOptionalGroup {
                    #[inline]
                    fn eq(&self, other: &UnknownToTestAllTypesOptionalGroup) -> bool {
                        self.a == other.a
                    }
                }

                impl binformat::BinProto for UnknownToTestAllTypesOptionalGroup {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_optional(
                                &mut self.a,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.a,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for UnknownToTestAllTypesOptionalGroup {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "a" => self.a.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("a", &self.a);
                    }
                }

                pub struct NullHypothesisProto2 {}

                #[automatically_derived]
                impl ::core::fmt::Debug for NullHypothesisProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "NullHypothesisProto2")
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for NullHypothesisProto2 {
                    #[inline]
                    fn default() -> NullHypothesisProto2 {
                        NullHypothesisProto2 {}
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for NullHypothesisProto2 {
                    #[inline]
                    fn clone(&self) -> NullHypothesisProto2 {
                        NullHypothesisProto2 {}
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for NullHypothesisProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for NullHypothesisProto2 {
                    #[inline]
                    fn eq(&self, other: &NullHypothesisProto2) -> bool {
                        true
                    }
                }

                impl binformat::BinProto for NullHypothesisProto2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {}
                }

                impl textformat::TextProto for NullHypothesisProto2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {}
                }

                pub struct EnumOnlyProto2 {}

                #[automatically_derived]
                impl ::core::fmt::Debug for EnumOnlyProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "EnumOnlyProto2")
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for EnumOnlyProto2 {
                    #[inline]
                    fn default() -> EnumOnlyProto2 {
                        EnumOnlyProto2 {}
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for EnumOnlyProto2 {
                    #[inline]
                    fn clone(&self) -> EnumOnlyProto2 {
                        EnumOnlyProto2 {}
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for EnumOnlyProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for EnumOnlyProto2 {
                    #[inline]
                    fn eq(&self, other: &EnumOnlyProto2) -> bool {
                        true
                    }
                }

                impl binformat::BinProto for EnumOnlyProto2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {}
                }

                impl textformat::TextProto for EnumOnlyProto2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {}
                }

                pub struct OneStringProto2 {
                    #[field(1u32, "data", string, optional)]
                    pub data: Option<String>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for OneStringProto2 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OneStringProto2",
                            "data",
                            &&self.data,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for OneStringProto2 {
                    #[inline]
                    fn default() -> OneStringProto2 {
                        OneStringProto2 {
                            data: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for OneStringProto2 {
                    #[inline]
                    fn clone(&self) -> OneStringProto2 {
                        OneStringProto2 {
                            data: ::core::clone::Clone::clone(&self.data),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for OneStringProto2 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for OneStringProto2 {
                    #[inline]
                    fn eq(&self, other: &OneStringProto2) -> bool {
                        self.data == other.data
                    }
                }

                impl binformat::BinProto for OneStringProto2 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_optional(
                                &mut self.data,
                                stream,
                                binformat::InputStream::string,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.data,
                            10u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                    }
                }

                impl textformat::TextProto for OneStringProto2 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "data" => self.data.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("data", &self.data);
                    }
                }

                pub struct ProtoWithKeywords {
                    #[field(1u32, "inline", varint, optional)]
                    pub inline: Option<i32>,
                    #[field(2u32, "concept", string, optional)]
                    pub concept: Option<String>,
                    #[field(3u32, "requires", string, repeated)]
                    pub requires: Vec<String>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ProtoWithKeywords {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "ProtoWithKeywords",
                            "inline",
                            &&self.inline,
                            "concept",
                            &&self.concept,
                            "requires",
                            &&self.requires,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for ProtoWithKeywords {
                    #[inline]
                    fn default() -> ProtoWithKeywords {
                        ProtoWithKeywords {
                            inline: ::core::default::Default::default(),
                            concept: ::core::default::Default::default(),
                            requires: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ProtoWithKeywords {
                    #[inline]
                    fn clone(&self) -> ProtoWithKeywords {
                        ProtoWithKeywords {
                            inline: ::core::clone::Clone::clone(&self.inline),
                            concept: ::core::clone::Clone::clone(&self.concept),
                            requires: ::core::clone::Clone::clone(&self.requires),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ProtoWithKeywords {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ProtoWithKeywords {
                    #[inline]
                    fn eq(&self, other: &ProtoWithKeywords) -> bool {
                        self.inline == other.inline
                            & &self.concept == other.concept
                            & &self.requires == other.requires
                    }
                }

                impl binformat::BinProto for ProtoWithKeywords {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_optional(
                                &mut self.inline,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            18u32 => binformat::merge_optional(
                                &mut self.concept,
                                stream,
                                binformat::InputStream::string,
                            ),
                            26u32 => binformat::merge_repeated(
                                &mut self.requires,
                                stream,
                                binformat::InputStream::string,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_optional(
                            &self.inline,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.concept,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.requires,
                            26u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                    }
                }

                impl textformat::TextProto for ProtoWithKeywords {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "inline" => self.inline.merge_text(stream),
                            "concept" => self.concept.merge_text(stream),
                            "requires" => self.requires.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("inline", &self.inline);
                        stream.emit_field("concept", &self.concept);
                        stream.emit_field("requires", &self.requires);
                    }
                }
            }

            pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
                test_messages_proto2::register_types(registry);
            }
        }

        pub mod proto3 {
            pub mod test_messages_proto3 {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                use super::super::super::google::protobuf::any::*;
                use super::super::super::google::protobuf::duration::*;
                use super::super::super::google::protobuf::field_mask::*;
                use super::super::super::google::protobuf::r#struct::*;
                use super::super::super::google::protobuf::timestamp::*;
                use super::super::super::google::protobuf::wrappers::*;

                pub struct TestAllTypesProto3NestedEnum(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto3NestedEnum {
                    #[inline]
                    fn default() -> TestAllTypesProto3NestedEnum {
                        TestAllTypesProto3NestedEnum(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto3NestedEnum {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TestAllTypesProto3NestedEnum",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto3NestedEnum {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto3NestedEnum {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for TestAllTypesProto3NestedEnum {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto3NestedEnum {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto3NestedEnum {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto3NestedEnum) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for TestAllTypesProto3NestedEnum {}

                #[automatically_derived]
                impl ::core::cmp::Eq for TestAllTypesProto3NestedEnum {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for TestAllTypesProto3NestedEnum {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &TestAllTypesProto3NestedEnum,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for TestAllTypesProto3NestedEnum {
                    #[inline]
                    fn cmp(&self, other: &TestAllTypesProto3NestedEnum) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl TestAllTypesProto3NestedEnum {
                    pub const FOO: TestAllTypesProto3NestedEnum =
                        TestAllTypesProto3NestedEnum(0u32);
                    pub const BAR: TestAllTypesProto3NestedEnum =
                        TestAllTypesProto3NestedEnum(1u32);
                    pub const BAZ: TestAllTypesProto3NestedEnum =
                        TestAllTypesProto3NestedEnum(2u32);
                    pub const NEG: TestAllTypesProto3NestedEnum =
                        TestAllTypesProto3NestedEnum(4294967295u32);
                }

                impl From<u32> for TestAllTypesProto3NestedEnum {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<TestAllTypesProto3NestedEnum> for u32 {
                    fn from(v: TestAllTypesProto3NestedEnum) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for TestAllTypesProto3NestedEnum {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "FOO" => *self = Self::from(0u32),
                            "BAR" => *self = Self::from(1u32),
                            "BAZ" => *self = Self::from(2u32),
                            "NEG" => *self = Self::from(4294967295u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("FOO"),
                            1u32 => stream.ident("BAR"),
                            2u32 => stream.ident("BAZ"),
                            4294967295u32 => stream.ident("NEG"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct TestAllTypesProto3AliasedEnum(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    fn default() -> TestAllTypesProto3AliasedEnum {
                        TestAllTypesProto3AliasedEnum(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto3AliasedEnum {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "TestAllTypesProto3AliasedEnum",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto3AliasedEnum {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for TestAllTypesProto3AliasedEnum {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto3AliasedEnum {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto3AliasedEnum) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for TestAllTypesProto3AliasedEnum {}

                #[automatically_derived]
                impl ::core::cmp::Eq for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &TestAllTypesProto3AliasedEnum,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for TestAllTypesProto3AliasedEnum {
                    #[inline]
                    fn cmp(&self, other: &TestAllTypesProto3AliasedEnum) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl TestAllTypesProto3AliasedEnum {
                    pub const ALIAS_FOO: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(0u32);
                    pub const ALIAS_BAR: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(1u32);
                    pub const ALIAS_BAZ: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(2u32);
                    pub const MOO: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(2u32);
                    pub const moo: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(2u32);
                    pub const bAz: TestAllTypesProto3AliasedEnum =
                        TestAllTypesProto3AliasedEnum(2u32);
                }

                impl From<u32> for TestAllTypesProto3AliasedEnum {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<TestAllTypesProto3AliasedEnum> for u32 {
                    fn from(v: TestAllTypesProto3AliasedEnum) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for TestAllTypesProto3AliasedEnum {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "ALIAS_FOO" => *self = Self::from(0u32),
                            "ALIAS_BAR" => *self = Self::from(1u32),
                            "ALIAS_BAZ" => *self = Self::from(2u32),
                            "MOO" => *self = Self::from(2u32),
                            "moo" => *self = Self::from(2u32),
                            "bAz" => *self = Self::from(2u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("ALIAS_FOO"),
                            1u32 => stream.ident("ALIAS_BAR"),
                            2u32 => stream.ident("ALIAS_BAZ"),
                            2u32 => stream.ident("MOO"),
                            2u32 => stream.ident("moo"),
                            2u32 => stream.ident("bAz"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct ForeignEnum(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for ForeignEnum {
                    #[inline]
                    fn default() -> ForeignEnum {
                        ForeignEnum(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ForeignEnum {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ForeignEnum",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ForeignEnum {
                    #[inline]
                    fn clone(&self) -> ForeignEnum {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for ForeignEnum {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ForeignEnum {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ForeignEnum {
                    #[inline]
                    fn eq(&self, other: &ForeignEnum) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for ForeignEnum {}

                #[automatically_derived]
                impl ::core::cmp::Eq for ForeignEnum {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for ForeignEnum {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &ForeignEnum,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for ForeignEnum {
                    #[inline]
                    fn cmp(&self, other: &ForeignEnum) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl ForeignEnum {
                    pub const FOREIGN_FOO: ForeignEnum = ForeignEnum(0u32);
                    pub const FOREIGN_BAR: ForeignEnum = ForeignEnum(1u32);
                    pub const FOREIGN_BAZ: ForeignEnum = ForeignEnum(2u32);
                }

                impl From<u32> for ForeignEnum {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<ForeignEnum> for u32 {
                    fn from(v: ForeignEnum) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for ForeignEnum {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "FOREIGN_FOO" => *self = Self::from(0u32),
                            "FOREIGN_BAR" => *self = Self::from(1u32),
                            "FOREIGN_BAZ" => *self = Self::from(2u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("FOREIGN_FOO"),
                            1u32 => stream.ident("FOREIGN_BAR"),
                            2u32 => stream.ident("FOREIGN_BAZ"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct EnumOnlyProto3Bool(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for EnumOnlyProto3Bool {
                    #[inline]
                    fn default() -> EnumOnlyProto3Bool {
                        EnumOnlyProto3Bool(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for EnumOnlyProto3Bool {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "EnumOnlyProto3Bool",
                            &&self.0,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for EnumOnlyProto3Bool {
                    #[inline]
                    fn clone(&self) -> EnumOnlyProto3Bool {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for EnumOnlyProto3Bool {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for EnumOnlyProto3Bool {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for EnumOnlyProto3Bool {
                    #[inline]
                    fn eq(&self, other: &EnumOnlyProto3Bool) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for EnumOnlyProto3Bool {}

                #[automatically_derived]
                impl ::core::cmp::Eq for EnumOnlyProto3Bool {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for EnumOnlyProto3Bool {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &EnumOnlyProto3Bool,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for EnumOnlyProto3Bool {
                    #[inline]
                    fn cmp(&self, other: &EnumOnlyProto3Bool) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl EnumOnlyProto3Bool {
                    pub const kFalse: EnumOnlyProto3Bool = EnumOnlyProto3Bool(0u32);
                    pub const kTrue: EnumOnlyProto3Bool = EnumOnlyProto3Bool(1u32);
                }

                impl From<u32> for EnumOnlyProto3Bool {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<EnumOnlyProto3Bool> for u32 {
                    fn from(v: EnumOnlyProto3Bool) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for EnumOnlyProto3Bool {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "kFalse" => *self = Self::from(0u32),
                            "kTrue" => *self = Self::from(1u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("kFalse"),
                            1u32 => stream.ident("kTrue"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub enum TestAllTypesProto3OneOfOneofField {
                    #[field(111u32, "oneof_uint32", varint, raw)]
                    OneofUint32(u32),
                    #[field(112u32, "oneof_nested_message", nested, raw)]
                    OneofNestedMessage(TestAllTypesProto3NestedMessage),
                    #[field(113u32, "oneof_string", string, raw)]
                    OneofString(String),
                    #[field(114u32, "oneof_bytes", bytes, raw)]
                    OneofBytes(Vec<u8>),
                    #[field(115u32, "oneof_bool", bool, raw)]
                    OneofBool(bool),
                    #[field(116u32, "oneof_uint64", varint, raw)]
                    OneofUint64(u64),
                    #[field(117u32, "oneof_float", fixed32, raw)]
                    OneofFloat(f32),
                    #[field(118u32, "oneof_double", fixed64, raw)]
                    OneofDouble(f64),
                    #[field(119u32, "oneof_enum", protoenum, raw)]
                    OneofEnum(TestAllTypesProto3NestedEnum),
                    #[field(120u32, "oneof_null_value", protoenum, raw)]
                    OneofNullValue(NullValue),
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto3OneOfOneofField {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match self {
                            TestAllTypesProto3OneOfOneofField::OneofUint32(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofUint32",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofNestedMessage(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofNestedMessage",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofString(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofString",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofBytes(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofBytes",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofBool(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofBool",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofUint64(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofUint64",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofFloat(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofFloat",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofDouble(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofDouble",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofEnum(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofEnum",
                                    &__self_0,
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofNullValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "OneofNullValue",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto3OneOfOneofField {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto3OneOfOneofField {
                        match self {
                            TestAllTypesProto3OneOfOneofField::OneofUint32(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofUint32(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofNestedMessage(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofNestedMessage(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofString(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofString(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofBytes(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofBytes(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofBool(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofBool(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofUint64(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofUint64(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofFloat(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofFloat(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofDouble(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofDouble(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofEnum(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofEnum(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            TestAllTypesProto3OneOfOneofField::OneofNullValue(__self_0) => {
                                TestAllTypesProto3OneOfOneofField::OneofNullValue(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto3OneOfOneofField {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto3OneOfOneofField {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto3OneOfOneofField) -> bool {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                        __self_tag == __arg1_tag
                            && match (self, other) {
                            (
                                TestAllTypesProto3OneOfOneofField::OneofUint32(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofUint32(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofNestedMessage(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofNestedMessage(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofString(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofString(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofBytes(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofBytes(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofBool(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofBool(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofUint64(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofUint64(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofFloat(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofFloat(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofDouble(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofDouble(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofEnum(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofEnum(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                TestAllTypesProto3OneOfOneofField::OneofNullValue(__self_0),
                                TestAllTypesProto3OneOfOneofField::OneofNullValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    }
                }

                impl TestAllTypesProto3OneOfOneofField {
                    fn make_OneofUint32_mut(&mut self) -> &mut u32 {
                        loop {
                            match self {
                                Self::OneofUint32(v) => return v,
                                _ => *self = Self::OneofUint32(Default::default()),
                            }
                        }
                    }
                    fn make_OneofNestedMessage_mut(
                        &mut self,
                    ) -> &mut TestAllTypesProto3NestedMessage {
                        loop {
                            match self {
                                Self::OneofNestedMessage(v) => return v,
                                _ => *self = Self::OneofNestedMessage(Default::default()),
                            }
                        }
                    }
                    fn make_OneofString_mut(&mut self) -> &mut String {
                        loop {
                            match self {
                                Self::OneofString(v) => return v,
                                _ => *self = Self::OneofString(Default::default()),
                            }
                        }
                    }
                    fn make_OneofBytes_mut(&mut self) -> &mut Vec<u8> {
                        loop {
                            match self {
                                Self::OneofBytes(v) => return v,
                                _ => *self = Self::OneofBytes(Default::default()),
                            }
                        }
                    }
                    fn make_OneofBool_mut(&mut self) -> &mut bool {
                        loop {
                            match self {
                                Self::OneofBool(v) => return v,
                                _ => *self = Self::OneofBool(Default::default()),
                            }
                        }
                    }
                    fn make_OneofUint64_mut(&mut self) -> &mut u64 {
                        loop {
                            match self {
                                Self::OneofUint64(v) => return v,
                                _ => *self = Self::OneofUint64(Default::default()),
                            }
                        }
                    }
                    fn make_OneofFloat_mut(&mut self) -> &mut f32 {
                        loop {
                            match self {
                                Self::OneofFloat(v) => return v,
                                _ => *self = Self::OneofFloat(Default::default()),
                            }
                        }
                    }
                    fn make_OneofDouble_mut(&mut self) -> &mut f64 {
                        loop {
                            match self {
                                Self::OneofDouble(v) => return v,
                                _ => *self = Self::OneofDouble(Default::default()),
                            }
                        }
                    }
                    fn make_OneofEnum_mut(&mut self) -> &mut TestAllTypesProto3NestedEnum {
                        loop {
                            match self {
                                Self::OneofEnum(v) => return v,
                                _ => *self = Self::OneofEnum(Default::default()),
                            }
                        }
                    }
                    fn make_OneofNullValue_mut(&mut self) -> &mut NullValue {
                        loop {
                            match self {
                                Self::OneofNullValue(v) => return v,
                                _ => *self = Self::OneofNullValue(Default::default()),
                            }
                        }
                    }
                }

                impl binformat::BinProto for TestAllTypesProto3OneOfOneofField {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            888u32 => binformat::merge_single(
                                self.make_OneofUint32_mut(),
                                stream,
                                binformat::InputStream::varint,
                            ),
                            898u32 => binformat::merge_single(
                                self.make_OneofNestedMessage_mut(),
                                stream,
                                binformat::InputStream::nested,
                            ),
                            906u32 => binformat::merge_single(
                                self.make_OneofString_mut(),
                                stream,
                                binformat::InputStream::string,
                            ),
                            914u32 => binformat::merge_single(
                                self.make_OneofBytes_mut(),
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            920u32 => binformat::merge_single(
                                self.make_OneofBool_mut(),
                                stream,
                                binformat::InputStream::bool,
                            ),
                            928u32 => binformat::merge_single(
                                self.make_OneofUint64_mut(),
                                stream,
                                binformat::InputStream::varint,
                            ),
                            941u32 => binformat::merge_single(
                                self.make_OneofFloat_mut(),
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            945u32 => binformat::merge_single(
                                self.make_OneofDouble_mut(),
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            952u32 => binformat::merge_single(
                                self.make_OneofEnum_mut(),
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            960u32 => binformat::merge_single(
                                self.make_OneofNullValue_mut(),
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        match self {
                            Self::OneofUint32(v) => {
                                binformat::emit_raw(
                                    v,
                                    888u32,
                                    stream,
                                    binformat::OutputStream::varint,
                                );
                            }
                            Self::OneofNestedMessage(v) => {
                                binformat::emit_raw(
                                    v,
                                    898u32,
                                    stream,
                                    binformat::OutputStream::nested,
                                );
                            }
                            Self::OneofString(v) => {
                                binformat::emit_raw(
                                    v,
                                    906u32,
                                    stream,
                                    binformat::OutputStream::string,
                                );
                            }
                            Self::OneofBytes(v) => {
                                binformat::emit_raw(
                                    v,
                                    914u32,
                                    stream,
                                    binformat::OutputStream::bytes,
                                );
                            }
                            Self::OneofBool(v) => {
                                binformat::emit_raw(
                                    v,
                                    920u32,
                                    stream,
                                    binformat::OutputStream::bool,
                                );
                            }
                            Self::OneofUint64(v) => {
                                binformat::emit_raw(
                                    v,
                                    928u32,
                                    stream,
                                    binformat::OutputStream::varint,
                                );
                            }
                            Self::OneofFloat(v) => {
                                binformat::emit_raw(
                                    v,
                                    941u32,
                                    stream,
                                    binformat::OutputStream::fixed32,
                                );
                            }
                            Self::OneofDouble(v) => {
                                binformat::emit_raw(
                                    v,
                                    945u32,
                                    stream,
                                    binformat::OutputStream::fixed64,
                                );
                            }
                            Self::OneofEnum(v) => {
                                binformat::emit_raw(
                                    v,
                                    952u32,
                                    stream,
                                    binformat::OutputStream::protoenum,
                                );
                            }
                            Self::OneofNullValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    960u32,
                                    stream,
                                    binformat::OutputStream::protoenum,
                                );
                            }
                        }
                    }
                }

                impl textformat::TextProto for TestAllTypesProto3OneOfOneofField {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "oneof_uint32" => self.make_OneofUint32_mut().merge_text(stream),
                            "oneof_nested_message" => {
                                self.make_OneofNestedMessage_mut().merge_text(stream)
                            }
                            "oneof_string" => self.make_OneofString_mut().merge_text(stream),
                            "oneof_bytes" => self.make_OneofBytes_mut().merge_text(stream),
                            "oneof_bool" => self.make_OneofBool_mut().merge_text(stream),
                            "oneof_uint64" => self.make_OneofUint64_mut().merge_text(stream),
                            "oneof_float" => self.make_OneofFloat_mut().merge_text(stream),
                            "oneof_double" => self.make_OneofDouble_mut().merge_text(stream),
                            "oneof_enum" => self.make_OneofEnum_mut().merge_text(stream),
                            "oneof_null_value" => self.make_OneofNullValue_mut().merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        match self {
                            Self::OneofUint32(v) => stream.emit_field("oneof_uint32", v),
                            Self::OneofNestedMessage(v) => {
                                stream.emit_field("oneof_nested_message", v)
                            }
                            Self::OneofString(v) => stream.emit_field("oneof_string", v),
                            Self::OneofBytes(v) => stream.emit_field("oneof_bytes", v),
                            Self::OneofBool(v) => stream.emit_field("oneof_bool", v),
                            Self::OneofUint64(v) => stream.emit_field("oneof_uint64", v),
                            Self::OneofFloat(v) => stream.emit_field("oneof_float", v),
                            Self::OneofDouble(v) => stream.emit_field("oneof_double", v),
                            Self::OneofEnum(v) => stream.emit_field("oneof_enum", v),
                            Self::OneofNullValue(v) => stream.emit_field("oneof_null_value", v),
                        }
                    }
                }

                impl Default for TestAllTypesProto3OneOfOneofField {
                    fn default() -> Self {
                        Self::OneofUint32(Default::default())
                    }
                }

                pub struct TestAllTypesProto3 {
                    #[field(1u32, "optional_int32", varint, singular)]
                    pub optional_int32: i32,
                    #[field(2u32, "optional_int64", varint, singular)]
                    pub optional_int64: i64,
                    #[field(3u32, "optional_uint32", varint, singular)]
                    pub optional_uint32: u32,
                    #[field(4u32, "optional_uint64", varint, singular)]
                    pub optional_uint64: u64,
                    #[field(5u32, "optional_sint32", sigint, singular)]
                    pub optional_sint32: i32,
                    #[field(6u32, "optional_sint64", sigint, singular)]
                    pub optional_sint64: i64,
                    #[field(7u32, "optional_fixed32", fixed32, singular)]
                    pub optional_fixed32: u32,
                    #[field(8u32, "optional_fixed64", fixed64, singular)]
                    pub optional_fixed64: u64,
                    #[field(9u32, "optional_sfixed32", fixed32, singular)]
                    pub optional_sfixed32: i32,
                    #[field(10u32, "optional_sfixed64", fixed64, singular)]
                    pub optional_sfixed64: i64,
                    #[field(11u32, "optional_float", fixed32, singular)]
                    pub optional_float: f32,
                    #[field(12u32, "optional_double", fixed64, singular)]
                    pub optional_double: f64,
                    #[field(13u32, "optional_bool", bool, singular)]
                    pub optional_bool: bool,
                    #[field(14u32, "optional_string", string, singular)]
                    pub optional_string: String,
                    #[field(15u32, "optional_bytes", bytes, singular)]
                    pub optional_bytes: Vec<u8>,
                    #[field(18u32, "optional_nested_message", nested, optional)]
                    pub optional_nested_message: Option<Box<TestAllTypesProto3NestedMessage>>,
                    #[field(19u32, "optional_foreign_message", nested, optional)]
                    pub optional_foreign_message: Option<Box<ForeignMessage>>,
                    #[field(21u32, "optional_nested_enum", protoenum, singular)]
                    pub optional_nested_enum: TestAllTypesProto3NestedEnum,
                    #[field(22u32, "optional_foreign_enum", protoenum, singular)]
                    pub optional_foreign_enum: ForeignEnum,
                    #[field(23u32, "optional_aliased_enum", protoenum, singular)]
                    pub optional_aliased_enum: TestAllTypesProto3AliasedEnum,
                    #[field(24u32, "optional_string_piece", string, singular)]
                    pub optional_string_piece: String,
                    #[field(25u32, "optional_cord", string, singular)]
                    pub optional_cord: String,
                    #[field(27u32, "recursive_message", nested, optional)]
                    pub recursive_message: Option<Box<TestAllTypesProto3>>,
                    #[field(31u32, "repeated_int32", varint, packed)]
                    pub repeated_int32: Vec<i32>,
                    #[field(32u32, "repeated_int64", varint, packed)]
                    pub repeated_int64: Vec<i64>,
                    #[field(33u32, "repeated_uint32", varint, packed)]
                    pub repeated_uint32: Vec<u32>,
                    #[field(34u32, "repeated_uint64", varint, packed)]
                    pub repeated_uint64: Vec<u64>,
                    #[field(35u32, "repeated_sint32", sigint, packed)]
                    pub repeated_sint32: Vec<i32>,
                    #[field(36u32, "repeated_sint64", sigint, packed)]
                    pub repeated_sint64: Vec<i64>,
                    #[field(37u32, "repeated_fixed32", fixed32, packed)]
                    pub repeated_fixed32: Vec<u32>,
                    #[field(38u32, "repeated_fixed64", fixed64, packed)]
                    pub repeated_fixed64: Vec<u64>,
                    #[field(39u32, "repeated_sfixed32", fixed32, packed)]
                    pub repeated_sfixed32: Vec<i32>,
                    #[field(40u32, "repeated_sfixed64", fixed64, packed)]
                    pub repeated_sfixed64: Vec<i64>,
                    #[field(41u32, "repeated_float", fixed32, packed)]
                    pub repeated_float: Vec<f32>,
                    #[field(42u32, "repeated_double", fixed64, packed)]
                    pub repeated_double: Vec<f64>,
                    #[field(43u32, "repeated_bool", bool, packed)]
                    pub repeated_bool: Vec<bool>,
                    #[field(44u32, "repeated_string", string, repeated)]
                    pub repeated_string: Vec<String>,
                    #[field(45u32, "repeated_bytes", bytes, repeated)]
                    pub repeated_bytes: Vec<Vec<u8>>,
                    #[field(48u32, "repeated_nested_message", nested, repeated)]
                    pub repeated_nested_message: Vec<TestAllTypesProto3NestedMessage>,
                    #[field(49u32, "repeated_foreign_message", nested, repeated)]
                    pub repeated_foreign_message: Vec<ForeignMessage>,
                    #[field(51u32, "repeated_nested_enum", protoenum, packed)]
                    pub repeated_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
                    #[field(52u32, "repeated_foreign_enum", protoenum, packed)]
                    pub repeated_foreign_enum: Vec<ForeignEnum>,
                    #[field(54u32, "repeated_string_piece", string, repeated)]
                    pub repeated_string_piece: Vec<String>,
                    #[field(55u32, "repeated_cord", string, repeated)]
                    pub repeated_cord: Vec<String>,
                    #[field(75u32, "packed_int32", varint, packed)]
                    pub packed_int32: Vec<i32>,
                    #[field(76u32, "packed_int64", varint, packed)]
                    pub packed_int64: Vec<i64>,
                    #[field(77u32, "packed_uint32", varint, packed)]
                    pub packed_uint32: Vec<u32>,
                    #[field(78u32, "packed_uint64", varint, packed)]
                    pub packed_uint64: Vec<u64>,
                    #[field(79u32, "packed_sint32", sigint, packed)]
                    pub packed_sint32: Vec<i32>,
                    #[field(80u32, "packed_sint64", sigint, packed)]
                    pub packed_sint64: Vec<i64>,
                    #[field(81u32, "packed_fixed32", fixed32, packed)]
                    pub packed_fixed32: Vec<u32>,
                    #[field(82u32, "packed_fixed64", fixed64, packed)]
                    pub packed_fixed64: Vec<u64>,
                    #[field(83u32, "packed_sfixed32", fixed32, packed)]
                    pub packed_sfixed32: Vec<i32>,
                    #[field(84u32, "packed_sfixed64", fixed64, packed)]
                    pub packed_sfixed64: Vec<i64>,
                    #[field(85u32, "packed_float", fixed32, packed)]
                    pub packed_float: Vec<f32>,
                    #[field(86u32, "packed_double", fixed64, packed)]
                    pub packed_double: Vec<f64>,
                    #[field(87u32, "packed_bool", bool, packed)]
                    pub packed_bool: Vec<bool>,
                    #[field(88u32, "packed_nested_enum", protoenum, packed)]
                    pub packed_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
                    #[field(89u32, "unpacked_int32", varint, packed)]
                    pub unpacked_int32: Vec<i32>,
                    #[field(90u32, "unpacked_int64", varint, packed)]
                    pub unpacked_int64: Vec<i64>,
                    #[field(91u32, "unpacked_uint32", varint, packed)]
                    pub unpacked_uint32: Vec<u32>,
                    #[field(92u32, "unpacked_uint64", varint, packed)]
                    pub unpacked_uint64: Vec<u64>,
                    #[field(93u32, "unpacked_sint32", sigint, packed)]
                    pub unpacked_sint32: Vec<i32>,
                    #[field(94u32, "unpacked_sint64", sigint, packed)]
                    pub unpacked_sint64: Vec<i64>,
                    #[field(95u32, "unpacked_fixed32", fixed32, packed)]
                    pub unpacked_fixed32: Vec<u32>,
                    #[field(96u32, "unpacked_fixed64", fixed64, packed)]
                    pub unpacked_fixed64: Vec<u64>,
                    #[field(97u32, "unpacked_sfixed32", fixed32, packed)]
                    pub unpacked_sfixed32: Vec<i32>,
                    #[field(98u32, "unpacked_sfixed64", fixed64, packed)]
                    pub unpacked_sfixed64: Vec<i64>,
                    #[field(99u32, "unpacked_float", fixed32, packed)]
                    pub unpacked_float: Vec<f32>,
                    #[field(100u32, "unpacked_double", fixed64, packed)]
                    pub unpacked_double: Vec<f64>,
                    #[field(101u32, "unpacked_bool", bool, packed)]
                    pub unpacked_bool: Vec<bool>,
                    #[field(102u32, "unpacked_nested_enum", protoenum, packed)]
                    pub unpacked_nested_enum: Vec<TestAllTypesProto3NestedEnum>,
                    #[field(56u32, "map_int32_int32", map(varint, varint), singular)]
                    pub map_int32_int32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(57u32, "map_int64_int64", map(varint, varint), singular)]
                    pub map_int64_int64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(58u32, "map_uint32_uint32", map(varint, varint), singular)]
                    pub map_uint32_uint32: ::std::collections::BTreeMap<u32, u32>,
                    #[field(59u32, "map_uint64_uint64", map(varint, varint), singular)]
                    pub map_uint64_uint64: ::std::collections::BTreeMap<u64, u64>,
                    #[field(60u32, "map_sint32_sint32", map(sigint, sigint), singular)]
                    pub map_sint32_sint32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(61u32, "map_sint64_sint64", map(sigint, sigint), singular)]
                    pub map_sint64_sint64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(62u32, "map_fixed32_fixed32", map(fixed32, fixed32), singular)]
                    pub map_fixed32_fixed32: ::std::collections::BTreeMap<u32, u32>,
                    #[field(63u32, "map_fixed64_fixed64", map(fixed64, fixed64), singular)]
                    pub map_fixed64_fixed64: ::std::collections::BTreeMap<u64, u64>,
                    #[field(64u32, "map_sfixed32_sfixed32", map(fixed32, fixed32), singular)]
                    pub map_sfixed32_sfixed32: ::std::collections::BTreeMap<i32, i32>,
                    #[field(65u32, "map_sfixed64_sfixed64", map(fixed64, fixed64), singular)]
                    pub map_sfixed64_sfixed64: ::std::collections::BTreeMap<i64, i64>,
                    #[field(66u32, "map_int32_float", map(varint, fixed32), singular)]
                    pub map_int32_float: ::std::collections::BTreeMap<i32, f32>,
                    #[field(67u32, "map_int32_double", map(varint, fixed64), singular)]
                    pub map_int32_double: ::std::collections::BTreeMap<i32, f64>,
                    #[field(68u32, "map_bool_bool", map(bool, bool), singular)]
                    pub map_bool_bool: ::std::collections::BTreeMap<bool, bool>,
                    #[field(69u32, "map_string_string", map(string, string), singular)]
                    pub map_string_string: ::std::collections::BTreeMap<String, String>,
                    #[field(70u32, "map_string_bytes", map(string, bytes), singular)]
                    pub map_string_bytes: ::std::collections::BTreeMap<String, Vec<u8>>,
                    #[field(71u32, "map_string_nested_message", map(string, nested), singular)]
                    pub map_string_nested_message:
                    ::std::collections::BTreeMap<String, TestAllTypesProto3NestedMessage>,
                    #[field(72u32, "map_string_foreign_message", map(string, nested), singular)]
                    pub map_string_foreign_message:
                    ::std::collections::BTreeMap<String, ForeignMessage>,
                    #[field(73u32, "map_string_nested_enum", map(string, protoenum), singular)]
                    pub map_string_nested_enum:
                    ::std::collections::BTreeMap<String, TestAllTypesProto3NestedEnum>,
                    #[field(74u32, "map_string_foreign_enum", map(string, protoenum), singular)]
                    pub map_string_foreign_enum: ::std::collections::BTreeMap<String, ForeignEnum>,
                    #[field(201u32, "optional_bool_wrapper", nested, optional)]
                    pub optional_bool_wrapper: Option<Box<BoolValue>>,
                    #[field(202u32, "optional_int32_wrapper", nested, optional)]
                    pub optional_int32_wrapper: Option<Box<Int32Value>>,
                    #[field(203u32, "optional_int64_wrapper", nested, optional)]
                    pub optional_int64_wrapper: Option<Box<Int64Value>>,
                    #[field(204u32, "optional_uint32_wrapper", nested, optional)]
                    pub optional_uint32_wrapper: Option<Box<UInt32Value>>,
                    #[field(205u32, "optional_uint64_wrapper", nested, optional)]
                    pub optional_uint64_wrapper: Option<Box<UInt64Value>>,
                    #[field(206u32, "optional_float_wrapper", nested, optional)]
                    pub optional_float_wrapper: Option<Box<FloatValue>>,
                    #[field(207u32, "optional_double_wrapper", nested, optional)]
                    pub optional_double_wrapper: Option<Box<DoubleValue>>,
                    #[field(208u32, "optional_string_wrapper", nested, optional)]
                    pub optional_string_wrapper: Option<Box<StringValue>>,
                    #[field(209u32, "optional_bytes_wrapper", nested, optional)]
                    pub optional_bytes_wrapper: Option<Box<BytesValue>>,
                    #[field(211u32, "repeated_bool_wrapper", nested, repeated)]
                    pub repeated_bool_wrapper: Vec<BoolValue>,
                    #[field(212u32, "repeated_int32_wrapper", nested, repeated)]
                    pub repeated_int32_wrapper: Vec<Int32Value>,
                    #[field(213u32, "repeated_int64_wrapper", nested, repeated)]
                    pub repeated_int64_wrapper: Vec<Int64Value>,
                    #[field(214u32, "repeated_uint32_wrapper", nested, repeated)]
                    pub repeated_uint32_wrapper: Vec<UInt32Value>,
                    #[field(215u32, "repeated_uint64_wrapper", nested, repeated)]
                    pub repeated_uint64_wrapper: Vec<UInt64Value>,
                    #[field(216u32, "repeated_float_wrapper", nested, repeated)]
                    pub repeated_float_wrapper: Vec<FloatValue>,
                    #[field(217u32, "repeated_double_wrapper", nested, repeated)]
                    pub repeated_double_wrapper: Vec<DoubleValue>,
                    #[field(218u32, "repeated_string_wrapper", nested, repeated)]
                    pub repeated_string_wrapper: Vec<StringValue>,
                    #[field(219u32, "repeated_bytes_wrapper", nested, repeated)]
                    pub repeated_bytes_wrapper: Vec<BytesValue>,
                    #[field(301u32, "optional_duration", nested, optional)]
                    pub optional_duration: Option<Box<Duration>>,
                    #[field(302u32, "optional_timestamp", nested, optional)]
                    pub optional_timestamp: Option<Box<Timestamp>>,
                    #[field(303u32, "optional_field_mask", nested, optional)]
                    pub optional_field_mask: Option<Box<FieldMask>>,
                    #[field(304u32, "optional_struct", nested, optional)]
                    pub optional_struct: Option<Box<Struct>>,
                    #[field(305u32, "optional_any", nested, optional)]
                    pub optional_any: Option<Box<Any>>,
                    #[field(306u32, "optional_value", nested, optional)]
                    pub optional_value: Option<Box<Value>>,
                    #[field(307u32, "optional_null_value", protoenum, singular)]
                    pub optional_null_value: NullValue,
                    #[field(311u32, "repeated_duration", nested, repeated)]
                    pub repeated_duration: Vec<Duration>,
                    #[field(312u32, "repeated_timestamp", nested, repeated)]
                    pub repeated_timestamp: Vec<Timestamp>,
                    #[field(313u32, "repeated_fieldmask", nested, repeated)]
                    pub repeated_fieldmask: Vec<FieldMask>,
                    #[field(324u32, "repeated_struct", nested, repeated)]
                    pub repeated_struct: Vec<Struct>,
                    #[field(315u32, "repeated_any", nested, repeated)]
                    pub repeated_any: Vec<Any>,
                    #[field(316u32, "repeated_value", nested, repeated)]
                    pub repeated_value: Vec<Value>,
                    #[field(317u32, "repeated_list_value", nested, repeated)]
                    pub repeated_list_value: Vec<ListValue>,
                    #[field(401u32, "fieldname1", varint, singular)]
                    pub fieldname1: i32,
                    #[field(402u32, "field_name2", varint, singular)]
                    pub field_name2: i32,
                    #[field(403u32, "_field_name3", varint, singular)]
                    pub _field_name3: i32,
                    #[field(404u32, "field__name4_", varint, singular)]
                    pub field__name4_: i32,
                    #[field(405u32, "field0name5", varint, singular)]
                    pub field0name5: i32,
                    #[field(406u32, "field_0_name6", varint, singular)]
                    pub field_0_name6: i32,
                    #[field(407u32, "fieldName7", varint, singular)]
                    pub fieldName7: i32,
                    #[field(408u32, "FieldName8", varint, singular)]
                    pub FieldName8: i32,
                    #[field(409u32, "field_Name9", varint, singular)]
                    pub field_Name9: i32,
                    #[field(410u32, "Field_Name10", varint, singular)]
                    pub Field_Name10: i32,
                    #[field(411u32, "FIELD_NAME11", varint, singular)]
                    pub FIELD_NAME11: i32,
                    #[field(412u32, "FIELD_name12", varint, singular)]
                    pub FIELD_name12: i32,
                    #[field(413u32, "__field_name13", varint, singular)]
                    pub __field_name13: i32,
                    #[field(414u32, "__Field_name14", varint, singular)]
                    pub __Field_name14: i32,
                    #[field(415u32, "field__name15", varint, singular)]
                    pub field__name15: i32,
                    #[field(416u32, "field__Name16", varint, singular)]
                    pub field__Name16: i32,
                    #[field(417u32, "field_name17__", varint, singular)]
                    pub field_name17__: i32,
                    #[field(418u32, "Field_name18__", varint, singular)]
                    pub Field_name18__: i32,
                    #[oneof([111u32, 112u32, 113u32, 114u32, 115u32, 116u32, 117u32, 118u32, 119u32, 120u32,], ["oneof_uint32", "oneof_nested_message", "oneof_string", "oneof_bytes", "oneof_bool", "oneof_uint64", "oneof_float", "oneof_double", "oneof_enum", "oneof_null_value", ])]
                    pub oneof_field: Option<TestAllTypesProto3OneOfOneofField>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto3 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let names: &'static _ = &[
                            "optional_int32",
                            "optional_int64",
                            "optional_uint32",
                            "optional_uint64",
                            "optional_sint32",
                            "optional_sint64",
                            "optional_fixed32",
                            "optional_fixed64",
                            "optional_sfixed32",
                            "optional_sfixed64",
                            "optional_float",
                            "optional_double",
                            "optional_bool",
                            "optional_string",
                            "optional_bytes",
                            "optional_nested_message",
                            "optional_foreign_message",
                            "optional_nested_enum",
                            "optional_foreign_enum",
                            "optional_aliased_enum",
                            "optional_string_piece",
                            "optional_cord",
                            "recursive_message",
                            "repeated_int32",
                            "repeated_int64",
                            "repeated_uint32",
                            "repeated_uint64",
                            "repeated_sint32",
                            "repeated_sint64",
                            "repeated_fixed32",
                            "repeated_fixed64",
                            "repeated_sfixed32",
                            "repeated_sfixed64",
                            "repeated_float",
                            "repeated_double",
                            "repeated_bool",
                            "repeated_string",
                            "repeated_bytes",
                            "repeated_nested_message",
                            "repeated_foreign_message",
                            "repeated_nested_enum",
                            "repeated_foreign_enum",
                            "repeated_string_piece",
                            "repeated_cord",
                            "packed_int32",
                            "packed_int64",
                            "packed_uint32",
                            "packed_uint64",
                            "packed_sint32",
                            "packed_sint64",
                            "packed_fixed32",
                            "packed_fixed64",
                            "packed_sfixed32",
                            "packed_sfixed64",
                            "packed_float",
                            "packed_double",
                            "packed_bool",
                            "packed_nested_enum",
                            "unpacked_int32",
                            "unpacked_int64",
                            "unpacked_uint32",
                            "unpacked_uint64",
                            "unpacked_sint32",
                            "unpacked_sint64",
                            "unpacked_fixed32",
                            "unpacked_fixed64",
                            "unpacked_sfixed32",
                            "unpacked_sfixed64",
                            "unpacked_float",
                            "unpacked_double",
                            "unpacked_bool",
                            "unpacked_nested_enum",
                            "map_int32_int32",
                            "map_int64_int64",
                            "map_uint32_uint32",
                            "map_uint64_uint64",
                            "map_sint32_sint32",
                            "map_sint64_sint64",
                            "map_fixed32_fixed32",
                            "map_fixed64_fixed64",
                            "map_sfixed32_sfixed32",
                            "map_sfixed64_sfixed64",
                            "map_int32_float",
                            "map_int32_double",
                            "map_bool_bool",
                            "map_string_string",
                            "map_string_bytes",
                            "map_string_nested_message",
                            "map_string_foreign_message",
                            "map_string_nested_enum",
                            "map_string_foreign_enum",
                            "optional_bool_wrapper",
                            "optional_int32_wrapper",
                            "optional_int64_wrapper",
                            "optional_uint32_wrapper",
                            "optional_uint64_wrapper",
                            "optional_float_wrapper",
                            "optional_double_wrapper",
                            "optional_string_wrapper",
                            "optional_bytes_wrapper",
                            "repeated_bool_wrapper",
                            "repeated_int32_wrapper",
                            "repeated_int64_wrapper",
                            "repeated_uint32_wrapper",
                            "repeated_uint64_wrapper",
                            "repeated_float_wrapper",
                            "repeated_double_wrapper",
                            "repeated_string_wrapper",
                            "repeated_bytes_wrapper",
                            "optional_duration",
                            "optional_timestamp",
                            "optional_field_mask",
                            "optional_struct",
                            "optional_any",
                            "optional_value",
                            "optional_null_value",
                            "repeated_duration",
                            "repeated_timestamp",
                            "repeated_fieldmask",
                            "repeated_struct",
                            "repeated_any",
                            "repeated_value",
                            "repeated_list_value",
                            "fieldname1",
                            "field_name2",
                            "_field_name3",
                            "field__name4_",
                            "field0name5",
                            "field_0_name6",
                            "fieldName7",
                            "FieldName8",
                            "field_Name9",
                            "Field_Name10",
                            "FIELD_NAME11",
                            "FIELD_name12",
                            "__field_name13",
                            "__Field_name14",
                            "field__name15",
                            "field__Name16",
                            "field_name17__",
                            "Field_name18__",
                            "oneof_field",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &&self.optional_int32,
                            &&self.optional_int64,
                            &&self.optional_uint32,
                            &&self.optional_uint64,
                            &&self.optional_sint32,
                            &&self.optional_sint64,
                            &&self.optional_fixed32,
                            &&self.optional_fixed64,
                            &&self.optional_sfixed32,
                            &&self.optional_sfixed64,
                            &&self.optional_float,
                            &&self.optional_double,
                            &&self.optional_bool,
                            &&self.optional_string,
                            &&self.optional_bytes,
                            &&self.optional_nested_message,
                            &&self.optional_foreign_message,
                            &&self.optional_nested_enum,
                            &&self.optional_foreign_enum,
                            &&self.optional_aliased_enum,
                            &&self.optional_string_piece,
                            &&self.optional_cord,
                            &&self.recursive_message,
                            &&self.repeated_int32,
                            &&self.repeated_int64,
                            &&self.repeated_uint32,
                            &&self.repeated_uint64,
                            &&self.repeated_sint32,
                            &&self.repeated_sint64,
                            &&self.repeated_fixed32,
                            &&self.repeated_fixed64,
                            &&self.repeated_sfixed32,
                            &&self.repeated_sfixed64,
                            &&self.repeated_float,
                            &&self.repeated_double,
                            &&self.repeated_bool,
                            &&self.repeated_string,
                            &&self.repeated_bytes,
                            &&self.repeated_nested_message,
                            &&self.repeated_foreign_message,
                            &&self.repeated_nested_enum,
                            &&self.repeated_foreign_enum,
                            &&self.repeated_string_piece,
                            &&self.repeated_cord,
                            &&self.packed_int32,
                            &&self.packed_int64,
                            &&self.packed_uint32,
                            &&self.packed_uint64,
                            &&self.packed_sint32,
                            &&self.packed_sint64,
                            &&self.packed_fixed32,
                            &&self.packed_fixed64,
                            &&self.packed_sfixed32,
                            &&self.packed_sfixed64,
                            &&self.packed_float,
                            &&self.packed_double,
                            &&self.packed_bool,
                            &&self.packed_nested_enum,
                            &&self.unpacked_int32,
                            &&self.unpacked_int64,
                            &&self.unpacked_uint32,
                            &&self.unpacked_uint64,
                            &&self.unpacked_sint32,
                            &&self.unpacked_sint64,
                            &&self.unpacked_fixed32,
                            &&self.unpacked_fixed64,
                            &&self.unpacked_sfixed32,
                            &&self.unpacked_sfixed64,
                            &&self.unpacked_float,
                            &&self.unpacked_double,
                            &&self.unpacked_bool,
                            &&self.unpacked_nested_enum,
                            &&self.map_int32_int32,
                            &&self.map_int64_int64,
                            &&self.map_uint32_uint32,
                            &&self.map_uint64_uint64,
                            &&self.map_sint32_sint32,
                            &&self.map_sint64_sint64,
                            &&self.map_fixed32_fixed32,
                            &&self.map_fixed64_fixed64,
                            &&self.map_sfixed32_sfixed32,
                            &&self.map_sfixed64_sfixed64,
                            &&self.map_int32_float,
                            &&self.map_int32_double,
                            &&self.map_bool_bool,
                            &&self.map_string_string,
                            &&self.map_string_bytes,
                            &&self.map_string_nested_message,
                            &&self.map_string_foreign_message,
                            &&self.map_string_nested_enum,
                            &&self.map_string_foreign_enum,
                            &&self.optional_bool_wrapper,
                            &&self.optional_int32_wrapper,
                            &&self.optional_int64_wrapper,
                            &&self.optional_uint32_wrapper,
                            &&self.optional_uint64_wrapper,
                            &&self.optional_float_wrapper,
                            &&self.optional_double_wrapper,
                            &&self.optional_string_wrapper,
                            &&self.optional_bytes_wrapper,
                            &&self.repeated_bool_wrapper,
                            &&self.repeated_int32_wrapper,
                            &&self.repeated_int64_wrapper,
                            &&self.repeated_uint32_wrapper,
                            &&self.repeated_uint64_wrapper,
                            &&self.repeated_float_wrapper,
                            &&self.repeated_double_wrapper,
                            &&self.repeated_string_wrapper,
                            &&self.repeated_bytes_wrapper,
                            &&self.optional_duration,
                            &&self.optional_timestamp,
                            &&self.optional_field_mask,
                            &&self.optional_struct,
                            &&self.optional_any,
                            &&self.optional_value,
                            &&self.optional_null_value,
                            &&self.repeated_duration,
                            &&self.repeated_timestamp,
                            &&self.repeated_fieldmask,
                            &&self.repeated_struct,
                            &&self.repeated_any,
                            &&self.repeated_value,
                            &&self.repeated_list_value,
                            &&self.fieldname1,
                            &&self.field_name2,
                            &&self._field_name3,
                            &&self.field__name4_,
                            &&self.field0name5,
                            &&self.field_0_name6,
                            &&self.fieldName7,
                            &&self.FieldName8,
                            &&self.field_Name9,
                            &&self.Field_Name10,
                            &&self.FIELD_NAME11,
                            &&self.FIELD_name12,
                            &&self.__field_name13,
                            &&self.__Field_name14,
                            &&self.field__name15,
                            &&self.field__Name16,
                            &&self.field_name17__,
                            &&self.Field_name18__,
                            &&self.oneof_field,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "TestAllTypesProto3",
                            names,
                            values,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto3 {
                    #[inline]
                    fn default() -> TestAllTypesProto3 {
                        TestAllTypesProto3 {
                            optional_int32: ::core::default::Default::default(),
                            optional_int64: ::core::default::Default::default(),
                            optional_uint32: ::core::default::Default::default(),
                            optional_uint64: ::core::default::Default::default(),
                            optional_sint32: ::core::default::Default::default(),
                            optional_sint64: ::core::default::Default::default(),
                            optional_fixed32: ::core::default::Default::default(),
                            optional_fixed64: ::core::default::Default::default(),
                            optional_sfixed32: ::core::default::Default::default(),
                            optional_sfixed64: ::core::default::Default::default(),
                            optional_float: ::core::default::Default::default(),
                            optional_double: ::core::default::Default::default(),
                            optional_bool: ::core::default::Default::default(),
                            optional_string: ::core::default::Default::default(),
                            optional_bytes: ::core::default::Default::default(),
                            optional_nested_message: ::core::default::Default::default(),
                            optional_foreign_message: ::core::default::Default::default(),
                            optional_nested_enum: ::core::default::Default::default(),
                            optional_foreign_enum: ::core::default::Default::default(),
                            optional_aliased_enum: ::core::default::Default::default(),
                            optional_string_piece: ::core::default::Default::default(),
                            optional_cord: ::core::default::Default::default(),
                            recursive_message: ::core::default::Default::default(),
                            repeated_int32: ::core::default::Default::default(),
                            repeated_int64: ::core::default::Default::default(),
                            repeated_uint32: ::core::default::Default::default(),
                            repeated_uint64: ::core::default::Default::default(),
                            repeated_sint32: ::core::default::Default::default(),
                            repeated_sint64: ::core::default::Default::default(),
                            repeated_fixed32: ::core::default::Default::default(),
                            repeated_fixed64: ::core::default::Default::default(),
                            repeated_sfixed32: ::core::default::Default::default(),
                            repeated_sfixed64: ::core::default::Default::default(),
                            repeated_float: ::core::default::Default::default(),
                            repeated_double: ::core::default::Default::default(),
                            repeated_bool: ::core::default::Default::default(),
                            repeated_string: ::core::default::Default::default(),
                            repeated_bytes: ::core::default::Default::default(),
                            repeated_nested_message: ::core::default::Default::default(),
                            repeated_foreign_message: ::core::default::Default::default(),
                            repeated_nested_enum: ::core::default::Default::default(),
                            repeated_foreign_enum: ::core::default::Default::default(),
                            repeated_string_piece: ::core::default::Default::default(),
                            repeated_cord: ::core::default::Default::default(),
                            packed_int32: ::core::default::Default::default(),
                            packed_int64: ::core::default::Default::default(),
                            packed_uint32: ::core::default::Default::default(),
                            packed_uint64: ::core::default::Default::default(),
                            packed_sint32: ::core::default::Default::default(),
                            packed_sint64: ::core::default::Default::default(),
                            packed_fixed32: ::core::default::Default::default(),
                            packed_fixed64: ::core::default::Default::default(),
                            packed_sfixed32: ::core::default::Default::default(),
                            packed_sfixed64: ::core::default::Default::default(),
                            packed_float: ::core::default::Default::default(),
                            packed_double: ::core::default::Default::default(),
                            packed_bool: ::core::default::Default::default(),
                            packed_nested_enum: ::core::default::Default::default(),
                            unpacked_int32: ::core::default::Default::default(),
                            unpacked_int64: ::core::default::Default::default(),
                            unpacked_uint32: ::core::default::Default::default(),
                            unpacked_uint64: ::core::default::Default::default(),
                            unpacked_sint32: ::core::default::Default::default(),
                            unpacked_sint64: ::core::default::Default::default(),
                            unpacked_fixed32: ::core::default::Default::default(),
                            unpacked_fixed64: ::core::default::Default::default(),
                            unpacked_sfixed32: ::core::default::Default::default(),
                            unpacked_sfixed64: ::core::default::Default::default(),
                            unpacked_float: ::core::default::Default::default(),
                            unpacked_double: ::core::default::Default::default(),
                            unpacked_bool: ::core::default::Default::default(),
                            unpacked_nested_enum: ::core::default::Default::default(),
                            map_int32_int32: ::core::default::Default::default(),
                            map_int64_int64: ::core::default::Default::default(),
                            map_uint32_uint32: ::core::default::Default::default(),
                            map_uint64_uint64: ::core::default::Default::default(),
                            map_sint32_sint32: ::core::default::Default::default(),
                            map_sint64_sint64: ::core::default::Default::default(),
                            map_fixed32_fixed32: ::core::default::Default::default(),
                            map_fixed64_fixed64: ::core::default::Default::default(),
                            map_sfixed32_sfixed32: ::core::default::Default::default(),
                            map_sfixed64_sfixed64: ::core::default::Default::default(),
                            map_int32_float: ::core::default::Default::default(),
                            map_int32_double: ::core::default::Default::default(),
                            map_bool_bool: ::core::default::Default::default(),
                            map_string_string: ::core::default::Default::default(),
                            map_string_bytes: ::core::default::Default::default(),
                            map_string_nested_message: ::core::default::Default::default(),
                            map_string_foreign_message: ::core::default::Default::default(),
                            map_string_nested_enum: ::core::default::Default::default(),
                            map_string_foreign_enum: ::core::default::Default::default(),
                            optional_bool_wrapper: ::core::default::Default::default(),
                            optional_int32_wrapper: ::core::default::Default::default(),
                            optional_int64_wrapper: ::core::default::Default::default(),
                            optional_uint32_wrapper: ::core::default::Default::default(),
                            optional_uint64_wrapper: ::core::default::Default::default(),
                            optional_float_wrapper: ::core::default::Default::default(),
                            optional_double_wrapper: ::core::default::Default::default(),
                            optional_string_wrapper: ::core::default::Default::default(),
                            optional_bytes_wrapper: ::core::default::Default::default(),
                            repeated_bool_wrapper: ::core::default::Default::default(),
                            repeated_int32_wrapper: ::core::default::Default::default(),
                            repeated_int64_wrapper: ::core::default::Default::default(),
                            repeated_uint32_wrapper: ::core::default::Default::default(),
                            repeated_uint64_wrapper: ::core::default::Default::default(),
                            repeated_float_wrapper: ::core::default::Default::default(),
                            repeated_double_wrapper: ::core::default::Default::default(),
                            repeated_string_wrapper: ::core::default::Default::default(),
                            repeated_bytes_wrapper: ::core::default::Default::default(),
                            optional_duration: ::core::default::Default::default(),
                            optional_timestamp: ::core::default::Default::default(),
                            optional_field_mask: ::core::default::Default::default(),
                            optional_struct: ::core::default::Default::default(),
                            optional_any: ::core::default::Default::default(),
                            optional_value: ::core::default::Default::default(),
                            optional_null_value: ::core::default::Default::default(),
                            repeated_duration: ::core::default::Default::default(),
                            repeated_timestamp: ::core::default::Default::default(),
                            repeated_fieldmask: ::core::default::Default::default(),
                            repeated_struct: ::core::default::Default::default(),
                            repeated_any: ::core::default::Default::default(),
                            repeated_value: ::core::default::Default::default(),
                            repeated_list_value: ::core::default::Default::default(),
                            fieldname1: ::core::default::Default::default(),
                            field_name2: ::core::default::Default::default(),
                            _field_name3: ::core::default::Default::default(),
                            field__name4_: ::core::default::Default::default(),
                            field0name5: ::core::default::Default::default(),
                            field_0_name6: ::core::default::Default::default(),
                            fieldName7: ::core::default::Default::default(),
                            FieldName8: ::core::default::Default::default(),
                            field_Name9: ::core::default::Default::default(),
                            Field_Name10: ::core::default::Default::default(),
                            FIELD_NAME11: ::core::default::Default::default(),
                            FIELD_name12: ::core::default::Default::default(),
                            __field_name13: ::core::default::Default::default(),
                            __Field_name14: ::core::default::Default::default(),
                            field__name15: ::core::default::Default::default(),
                            field__Name16: ::core::default::Default::default(),
                            field_name17__: ::core::default::Default::default(),
                            Field_name18__: ::core::default::Default::default(),
                            oneof_field: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto3 {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto3 {
                        TestAllTypesProto3 {
                            optional_int32: ::core::clone::Clone::clone(&self.optional_int32),
                            optional_int64: ::core::clone::Clone::clone(&self.optional_int64),
                            optional_uint32: ::core::clone::Clone::clone(&self.optional_uint32),
                            optional_uint64: ::core::clone::Clone::clone(&self.optional_uint64),
                            optional_sint32: ::core::clone::Clone::clone(&self.optional_sint32),
                            optional_sint64: ::core::clone::Clone::clone(&self.optional_sint64),
                            optional_fixed32: ::core::clone::Clone::clone(&self.optional_fixed32),
                            optional_fixed64: ::core::clone::Clone::clone(&self.optional_fixed64),
                            optional_sfixed32: ::core::clone::Clone::clone(&self.optional_sfixed32),
                            optional_sfixed64: ::core::clone::Clone::clone(&self.optional_sfixed64),
                            optional_float: ::core::clone::Clone::clone(&self.optional_float),
                            optional_double: ::core::clone::Clone::clone(&self.optional_double),
                            optional_bool: ::core::clone::Clone::clone(&self.optional_bool),
                            optional_string: ::core::clone::Clone::clone(&self.optional_string),
                            optional_bytes: ::core::clone::Clone::clone(&self.optional_bytes),
                            optional_nested_message: ::core::clone::Clone::clone(
                                &self.optional_nested_message,
                            ),
                            optional_foreign_message: ::core::clone::Clone::clone(
                                &self.optional_foreign_message,
                            ),
                            optional_nested_enum: ::core::clone::Clone::clone(
                                &self.optional_nested_enum,
                            ),
                            optional_foreign_enum: ::core::clone::Clone::clone(
                                &self.optional_foreign_enum,
                            ),
                            optional_aliased_enum: ::core::clone::Clone::clone(
                                &self.optional_aliased_enum,
                            ),
                            optional_string_piece: ::core::clone::Clone::clone(
                                &self.optional_string_piece,
                            ),
                            optional_cord: ::core::clone::Clone::clone(&self.optional_cord),
                            recursive_message: ::core::clone::Clone::clone(&self.recursive_message),
                            repeated_int32: ::core::clone::Clone::clone(&self.repeated_int32),
                            repeated_int64: ::core::clone::Clone::clone(&self.repeated_int64),
                            repeated_uint32: ::core::clone::Clone::clone(&self.repeated_uint32),
                            repeated_uint64: ::core::clone::Clone::clone(&self.repeated_uint64),
                            repeated_sint32: ::core::clone::Clone::clone(&self.repeated_sint32),
                            repeated_sint64: ::core::clone::Clone::clone(&self.repeated_sint64),
                            repeated_fixed32: ::core::clone::Clone::clone(&self.repeated_fixed32),
                            repeated_fixed64: ::core::clone::Clone::clone(&self.repeated_fixed64),
                            repeated_sfixed32: ::core::clone::Clone::clone(&self.repeated_sfixed32),
                            repeated_sfixed64: ::core::clone::Clone::clone(&self.repeated_sfixed64),
                            repeated_float: ::core::clone::Clone::clone(&self.repeated_float),
                            repeated_double: ::core::clone::Clone::clone(&self.repeated_double),
                            repeated_bool: ::core::clone::Clone::clone(&self.repeated_bool),
                            repeated_string: ::core::clone::Clone::clone(&self.repeated_string),
                            repeated_bytes: ::core::clone::Clone::clone(&self.repeated_bytes),
                            repeated_nested_message: ::core::clone::Clone::clone(
                                &self.repeated_nested_message,
                            ),
                            repeated_foreign_message: ::core::clone::Clone::clone(
                                &self.repeated_foreign_message,
                            ),
                            repeated_nested_enum: ::core::clone::Clone::clone(
                                &self.repeated_nested_enum,
                            ),
                            repeated_foreign_enum: ::core::clone::Clone::clone(
                                &self.repeated_foreign_enum,
                            ),
                            repeated_string_piece: ::core::clone::Clone::clone(
                                &self.repeated_string_piece,
                            ),
                            repeated_cord: ::core::clone::Clone::clone(&self.repeated_cord),
                            packed_int32: ::core::clone::Clone::clone(&self.packed_int32),
                            packed_int64: ::core::clone::Clone::clone(&self.packed_int64),
                            packed_uint32: ::core::clone::Clone::clone(&self.packed_uint32),
                            packed_uint64: ::core::clone::Clone::clone(&self.packed_uint64),
                            packed_sint32: ::core::clone::Clone::clone(&self.packed_sint32),
                            packed_sint64: ::core::clone::Clone::clone(&self.packed_sint64),
                            packed_fixed32: ::core::clone::Clone::clone(&self.packed_fixed32),
                            packed_fixed64: ::core::clone::Clone::clone(&self.packed_fixed64),
                            packed_sfixed32: ::core::clone::Clone::clone(&self.packed_sfixed32),
                            packed_sfixed64: ::core::clone::Clone::clone(&self.packed_sfixed64),
                            packed_float: ::core::clone::Clone::clone(&self.packed_float),
                            packed_double: ::core::clone::Clone::clone(&self.packed_double),
                            packed_bool: ::core::clone::Clone::clone(&self.packed_bool),
                            packed_nested_enum: ::core::clone::Clone::clone(
                                &self.packed_nested_enum,
                            ),
                            unpacked_int32: ::core::clone::Clone::clone(&self.unpacked_int32),
                            unpacked_int64: ::core::clone::Clone::clone(&self.unpacked_int64),
                            unpacked_uint32: ::core::clone::Clone::clone(&self.unpacked_uint32),
                            unpacked_uint64: ::core::clone::Clone::clone(&self.unpacked_uint64),
                            unpacked_sint32: ::core::clone::Clone::clone(&self.unpacked_sint32),
                            unpacked_sint64: ::core::clone::Clone::clone(&self.unpacked_sint64),
                            unpacked_fixed32: ::core::clone::Clone::clone(&self.unpacked_fixed32),
                            unpacked_fixed64: ::core::clone::Clone::clone(&self.unpacked_fixed64),
                            unpacked_sfixed32: ::core::clone::Clone::clone(&self.unpacked_sfixed32),
                            unpacked_sfixed64: ::core::clone::Clone::clone(&self.unpacked_sfixed64),
                            unpacked_float: ::core::clone::Clone::clone(&self.unpacked_float),
                            unpacked_double: ::core::clone::Clone::clone(&self.unpacked_double),
                            unpacked_bool: ::core::clone::Clone::clone(&self.unpacked_bool),
                            unpacked_nested_enum: ::core::clone::Clone::clone(
                                &self.unpacked_nested_enum,
                            ),
                            map_int32_int32: ::core::clone::Clone::clone(&self.map_int32_int32),
                            map_int64_int64: ::core::clone::Clone::clone(&self.map_int64_int64),
                            map_uint32_uint32: ::core::clone::Clone::clone(&self.map_uint32_uint32),
                            map_uint64_uint64: ::core::clone::Clone::clone(&self.map_uint64_uint64),
                            map_sint32_sint32: ::core::clone::Clone::clone(&self.map_sint32_sint32),
                            map_sint64_sint64: ::core::clone::Clone::clone(&self.map_sint64_sint64),
                            map_fixed32_fixed32: ::core::clone::Clone::clone(
                                &self.map_fixed32_fixed32,
                            ),
                            map_fixed64_fixed64: ::core::clone::Clone::clone(
                                &self.map_fixed64_fixed64,
                            ),
                            map_sfixed32_sfixed32: ::core::clone::Clone::clone(
                                &self.map_sfixed32_sfixed32,
                            ),
                            map_sfixed64_sfixed64: ::core::clone::Clone::clone(
                                &self.map_sfixed64_sfixed64,
                            ),
                            map_int32_float: ::core::clone::Clone::clone(&self.map_int32_float),
                            map_int32_double: ::core::clone::Clone::clone(&self.map_int32_double),
                            map_bool_bool: ::core::clone::Clone::clone(&self.map_bool_bool),
                            map_string_string: ::core::clone::Clone::clone(&self.map_string_string),
                            map_string_bytes: ::core::clone::Clone::clone(&self.map_string_bytes),
                            map_string_nested_message: ::core::clone::Clone::clone(
                                &self.map_string_nested_message,
                            ),
                            map_string_foreign_message: ::core::clone::Clone::clone(
                                &self.map_string_foreign_message,
                            ),
                            map_string_nested_enum: ::core::clone::Clone::clone(
                                &self.map_string_nested_enum,
                            ),
                            map_string_foreign_enum: ::core::clone::Clone::clone(
                                &self.map_string_foreign_enum,
                            ),
                            optional_bool_wrapper: ::core::clone::Clone::clone(
                                &self.optional_bool_wrapper,
                            ),
                            optional_int32_wrapper: ::core::clone::Clone::clone(
                                &self.optional_int32_wrapper,
                            ),
                            optional_int64_wrapper: ::core::clone::Clone::clone(
                                &self.optional_int64_wrapper,
                            ),
                            optional_uint32_wrapper: ::core::clone::Clone::clone(
                                &self.optional_uint32_wrapper,
                            ),
                            optional_uint64_wrapper: ::core::clone::Clone::clone(
                                &self.optional_uint64_wrapper,
                            ),
                            optional_float_wrapper: ::core::clone::Clone::clone(
                                &self.optional_float_wrapper,
                            ),
                            optional_double_wrapper: ::core::clone::Clone::clone(
                                &self.optional_double_wrapper,
                            ),
                            optional_string_wrapper: ::core::clone::Clone::clone(
                                &self.optional_string_wrapper,
                            ),
                            optional_bytes_wrapper: ::core::clone::Clone::clone(
                                &self.optional_bytes_wrapper,
                            ),
                            repeated_bool_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_bool_wrapper,
                            ),
                            repeated_int32_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_int32_wrapper,
                            ),
                            repeated_int64_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_int64_wrapper,
                            ),
                            repeated_uint32_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_uint32_wrapper,
                            ),
                            repeated_uint64_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_uint64_wrapper,
                            ),
                            repeated_float_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_float_wrapper,
                            ),
                            repeated_double_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_double_wrapper,
                            ),
                            repeated_string_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_string_wrapper,
                            ),
                            repeated_bytes_wrapper: ::core::clone::Clone::clone(
                                &self.repeated_bytes_wrapper,
                            ),
                            optional_duration: ::core::clone::Clone::clone(&self.optional_duration),
                            optional_timestamp: ::core::clone::Clone::clone(
                                &self.optional_timestamp,
                            ),
                            optional_field_mask: ::core::clone::Clone::clone(
                                &self.optional_field_mask,
                            ),
                            optional_struct: ::core::clone::Clone::clone(&self.optional_struct),
                            optional_any: ::core::clone::Clone::clone(&self.optional_any),
                            optional_value: ::core::clone::Clone::clone(&self.optional_value),
                            optional_null_value: ::core::clone::Clone::clone(
                                &self.optional_null_value,
                            ),
                            repeated_duration: ::core::clone::Clone::clone(&self.repeated_duration),
                            repeated_timestamp: ::core::clone::Clone::clone(
                                &self.repeated_timestamp,
                            ),
                            repeated_fieldmask: ::core::clone::Clone::clone(
                                &self.repeated_fieldmask,
                            ),
                            repeated_struct: ::core::clone::Clone::clone(&self.repeated_struct),
                            repeated_any: ::core::clone::Clone::clone(&self.repeated_any),
                            repeated_value: ::core::clone::Clone::clone(&self.repeated_value),
                            repeated_list_value: ::core::clone::Clone::clone(
                                &self.repeated_list_value,
                            ),
                            fieldname1: ::core::clone::Clone::clone(&self.fieldname1),
                            field_name2: ::core::clone::Clone::clone(&self.field_name2),
                            _field_name3: ::core::clone::Clone::clone(&self._field_name3),
                            field__name4_: ::core::clone::Clone::clone(&self.field__name4_),
                            field0name5: ::core::clone::Clone::clone(&self.field0name5),
                            field_0_name6: ::core::clone::Clone::clone(&self.field_0_name6),
                            fieldName7: ::core::clone::Clone::clone(&self.fieldName7),
                            FieldName8: ::core::clone::Clone::clone(&self.FieldName8),
                            field_Name9: ::core::clone::Clone::clone(&self.field_Name9),
                            Field_Name10: ::core::clone::Clone::clone(&self.Field_Name10),
                            FIELD_NAME11: ::core::clone::Clone::clone(&self.FIELD_NAME11),
                            FIELD_name12: ::core::clone::Clone::clone(&self.FIELD_name12),
                            __field_name13: ::core::clone::Clone::clone(&self.__field_name13),
                            __Field_name14: ::core::clone::Clone::clone(&self.__Field_name14),
                            field__name15: ::core::clone::Clone::clone(&self.field__name15),
                            field__Name16: ::core::clone::Clone::clone(&self.field__Name16),
                            field_name17__: ::core::clone::Clone::clone(&self.field_name17__),
                            Field_name18__: ::core::clone::Clone::clone(&self.Field_name18__),
                            oneof_field: ::core::clone::Clone::clone(&self.oneof_field),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto3 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto3 {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto3) -> bool {
                        self.optional_int32 == other.optional_int32
                            & &self.optional_int64 == other.optional_int64
                            & &self.optional_uint32 == other.optional_uint32
                            & &self.optional_uint64 == other.optional_uint64
                            & &self.optional_sint32 == other.optional_sint32
                            & &self.optional_sint64 == other.optional_sint64
                            & &self.optional_fixed32 == other.optional_fixed32
                            & &self.optional_fixed64 == other.optional_fixed64
                            & &self.optional_sfixed32 == other.optional_sfixed32
                            && self.optional_sfixed64 == other.optional_sfixed64
                            & &self.optional_float == other.optional_float
                            & &self.optional_double == other.optional_double
                            & &self.optional_bool == other.optional_bool
                            & &self.optional_string == other.optional_string
                            & &self.optional_bytes == other.optional_bytes
                            & &self.optional_nested_message == other.optional_nested_message
                            & &self.optional_foreign_message == other.optional_foreign_message
                            & &self.optional_nested_enum == other.optional_nested_enum
                            & &self.optional_foreign_enum == other.optional_foreign_enum
                            && self.optional_aliased_enum == other.optional_aliased_enum
                            & &self.optional_string_piece == other.optional_string_piece
                            & &self.optional_cord == other.optional_cord
                            & &self.recursive_message == other.recursive_message
                            & &self.repeated_int32 == other.repeated_int32
                            & &self.repeated_int64 == other.repeated_int64
                            & &self.repeated_uint32 == other.repeated_uint32
                            & &self.repeated_uint64 == other.repeated_uint64
                            & &self.repeated_sint32 == other.repeated_sint32
                            & &self.repeated_sint64 == other.repeated_sint64
                            && self.repeated_fixed32 == other.repeated_fixed32
                            & &self.repeated_fixed64 == other.repeated_fixed64
                            & &self.repeated_sfixed32 == other.repeated_sfixed32
                            & &self.repeated_sfixed64 == other.repeated_sfixed64
                            & &self.repeated_float == other.repeated_float
                            & &self.repeated_double == other.repeated_double
                            & &self.repeated_bool == other.repeated_bool
                            & &self.repeated_string == other.repeated_string
                            & &self.repeated_bytes == other.repeated_bytes
                            & &self.repeated_nested_message == other.repeated_nested_message
                            && self.repeated_foreign_message == other.repeated_foreign_message
                            & &self.repeated_nested_enum == other.repeated_nested_enum
                            & &self.repeated_foreign_enum == other.repeated_foreign_enum
                            & &self.repeated_string_piece == other.repeated_string_piece
                            & &self.repeated_cord == other.repeated_cord
                            & &self.packed_int32 == other.packed_int32
                            & &self.packed_int64 == other.packed_int64
                            & &self.packed_uint32 == other.packed_uint32
                            & &self.packed_uint64 == other.packed_uint64
                            & &self.packed_sint32 == other.packed_sint32
                            && self.packed_sint64 == other.packed_sint64
                            & &self.packed_fixed32 == other.packed_fixed32
                            & &self.packed_fixed64 == other.packed_fixed64
                            & &self.packed_sfixed32 == other.packed_sfixed32
                            & &self.packed_sfixed64 == other.packed_sfixed64
                            & &self.packed_float == other.packed_float
                            & &self.packed_double == other.packed_double
                            & &self.packed_bool == other.packed_bool
                            & &self.packed_nested_enum == other.packed_nested_enum
                            & &self.unpacked_int32 == other.unpacked_int32
                            && self.unpacked_int64 == other.unpacked_int64
                            & &self.unpacked_uint32 == other.unpacked_uint32
                            & &self.unpacked_uint64 == other.unpacked_uint64
                            & &self.unpacked_sint32 == other.unpacked_sint32
                            & &self.unpacked_sint64 == other.unpacked_sint64
                            & &self.unpacked_fixed32 == other.unpacked_fixed32
                            & &self.unpacked_fixed64 == other.unpacked_fixed64
                            & &self.unpacked_sfixed32 == other.unpacked_sfixed32
                            & &self.unpacked_sfixed64 == other.unpacked_sfixed64
                            & &self.unpacked_float == other.unpacked_float
                            && self.unpacked_double == other.unpacked_double
                            & &self.unpacked_bool == other.unpacked_bool
                            & &self.unpacked_nested_enum == other.unpacked_nested_enum
                            & &self.map_int32_int32 == other.map_int32_int32
                            & &self.map_int64_int64 == other.map_int64_int64
                            & &self.map_uint32_uint32 == other.map_uint32_uint32
                            & &self.map_uint64_uint64 == other.map_uint64_uint64
                            & &self.map_sint32_sint32 == other.map_sint32_sint32
                            & &self.map_sint64_sint64 == other.map_sint64_sint64
                            & &self.map_fixed32_fixed32 == other.map_fixed32_fixed32
                            && self.map_fixed64_fixed64 == other.map_fixed64_fixed64
                            & &self.map_sfixed32_sfixed32 == other.map_sfixed32_sfixed32
                            & &self.map_sfixed64_sfixed64 == other.map_sfixed64_sfixed64
                            & &self.map_int32_float == other.map_int32_float
                            & &self.map_int32_double == other.map_int32_double
                            & &self.map_bool_bool == other.map_bool_bool
                            & &self.map_string_string == other.map_string_string
                            & &self.map_string_bytes == other.map_string_bytes
                            & &self.map_string_nested_message == other.map_string_nested_message
                            & &self.map_string_foreign_message == other.map_string_foreign_message
                            && self.map_string_nested_enum == other.map_string_nested_enum
                            & &self.map_string_foreign_enum == other.map_string_foreign_enum
                            & &self.optional_bool_wrapper == other.optional_bool_wrapper
                            & &self.optional_int32_wrapper == other.optional_int32_wrapper
                            & &self.optional_int64_wrapper == other.optional_int64_wrapper
                            & &self.optional_uint32_wrapper == other.optional_uint32_wrapper
                            & &self.optional_uint64_wrapper == other.optional_uint64_wrapper
                            & &self.optional_float_wrapper == other.optional_float_wrapper
                            & &self.optional_double_wrapper == other.optional_double_wrapper
                            & &self.optional_string_wrapper == other.optional_string_wrapper
                            && self.optional_bytes_wrapper == other.optional_bytes_wrapper
                            & &self.repeated_bool_wrapper == other.repeated_bool_wrapper
                            & &self.repeated_int32_wrapper == other.repeated_int32_wrapper
                            & &self.repeated_int64_wrapper == other.repeated_int64_wrapper
                            & &self.repeated_uint32_wrapper == other.repeated_uint32_wrapper
                            & &self.repeated_uint64_wrapper == other.repeated_uint64_wrapper
                            & &self.repeated_float_wrapper == other.repeated_float_wrapper
                            & &self.repeated_double_wrapper == other.repeated_double_wrapper
                            & &self.repeated_string_wrapper == other.repeated_string_wrapper
                            & &self.repeated_bytes_wrapper == other.repeated_bytes_wrapper
                            && self.optional_duration == other.optional_duration
                            & &self.optional_timestamp == other.optional_timestamp
                            & &self.optional_field_mask == other.optional_field_mask
                            & &self.optional_struct == other.optional_struct
                            & &self.optional_any == other.optional_any
                            & &self.optional_value == other.optional_value
                            & &self.optional_null_value == other.optional_null_value
                            & &self.repeated_duration == other.repeated_duration
                            & &self.repeated_timestamp == other.repeated_timestamp
                            & &self.repeated_fieldmask == other.repeated_fieldmask
                            && self.repeated_struct == other.repeated_struct
                            & &self.repeated_any == other.repeated_any
                            & &self.repeated_value == other.repeated_value
                            & &self.repeated_list_value == other.repeated_list_value
                            & &self.fieldname1 == other.fieldname1
                            & &self.field_name2 == other.field_name2
                            & &self._field_name3 == other._field_name3
                            & &self.field__name4_ == other.field__name4_
                            & &self.field0name5 == other.field0name5
                            & &self.field_0_name6 == other.field_0_name6
                            && self.fieldName7 == other.fieldName7
                            & &self.FieldName8 == other.FieldName8
                            & &self.field_Name9 == other.field_Name9
                            & &self.Field_Name10 == other.Field_Name10
                            & &self.FIELD_NAME11 == other.FIELD_NAME11
                            & &self.FIELD_name12 == other.FIELD_name12
                            & &self.__field_name13 == other.__field_name13
                            & &self.__Field_name14 == other.__Field_name14
                            & &self.field__name15 == other.field__name15
                            & &self.field__Name16 == other.field__Name16
                            && self.field_name17__ == other.field_name17__
                            & &self.Field_name18__ == other.Field_name18__
                            & &self.oneof_field == other.oneof_field
                    }
                }

                impl binformat::BinProto for TestAllTypesProto3 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.optional_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            16u32 => binformat::merge_single(
                                &mut self.optional_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            24u32 => binformat::merge_single(
                                &mut self.optional_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            32u32 => binformat::merge_single(
                                &mut self.optional_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            40u32 => binformat::merge_single(
                                &mut self.optional_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            48u32 => binformat::merge_single(
                                &mut self.optional_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            61u32 => binformat::merge_single(
                                &mut self.optional_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            65u32 => binformat::merge_single(
                                &mut self.optional_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            77u32 => binformat::merge_single(
                                &mut self.optional_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            81u32 => binformat::merge_single(
                                &mut self.optional_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            93u32 => binformat::merge_single(
                                &mut self.optional_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            97u32 => binformat::merge_single(
                                &mut self.optional_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            104u32 => binformat::merge_single(
                                &mut self.optional_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            114u32 => binformat::merge_single(
                                &mut self.optional_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            122u32 => binformat::merge_single(
                                &mut self.optional_bytes,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            146u32 => binformat::merge_optional(
                                &mut self.optional_nested_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            154u32 => binformat::merge_optional(
                                &mut self.optional_foreign_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            168u32 => binformat::merge_single(
                                &mut self.optional_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            176u32 => binformat::merge_single(
                                &mut self.optional_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            184u32 => binformat::merge_single(
                                &mut self.optional_aliased_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            194u32 => binformat::merge_single(
                                &mut self.optional_string_piece,
                                stream,
                                binformat::InputStream::string,
                            ),
                            202u32 => binformat::merge_single(
                                &mut self.optional_cord,
                                stream,
                                binformat::InputStream::string,
                            ),
                            218u32 => binformat::merge_optional(
                                &mut self.recursive_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            248u32 => binformat::merge_repeated(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            250u32 => binformat::merge_packed(
                                &mut self.repeated_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            256u32 => binformat::merge_repeated(
                                &mut self.repeated_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            258u32 => binformat::merge_packed(
                                &mut self.repeated_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            264u32 => binformat::merge_repeated(
                                &mut self.repeated_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            266u32 => binformat::merge_packed(
                                &mut self.repeated_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            272u32 => binformat::merge_repeated(
                                &mut self.repeated_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            274u32 => binformat::merge_packed(
                                &mut self.repeated_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            280u32 => binformat::merge_repeated(
                                &mut self.repeated_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            282u32 => binformat::merge_packed(
                                &mut self.repeated_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            288u32 => binformat::merge_repeated(
                                &mut self.repeated_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            290u32 => binformat::merge_packed(
                                &mut self.repeated_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            301u32 => binformat::merge_repeated(
                                &mut self.repeated_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            298u32 => binformat::merge_packed(
                                &mut self.repeated_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            305u32 => binformat::merge_repeated(
                                &mut self.repeated_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            306u32 => binformat::merge_packed(
                                &mut self.repeated_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            317u32 => binformat::merge_repeated(
                                &mut self.repeated_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            314u32 => binformat::merge_packed(
                                &mut self.repeated_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            321u32 => binformat::merge_repeated(
                                &mut self.repeated_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            322u32 => binformat::merge_packed(
                                &mut self.repeated_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            333u32 => binformat::merge_repeated(
                                &mut self.repeated_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            330u32 => binformat::merge_packed(
                                &mut self.repeated_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            337u32 => binformat::merge_repeated(
                                &mut self.repeated_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            338u32 => binformat::merge_packed(
                                &mut self.repeated_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            344u32 => binformat::merge_repeated(
                                &mut self.repeated_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            346u32 => binformat::merge_packed(
                                &mut self.repeated_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            354u32 => binformat::merge_repeated(
                                &mut self.repeated_string,
                                stream,
                                binformat::InputStream::string,
                            ),
                            362u32 => binformat::merge_repeated(
                                &mut self.repeated_bytes,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            386u32 => binformat::merge_repeated(
                                &mut self.repeated_nested_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            394u32 => binformat::merge_repeated(
                                &mut self.repeated_foreign_message,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            408u32 => binformat::merge_repeated(
                                &mut self.repeated_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            410u32 => binformat::merge_packed(
                                &mut self.repeated_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            416u32 => binformat::merge_repeated(
                                &mut self.repeated_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            418u32 => binformat::merge_packed(
                                &mut self.repeated_foreign_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            434u32 => binformat::merge_repeated(
                                &mut self.repeated_string_piece,
                                stream,
                                binformat::InputStream::string,
                            ),
                            442u32 => binformat::merge_repeated(
                                &mut self.repeated_cord,
                                stream,
                                binformat::InputStream::string,
                            ),
                            600u32 => binformat::merge_repeated(
                                &mut self.packed_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            602u32 => binformat::merge_packed(
                                &mut self.packed_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            608u32 => binformat::merge_repeated(
                                &mut self.packed_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            610u32 => binformat::merge_packed(
                                &mut self.packed_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            616u32 => binformat::merge_repeated(
                                &mut self.packed_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            618u32 => binformat::merge_packed(
                                &mut self.packed_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            624u32 => binformat::merge_repeated(
                                &mut self.packed_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            626u32 => binformat::merge_packed(
                                &mut self.packed_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            632u32 => binformat::merge_repeated(
                                &mut self.packed_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            634u32 => binformat::merge_packed(
                                &mut self.packed_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            640u32 => binformat::merge_repeated(
                                &mut self.packed_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            642u32 => binformat::merge_packed(
                                &mut self.packed_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            653u32 => binformat::merge_repeated(
                                &mut self.packed_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            650u32 => binformat::merge_packed(
                                &mut self.packed_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            657u32 => binformat::merge_repeated(
                                &mut self.packed_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            658u32 => binformat::merge_packed(
                                &mut self.packed_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            669u32 => binformat::merge_repeated(
                                &mut self.packed_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            666u32 => binformat::merge_packed(
                                &mut self.packed_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            673u32 => binformat::merge_repeated(
                                &mut self.packed_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            674u32 => binformat::merge_packed(
                                &mut self.packed_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            685u32 => binformat::merge_repeated(
                                &mut self.packed_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            682u32 => binformat::merge_packed(
                                &mut self.packed_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            689u32 => binformat::merge_repeated(
                                &mut self.packed_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            690u32 => binformat::merge_packed(
                                &mut self.packed_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            696u32 => binformat::merge_repeated(
                                &mut self.packed_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            698u32 => binformat::merge_packed(
                                &mut self.packed_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            704u32 => binformat::merge_repeated(
                                &mut self.packed_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            706u32 => binformat::merge_packed(
                                &mut self.packed_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            712u32 => binformat::merge_repeated(
                                &mut self.unpacked_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            714u32 => binformat::merge_packed(
                                &mut self.unpacked_int32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            720u32 => binformat::merge_repeated(
                                &mut self.unpacked_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            722u32 => binformat::merge_packed(
                                &mut self.unpacked_int64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            728u32 => binformat::merge_repeated(
                                &mut self.unpacked_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            730u32 => binformat::merge_packed(
                                &mut self.unpacked_uint32,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            736u32 => binformat::merge_repeated(
                                &mut self.unpacked_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            738u32 => binformat::merge_packed(
                                &mut self.unpacked_uint64,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            744u32 => binformat::merge_repeated(
                                &mut self.unpacked_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            746u32 => binformat::merge_packed(
                                &mut self.unpacked_sint32,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            752u32 => binformat::merge_repeated(
                                &mut self.unpacked_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            754u32 => binformat::merge_packed(
                                &mut self.unpacked_sint64,
                                stream,
                                binformat::InputStream::sigint,
                            ),
                            765u32 => binformat::merge_repeated(
                                &mut self.unpacked_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            762u32 => binformat::merge_packed(
                                &mut self.unpacked_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            769u32 => binformat::merge_repeated(
                                &mut self.unpacked_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            770u32 => binformat::merge_packed(
                                &mut self.unpacked_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            781u32 => binformat::merge_repeated(
                                &mut self.unpacked_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            778u32 => binformat::merge_packed(
                                &mut self.unpacked_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            785u32 => binformat::merge_repeated(
                                &mut self.unpacked_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            786u32 => binformat::merge_packed(
                                &mut self.unpacked_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            797u32 => binformat::merge_repeated(
                                &mut self.unpacked_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            794u32 => binformat::merge_packed(
                                &mut self.unpacked_float,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            801u32 => binformat::merge_repeated(
                                &mut self.unpacked_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            802u32 => binformat::merge_packed(
                                &mut self.unpacked_double,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            808u32 => binformat::merge_repeated(
                                &mut self.unpacked_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            810u32 => binformat::merge_packed(
                                &mut self.unpacked_bool,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            816u32 => binformat::merge_repeated(
                                &mut self.unpacked_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            818u32 => binformat::merge_packed(
                                &mut self.unpacked_nested_enum,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            450u32 => binformat::merge_map(
                                &mut self.map_int32_int32,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            458u32 => binformat::merge_map(
                                &mut self.map_int64_int64,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            466u32 => binformat::merge_map(
                                &mut self.map_uint32_uint32,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            474u32 => binformat::merge_map(
                                &mut self.map_uint64_uint64,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::varint,
                            ),
                            482u32 => binformat::merge_map(
                                &mut self.map_sint32_sint32,
                                stream,
                                binformat::InputStream::sigint,
                                binformat::InputStream::sigint,
                            ),
                            490u32 => binformat::merge_map(
                                &mut self.map_sint64_sint64,
                                stream,
                                binformat::InputStream::sigint,
                                binformat::InputStream::sigint,
                            ),
                            498u32 => binformat::merge_map(
                                &mut self.map_fixed32_fixed32,
                                stream,
                                binformat::InputStream::fixed32,
                                binformat::InputStream::fixed32,
                            ),
                            506u32 => binformat::merge_map(
                                &mut self.map_fixed64_fixed64,
                                stream,
                                binformat::InputStream::fixed64,
                                binformat::InputStream::fixed64,
                            ),
                            514u32 => binformat::merge_map(
                                &mut self.map_sfixed32_sfixed32,
                                stream,
                                binformat::InputStream::fixed32,
                                binformat::InputStream::fixed32,
                            ),
                            522u32 => binformat::merge_map(
                                &mut self.map_sfixed64_sfixed64,
                                stream,
                                binformat::InputStream::fixed64,
                                binformat::InputStream::fixed64,
                            ),
                            530u32 => binformat::merge_map(
                                &mut self.map_int32_float,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::fixed32,
                            ),
                            538u32 => binformat::merge_map(
                                &mut self.map_int32_double,
                                stream,
                                binformat::InputStream::varint,
                                binformat::InputStream::fixed64,
                            ),
                            546u32 => binformat::merge_map(
                                &mut self.map_bool_bool,
                                stream,
                                binformat::InputStream::bool,
                                binformat::InputStream::bool,
                            ),
                            554u32 => binformat::merge_map(
                                &mut self.map_string_string,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::string,
                            ),
                            562u32 => binformat::merge_map(
                                &mut self.map_string_bytes,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::bytes,
                            ),
                            570u32 => binformat::merge_map(
                                &mut self.map_string_nested_message,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::nested,
                            ),
                            578u32 => binformat::merge_map(
                                &mut self.map_string_foreign_message,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::nested,
                            ),
                            586u32 => binformat::merge_map(
                                &mut self.map_string_nested_enum,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::protoenum,
                            ),
                            594u32 => binformat::merge_map(
                                &mut self.map_string_foreign_enum,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::protoenum,
                            ),
                            1610u32 => binformat::merge_optional(
                                &mut self.optional_bool_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1618u32 => binformat::merge_optional(
                                &mut self.optional_int32_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1626u32 => binformat::merge_optional(
                                &mut self.optional_int64_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1634u32 => binformat::merge_optional(
                                &mut self.optional_uint32_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1642u32 => binformat::merge_optional(
                                &mut self.optional_uint64_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1650u32 => binformat::merge_optional(
                                &mut self.optional_float_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1658u32 => binformat::merge_optional(
                                &mut self.optional_double_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1666u32 => binformat::merge_optional(
                                &mut self.optional_string_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1674u32 => binformat::merge_optional(
                                &mut self.optional_bytes_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1690u32 => binformat::merge_repeated(
                                &mut self.repeated_bool_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1698u32 => binformat::merge_repeated(
                                &mut self.repeated_int32_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1706u32 => binformat::merge_repeated(
                                &mut self.repeated_int64_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1714u32 => binformat::merge_repeated(
                                &mut self.repeated_uint32_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1722u32 => binformat::merge_repeated(
                                &mut self.repeated_uint64_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1730u32 => binformat::merge_repeated(
                                &mut self.repeated_float_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1738u32 => binformat::merge_repeated(
                                &mut self.repeated_double_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1746u32 => binformat::merge_repeated(
                                &mut self.repeated_string_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            1754u32 => binformat::merge_repeated(
                                &mut self.repeated_bytes_wrapper,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2410u32 => binformat::merge_optional(
                                &mut self.optional_duration,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2418u32 => binformat::merge_optional(
                                &mut self.optional_timestamp,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2426u32 => binformat::merge_optional(
                                &mut self.optional_field_mask,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2434u32 => binformat::merge_optional(
                                &mut self.optional_struct,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2442u32 => binformat::merge_optional(
                                &mut self.optional_any,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2450u32 => binformat::merge_optional(
                                &mut self.optional_value,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2456u32 => binformat::merge_single(
                                &mut self.optional_null_value,
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            2490u32 => binformat::merge_repeated(
                                &mut self.repeated_duration,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2498u32 => binformat::merge_repeated(
                                &mut self.repeated_timestamp,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2506u32 => binformat::merge_repeated(
                                &mut self.repeated_fieldmask,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2594u32 => binformat::merge_repeated(
                                &mut self.repeated_struct,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2522u32 => binformat::merge_repeated(
                                &mut self.repeated_any,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2530u32 => binformat::merge_repeated(
                                &mut self.repeated_value,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            2538u32 => binformat::merge_repeated(
                                &mut self.repeated_list_value,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            3208u32 => binformat::merge_single(
                                &mut self.fieldname1,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3216u32 => binformat::merge_single(
                                &mut self.field_name2,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3224u32 => binformat::merge_single(
                                &mut self._field_name3,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3232u32 => binformat::merge_single(
                                &mut self.field__name4_,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3240u32 => binformat::merge_single(
                                &mut self.field0name5,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3248u32 => binformat::merge_single(
                                &mut self.field_0_name6,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3256u32 => binformat::merge_single(
                                &mut self.fieldName7,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3264u32 => binformat::merge_single(
                                &mut self.FieldName8,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3272u32 => binformat::merge_single(
                                &mut self.field_Name9,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3280u32 => binformat::merge_single(
                                &mut self.Field_Name10,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3288u32 => binformat::merge_single(
                                &mut self.FIELD_NAME11,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3296u32 => binformat::merge_single(
                                &mut self.FIELD_name12,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3304u32 => binformat::merge_single(
                                &mut self.__field_name13,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3312u32 => binformat::merge_single(
                                &mut self.__Field_name14,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3320u32 => binformat::merge_single(
                                &mut self.field__name15,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3328u32 => binformat::merge_single(
                                &mut self.field__Name16,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3336u32 => binformat::merge_single(
                                &mut self.field_name17__,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            3344u32 => binformat::merge_single(
                                &mut self.Field_name18__,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            888u32 | 889u32 | 890u32 | 891u32 | 892u32 | 893u32 | 894u32
                            | 895u32 | 896u32 | 897u32 | 898u32 | 899u32 | 900u32 | 901u32
                            | 902u32 | 903u32 | 904u32 | 905u32 | 906u32 | 907u32 | 908u32
                            | 909u32 | 910u32 | 911u32 | 912u32 | 913u32 | 914u32 | 915u32
                            | 916u32 | 917u32 | 918u32 | 919u32 | 920u32 | 921u32 | 922u32
                            | 923u32 | 924u32 | 925u32 | 926u32 | 927u32 | 928u32 | 929u32
                            | 930u32 | 931u32 | 932u32 | 933u32 | 934u32 | 935u32 | 936u32
                            | 937u32 | 938u32 | 939u32 | 940u32 | 941u32 | 942u32 | 943u32
                            | 944u32 | 945u32 | 946u32 | 947u32 | 948u32 | 949u32 | 950u32
                            | 951u32 | 952u32 | 953u32 | 954u32 | 955u32 | 956u32 | 957u32
                            | 958u32 | 959u32 | 960u32 | 961u32 | 962u32 | 963u32 | 964u32
                            | 965u32 | 966u32 | 967u32 => {
                                binformat::merge_oneof(&mut self.oneof_field, tag, stream)
                            }
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.optional_int32,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.optional_int64,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.optional_uint32,
                            24u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.optional_uint64,
                            32u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.optional_sint32,
                            40u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_single(
                            &self.optional_sint64,
                            48u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_single(
                            &self.optional_fixed32,
                            61u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_single(
                            &self.optional_fixed64,
                            65u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_single(
                            &self.optional_sfixed32,
                            77u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_single(
                            &self.optional_sfixed64,
                            81u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_single(
                            &self.optional_float,
                            93u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_single(
                            &self.optional_double,
                            97u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_single(
                            &self.optional_bool,
                            104u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_single(
                            &self.optional_string,
                            114u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_single(
                            &self.optional_bytes,
                            122u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_optional(
                            &self.optional_nested_message,
                            146u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_foreign_message,
                            154u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_single(
                            &self.optional_nested_enum,
                            168u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_single(
                            &self.optional_foreign_enum,
                            176u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_single(
                            &self.optional_aliased_enum,
                            184u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_single(
                            &self.optional_string_piece,
                            194u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_single(
                            &self.optional_cord,
                            202u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_optional(
                            &self.recursive_message,
                            218u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_packed(
                            &self.repeated_int32,
                            250u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.repeated_int64,
                            258u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.repeated_uint32,
                            266u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.repeated_uint64,
                            274u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.repeated_sint32,
                            282u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.repeated_sint64,
                            290u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.repeated_fixed32,
                            298u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.repeated_fixed64,
                            306u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.repeated_sfixed32,
                            314u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.repeated_sfixed64,
                            322u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.repeated_float,
                            330u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.repeated_double,
                            338u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.repeated_bool,
                            346u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_repeated(
                            &self.repeated_string,
                            354u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.repeated_bytes,
                            362u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_repeated(
                            &self.repeated_nested_message,
                            386u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_foreign_message,
                            394u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_packed(
                            &self.repeated_nested_enum,
                            410u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_packed(
                            &self.repeated_foreign_enum,
                            418u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_repeated(
                            &self.repeated_string_piece,
                            434u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_repeated(
                            &self.repeated_cord,
                            442u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_packed(
                            &self.packed_int32,
                            602u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.packed_int64,
                            610u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.packed_uint32,
                            618u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.packed_uint64,
                            626u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.packed_sint32,
                            634u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.packed_sint64,
                            642u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.packed_fixed32,
                            650u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.packed_fixed64,
                            658u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.packed_sfixed32,
                            666u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.packed_sfixed64,
                            674u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.packed_float,
                            682u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.packed_double,
                            690u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.packed_bool,
                            698u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_packed(
                            &self.packed_nested_enum,
                            706u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_packed(
                            &self.unpacked_int32,
                            714u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_int64,
                            722u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_uint32,
                            730u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_uint64,
                            738u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_sint32,
                            746u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_sint64,
                            754u32,
                            stream,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_packed(
                            &self.unpacked_fixed32,
                            762u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.unpacked_fixed64,
                            770u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.unpacked_sfixed32,
                            778u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.unpacked_sfixed64,
                            786u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.unpacked_float,
                            794u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_packed(
                            &self.unpacked_double,
                            802u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_packed(
                            &self.unpacked_bool,
                            810u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_packed(
                            &self.unpacked_nested_enum,
                            818u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_map(
                            &self.map_int32_int32,
                            450u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_int64_int64,
                            458u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_uint32_uint32,
                            466u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_uint64_uint64,
                            474u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_map(
                            &self.map_sint32_sint32,
                            482u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::sigint,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_map(
                            &self.map_sint64_sint64,
                            490u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::sigint,
                            binformat::OutputStream::sigint,
                        );
                        binformat::emit_map(
                            &self.map_fixed32_fixed32,
                            498u32,
                            13u32,
                            21u32,
                            stream,
                            binformat::OutputStream::fixed32,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_fixed64_fixed64,
                            506u32,
                            9u32,
                            17u32,
                            stream,
                            binformat::OutputStream::fixed64,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_sfixed32_sfixed32,
                            514u32,
                            13u32,
                            21u32,
                            stream,
                            binformat::OutputStream::fixed32,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_sfixed64_sfixed64,
                            522u32,
                            9u32,
                            17u32,
                            stream,
                            binformat::OutputStream::fixed64,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_int32_float,
                            530u32,
                            8u32,
                            21u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::fixed32,
                        );
                        binformat::emit_map(
                            &self.map_int32_double,
                            538u32,
                            8u32,
                            17u32,
                            stream,
                            binformat::OutputStream::varint,
                            binformat::OutputStream::fixed64,
                        );
                        binformat::emit_map(
                            &self.map_bool_bool,
                            546u32,
                            8u32,
                            16u32,
                            stream,
                            binformat::OutputStream::bool,
                            binformat::OutputStream::bool,
                        );
                        binformat::emit_map(
                            &self.map_string_string,
                            554u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_map(
                            &self.map_string_bytes,
                            562u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::bytes,
                        );
                        binformat::emit_map(
                            &self.map_string_nested_message,
                            570u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_map(
                            &self.map_string_foreign_message,
                            578u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_map(
                            &self.map_string_nested_enum,
                            586u32,
                            10u32,
                            16u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_map(
                            &self.map_string_foreign_enum,
                            594u32,
                            10u32,
                            16u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_optional(
                            &self.optional_bool_wrapper,
                            1610u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_int32_wrapper,
                            1618u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_int64_wrapper,
                            1626u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_uint32_wrapper,
                            1634u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_uint64_wrapper,
                            1642u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_float_wrapper,
                            1650u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_double_wrapper,
                            1658u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_string_wrapper,
                            1666u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_bytes_wrapper,
                            1674u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_bool_wrapper,
                            1690u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_int32_wrapper,
                            1698u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_int64_wrapper,
                            1706u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_uint32_wrapper,
                            1714u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_uint64_wrapper,
                            1722u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_float_wrapper,
                            1730u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_double_wrapper,
                            1738u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_string_wrapper,
                            1746u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_bytes_wrapper,
                            1754u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_duration,
                            2410u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_timestamp,
                            2418u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_field_mask,
                            2426u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_struct,
                            2434u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_any,
                            2442u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_optional(
                            &self.optional_value,
                            2450u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_single(
                            &self.optional_null_value,
                            2456u32,
                            stream,
                            binformat::OutputStream::protoenum,
                        );
                        binformat::emit_repeated(
                            &self.repeated_duration,
                            2490u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_timestamp,
                            2498u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_fieldmask,
                            2506u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_struct,
                            2594u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_any,
                            2522u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_value,
                            2530u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_repeated(
                            &self.repeated_list_value,
                            2538u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                        binformat::emit_single(
                            &self.fieldname1,
                            3208u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field_name2,
                            3216u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self._field_name3,
                            3224u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field__name4_,
                            3232u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field0name5,
                            3240u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field_0_name6,
                            3248u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.fieldName7,
                            3256u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.FieldName8,
                            3264u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field_Name9,
                            3272u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.Field_Name10,
                            3280u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.FIELD_NAME11,
                            3288u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.FIELD_name12,
                            3296u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.__field_name13,
                            3304u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.__Field_name14,
                            3312u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field__name15,
                            3320u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field__Name16,
                            3328u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.field_name17__,
                            3336u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.Field_name18__,
                            3344u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_oneof(&self.oneof_field, stream);
                    }
                }

                impl textformat::TextProto for TestAllTypesProto3 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "optional_int32" => self.optional_int32.merge_text(stream),
                            "optional_int64" => self.optional_int64.merge_text(stream),
                            "optional_uint32" => self.optional_uint32.merge_text(stream),
                            "optional_uint64" => self.optional_uint64.merge_text(stream),
                            "optional_sint32" => self.optional_sint32.merge_text(stream),
                            "optional_sint64" => self.optional_sint64.merge_text(stream),
                            "optional_fixed32" => self.optional_fixed32.merge_text(stream),
                            "optional_fixed64" => self.optional_fixed64.merge_text(stream),
                            "optional_sfixed32" => self.optional_sfixed32.merge_text(stream),
                            "optional_sfixed64" => self.optional_sfixed64.merge_text(stream),
                            "optional_float" => self.optional_float.merge_text(stream),
                            "optional_double" => self.optional_double.merge_text(stream),
                            "optional_bool" => self.optional_bool.merge_text(stream),
                            "optional_string" => self.optional_string.merge_text(stream),
                            "optional_bytes" => self.optional_bytes.merge_text(stream),
                            "optional_nested_message" => {
                                self.optional_nested_message.merge_text(stream)
                            }
                            "optional_foreign_message" => {
                                self.optional_foreign_message.merge_text(stream)
                            }
                            "optional_nested_enum" => self.optional_nested_enum.merge_text(stream),
                            "optional_foreign_enum" => {
                                self.optional_foreign_enum.merge_text(stream)
                            }
                            "optional_aliased_enum" => {
                                self.optional_aliased_enum.merge_text(stream)
                            }
                            "optional_string_piece" => {
                                self.optional_string_piece.merge_text(stream)
                            }
                            "optional_cord" => self.optional_cord.merge_text(stream),
                            "recursive_message" => self.recursive_message.merge_text(stream),
                            "repeated_int32" => self.repeated_int32.merge_text(stream),
                            "repeated_int64" => self.repeated_int64.merge_text(stream),
                            "repeated_uint32" => self.repeated_uint32.merge_text(stream),
                            "repeated_uint64" => self.repeated_uint64.merge_text(stream),
                            "repeated_sint32" => self.repeated_sint32.merge_text(stream),
                            "repeated_sint64" => self.repeated_sint64.merge_text(stream),
                            "repeated_fixed32" => self.repeated_fixed32.merge_text(stream),
                            "repeated_fixed64" => self.repeated_fixed64.merge_text(stream),
                            "repeated_sfixed32" => self.repeated_sfixed32.merge_text(stream),
                            "repeated_sfixed64" => self.repeated_sfixed64.merge_text(stream),
                            "repeated_float" => self.repeated_float.merge_text(stream),
                            "repeated_double" => self.repeated_double.merge_text(stream),
                            "repeated_bool" => self.repeated_bool.merge_text(stream),
                            "repeated_string" => self.repeated_string.merge_text(stream),
                            "repeated_bytes" => self.repeated_bytes.merge_text(stream),
                            "repeated_nested_message" => {
                                self.repeated_nested_message.merge_text(stream)
                            }
                            "repeated_foreign_message" => {
                                self.repeated_foreign_message.merge_text(stream)
                            }
                            "repeated_nested_enum" => self.repeated_nested_enum.merge_text(stream),
                            "repeated_foreign_enum" => {
                                self.repeated_foreign_enum.merge_text(stream)
                            }
                            "repeated_string_piece" => {
                                self.repeated_string_piece.merge_text(stream)
                            }
                            "repeated_cord" => self.repeated_cord.merge_text(stream),
                            "packed_int32" => self.packed_int32.merge_text(stream),
                            "packed_int64" => self.packed_int64.merge_text(stream),
                            "packed_uint32" => self.packed_uint32.merge_text(stream),
                            "packed_uint64" => self.packed_uint64.merge_text(stream),
                            "packed_sint32" => self.packed_sint32.merge_text(stream),
                            "packed_sint64" => self.packed_sint64.merge_text(stream),
                            "packed_fixed32" => self.packed_fixed32.merge_text(stream),
                            "packed_fixed64" => self.packed_fixed64.merge_text(stream),
                            "packed_sfixed32" => self.packed_sfixed32.merge_text(stream),
                            "packed_sfixed64" => self.packed_sfixed64.merge_text(stream),
                            "packed_float" => self.packed_float.merge_text(stream),
                            "packed_double" => self.packed_double.merge_text(stream),
                            "packed_bool" => self.packed_bool.merge_text(stream),
                            "packed_nested_enum" => self.packed_nested_enum.merge_text(stream),
                            "unpacked_int32" => self.unpacked_int32.merge_text(stream),
                            "unpacked_int64" => self.unpacked_int64.merge_text(stream),
                            "unpacked_uint32" => self.unpacked_uint32.merge_text(stream),
                            "unpacked_uint64" => self.unpacked_uint64.merge_text(stream),
                            "unpacked_sint32" => self.unpacked_sint32.merge_text(stream),
                            "unpacked_sint64" => self.unpacked_sint64.merge_text(stream),
                            "unpacked_fixed32" => self.unpacked_fixed32.merge_text(stream),
                            "unpacked_fixed64" => self.unpacked_fixed64.merge_text(stream),
                            "unpacked_sfixed32" => self.unpacked_sfixed32.merge_text(stream),
                            "unpacked_sfixed64" => self.unpacked_sfixed64.merge_text(stream),
                            "unpacked_float" => self.unpacked_float.merge_text(stream),
                            "unpacked_double" => self.unpacked_double.merge_text(stream),
                            "unpacked_bool" => self.unpacked_bool.merge_text(stream),
                            "unpacked_nested_enum" => self.unpacked_nested_enum.merge_text(stream),
                            "map_int32_int32" => self.map_int32_int32.merge_text(stream),
                            "map_int64_int64" => self.map_int64_int64.merge_text(stream),
                            "map_uint32_uint32" => self.map_uint32_uint32.merge_text(stream),
                            "map_uint64_uint64" => self.map_uint64_uint64.merge_text(stream),
                            "map_sint32_sint32" => self.map_sint32_sint32.merge_text(stream),
                            "map_sint64_sint64" => self.map_sint64_sint64.merge_text(stream),
                            "map_fixed32_fixed32" => self.map_fixed32_fixed32.merge_text(stream),
                            "map_fixed64_fixed64" => self.map_fixed64_fixed64.merge_text(stream),
                            "map_sfixed32_sfixed32" => {
                                self.map_sfixed32_sfixed32.merge_text(stream)
                            }
                            "map_sfixed64_sfixed64" => {
                                self.map_sfixed64_sfixed64.merge_text(stream)
                            }
                            "map_int32_float" => self.map_int32_float.merge_text(stream),
                            "map_int32_double" => self.map_int32_double.merge_text(stream),
                            "map_bool_bool" => self.map_bool_bool.merge_text(stream),
                            "map_string_string" => self.map_string_string.merge_text(stream),
                            "map_string_bytes" => self.map_string_bytes.merge_text(stream),
                            "map_string_nested_message" => {
                                self.map_string_nested_message.merge_text(stream)
                            }
                            "map_string_foreign_message" => {
                                self.map_string_foreign_message.merge_text(stream)
                            }
                            "map_string_nested_enum" => {
                                self.map_string_nested_enum.merge_text(stream)
                            }
                            "map_string_foreign_enum" => {
                                self.map_string_foreign_enum.merge_text(stream)
                            }
                            "optional_bool_wrapper" => {
                                self.optional_bool_wrapper.merge_text(stream)
                            }
                            "optional_int32_wrapper" => {
                                self.optional_int32_wrapper.merge_text(stream)
                            }
                            "optional_int64_wrapper" => {
                                self.optional_int64_wrapper.merge_text(stream)
                            }
                            "optional_uint32_wrapper" => {
                                self.optional_uint32_wrapper.merge_text(stream)
                            }
                            "optional_uint64_wrapper" => {
                                self.optional_uint64_wrapper.merge_text(stream)
                            }
                            "optional_float_wrapper" => {
                                self.optional_float_wrapper.merge_text(stream)
                            }
                            "optional_double_wrapper" => {
                                self.optional_double_wrapper.merge_text(stream)
                            }
                            "optional_string_wrapper" => {
                                self.optional_string_wrapper.merge_text(stream)
                            }
                            "optional_bytes_wrapper" => {
                                self.optional_bytes_wrapper.merge_text(stream)
                            }
                            "repeated_bool_wrapper" => {
                                self.repeated_bool_wrapper.merge_text(stream)
                            }
                            "repeated_int32_wrapper" => {
                                self.repeated_int32_wrapper.merge_text(stream)
                            }
                            "repeated_int64_wrapper" => {
                                self.repeated_int64_wrapper.merge_text(stream)
                            }
                            "repeated_uint32_wrapper" => {
                                self.repeated_uint32_wrapper.merge_text(stream)
                            }
                            "repeated_uint64_wrapper" => {
                                self.repeated_uint64_wrapper.merge_text(stream)
                            }
                            "repeated_float_wrapper" => {
                                self.repeated_float_wrapper.merge_text(stream)
                            }
                            "repeated_double_wrapper" => {
                                self.repeated_double_wrapper.merge_text(stream)
                            }
                            "repeated_string_wrapper" => {
                                self.repeated_string_wrapper.merge_text(stream)
                            }
                            "repeated_bytes_wrapper" => {
                                self.repeated_bytes_wrapper.merge_text(stream)
                            }
                            "optional_duration" => self.optional_duration.merge_text(stream),
                            "optional_timestamp" => self.optional_timestamp.merge_text(stream),
                            "optional_field_mask" => self.optional_field_mask.merge_text(stream),
                            "optional_struct" => self.optional_struct.merge_text(stream),
                            "optional_any" => self.optional_any.merge_text(stream),
                            "optional_value" => self.optional_value.merge_text(stream),
                            "optional_null_value" => self.optional_null_value.merge_text(stream),
                            "repeated_duration" => self.repeated_duration.merge_text(stream),
                            "repeated_timestamp" => self.repeated_timestamp.merge_text(stream),
                            "repeated_fieldmask" => self.repeated_fieldmask.merge_text(stream),
                            "repeated_struct" => self.repeated_struct.merge_text(stream),
                            "repeated_any" => self.repeated_any.merge_text(stream),
                            "repeated_value" => self.repeated_value.merge_text(stream),
                            "repeated_list_value" => self.repeated_list_value.merge_text(stream),
                            "fieldname1" => self.fieldname1.merge_text(stream),
                            "field_name2" => self.field_name2.merge_text(stream),
                            "_field_name3" => self._field_name3.merge_text(stream),
                            "field__name4_" => self.field__name4_.merge_text(stream),
                            "field0name5" => self.field0name5.merge_text(stream),
                            "field_0_name6" => self.field_0_name6.merge_text(stream),
                            "fieldName7" => self.fieldName7.merge_text(stream),
                            "FieldName8" => self.FieldName8.merge_text(stream),
                            "field_Name9" => self.field_Name9.merge_text(stream),
                            "Field_Name10" => self.Field_Name10.merge_text(stream),
                            "FIELD_NAME11" => self.FIELD_NAME11.merge_text(stream),
                            "FIELD_name12" => self.FIELD_name12.merge_text(stream),
                            "__field_name13" => self.__field_name13.merge_text(stream),
                            "__Field_name14" => self.__Field_name14.merge_text(stream),
                            "field__name15" => self.field__name15.merge_text(stream),
                            "field__Name16" => self.field__Name16.merge_text(stream),
                            "field_name17__" => self.field_name17__.merge_text(stream),
                            "Field_name18__" => self.Field_name18__.merge_text(stream),
                            "oneof_uint32"
                            | "oneof_nested_message"
                            | "oneof_string"
                            | "oneof_bytes"
                            | "oneof_bool"
                            | "oneof_uint64"
                            | "oneof_float"
                            | "oneof_double"
                            | "oneof_enum"
                            | "oneof_null_value" => self.oneof_field.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("optional_int32", &self.optional_int32);
                        stream.emit_field("optional_int64", &self.optional_int64);
                        stream.emit_field("optional_uint32", &self.optional_uint32);
                        stream.emit_field("optional_uint64", &self.optional_uint64);
                        stream.emit_field("optional_sint32", &self.optional_sint32);
                        stream.emit_field("optional_sint64", &self.optional_sint64);
                        stream.emit_field("optional_fixed32", &self.optional_fixed32);
                        stream.emit_field("optional_fixed64", &self.optional_fixed64);
                        stream.emit_field("optional_sfixed32", &self.optional_sfixed32);
                        stream.emit_field("optional_sfixed64", &self.optional_sfixed64);
                        stream.emit_field("optional_float", &self.optional_float);
                        stream.emit_field("optional_double", &self.optional_double);
                        stream.emit_field("optional_bool", &self.optional_bool);
                        stream.emit_field("optional_string", &self.optional_string);
                        stream.emit_field("optional_bytes", &self.optional_bytes);
                        stream.emit_field("optional_nested_message", &self.optional_nested_message);
                        stream
                            .emit_field("optional_foreign_message", &self.optional_foreign_message);
                        stream.emit_field("optional_nested_enum", &self.optional_nested_enum);
                        stream.emit_field("optional_foreign_enum", &self.optional_foreign_enum);
                        stream.emit_field("optional_aliased_enum", &self.optional_aliased_enum);
                        stream.emit_field("optional_string_piece", &self.optional_string_piece);
                        stream.emit_field("optional_cord", &self.optional_cord);
                        stream.emit_field("recursive_message", &self.recursive_message);
                        stream.emit_field("repeated_int32", &self.repeated_int32);
                        stream.emit_field("repeated_int64", &self.repeated_int64);
                        stream.emit_field("repeated_uint32", &self.repeated_uint32);
                        stream.emit_field("repeated_uint64", &self.repeated_uint64);
                        stream.emit_field("repeated_sint32", &self.repeated_sint32);
                        stream.emit_field("repeated_sint64", &self.repeated_sint64);
                        stream.emit_field("repeated_fixed32", &self.repeated_fixed32);
                        stream.emit_field("repeated_fixed64", &self.repeated_fixed64);
                        stream.emit_field("repeated_sfixed32", &self.repeated_sfixed32);
                        stream.emit_field("repeated_sfixed64", &self.repeated_sfixed64);
                        stream.emit_field("repeated_float", &self.repeated_float);
                        stream.emit_field("repeated_double", &self.repeated_double);
                        stream.emit_field("repeated_bool", &self.repeated_bool);
                        stream.emit_field("repeated_string", &self.repeated_string);
                        stream.emit_field("repeated_bytes", &self.repeated_bytes);
                        stream.emit_field("repeated_nested_message", &self.repeated_nested_message);
                        stream
                            .emit_field("repeated_foreign_message", &self.repeated_foreign_message);
                        stream.emit_field("repeated_nested_enum", &self.repeated_nested_enum);
                        stream.emit_field("repeated_foreign_enum", &self.repeated_foreign_enum);
                        stream.emit_field("repeated_string_piece", &self.repeated_string_piece);
                        stream.emit_field("repeated_cord", &self.repeated_cord);
                        stream.emit_field("packed_int32", &self.packed_int32);
                        stream.emit_field("packed_int64", &self.packed_int64);
                        stream.emit_field("packed_uint32", &self.packed_uint32);
                        stream.emit_field("packed_uint64", &self.packed_uint64);
                        stream.emit_field("packed_sint32", &self.packed_sint32);
                        stream.emit_field("packed_sint64", &self.packed_sint64);
                        stream.emit_field("packed_fixed32", &self.packed_fixed32);
                        stream.emit_field("packed_fixed64", &self.packed_fixed64);
                        stream.emit_field("packed_sfixed32", &self.packed_sfixed32);
                        stream.emit_field("packed_sfixed64", &self.packed_sfixed64);
                        stream.emit_field("packed_float", &self.packed_float);
                        stream.emit_field("packed_double", &self.packed_double);
                        stream.emit_field("packed_bool", &self.packed_bool);
                        stream.emit_field("packed_nested_enum", &self.packed_nested_enum);
                        stream.emit_field("unpacked_int32", &self.unpacked_int32);
                        stream.emit_field("unpacked_int64", &self.unpacked_int64);
                        stream.emit_field("unpacked_uint32", &self.unpacked_uint32);
                        stream.emit_field("unpacked_uint64", &self.unpacked_uint64);
                        stream.emit_field("unpacked_sint32", &self.unpacked_sint32);
                        stream.emit_field("unpacked_sint64", &self.unpacked_sint64);
                        stream.emit_field("unpacked_fixed32", &self.unpacked_fixed32);
                        stream.emit_field("unpacked_fixed64", &self.unpacked_fixed64);
                        stream.emit_field("unpacked_sfixed32", &self.unpacked_sfixed32);
                        stream.emit_field("unpacked_sfixed64", &self.unpacked_sfixed64);
                        stream.emit_field("unpacked_float", &self.unpacked_float);
                        stream.emit_field("unpacked_double", &self.unpacked_double);
                        stream.emit_field("unpacked_bool", &self.unpacked_bool);
                        stream.emit_field("unpacked_nested_enum", &self.unpacked_nested_enum);
                        stream.emit_field("map_int32_int32", &self.map_int32_int32);
                        stream.emit_field("map_int64_int64", &self.map_int64_int64);
                        stream.emit_field("map_uint32_uint32", &self.map_uint32_uint32);
                        stream.emit_field("map_uint64_uint64", &self.map_uint64_uint64);
                        stream.emit_field("map_sint32_sint32", &self.map_sint32_sint32);
                        stream.emit_field("map_sint64_sint64", &self.map_sint64_sint64);
                        stream.emit_field("map_fixed32_fixed32", &self.map_fixed32_fixed32);
                        stream.emit_field("map_fixed64_fixed64", &self.map_fixed64_fixed64);
                        stream.emit_field("map_sfixed32_sfixed32", &self.map_sfixed32_sfixed32);
                        stream.emit_field("map_sfixed64_sfixed64", &self.map_sfixed64_sfixed64);
                        stream.emit_field("map_int32_float", &self.map_int32_float);
                        stream.emit_field("map_int32_double", &self.map_int32_double);
                        stream.emit_field("map_bool_bool", &self.map_bool_bool);
                        stream.emit_field("map_string_string", &self.map_string_string);
                        stream.emit_field("map_string_bytes", &self.map_string_bytes);
                        stream.emit_field(
                            "map_string_nested_message",
                            &self.map_string_nested_message,
                        );
                        stream.emit_field(
                            "map_string_foreign_message",
                            &self.map_string_foreign_message,
                        );
                        stream.emit_field("map_string_nested_enum", &self.map_string_nested_enum);
                        stream.emit_field("map_string_foreign_enum", &self.map_string_foreign_enum);
                        stream.emit_field("optional_bool_wrapper", &self.optional_bool_wrapper);
                        stream.emit_field("optional_int32_wrapper", &self.optional_int32_wrapper);
                        stream.emit_field("optional_int64_wrapper", &self.optional_int64_wrapper);
                        stream.emit_field("optional_uint32_wrapper", &self.optional_uint32_wrapper);
                        stream.emit_field("optional_uint64_wrapper", &self.optional_uint64_wrapper);
                        stream.emit_field("optional_float_wrapper", &self.optional_float_wrapper);
                        stream.emit_field("optional_double_wrapper", &self.optional_double_wrapper);
                        stream.emit_field("optional_string_wrapper", &self.optional_string_wrapper);
                        stream.emit_field("optional_bytes_wrapper", &self.optional_bytes_wrapper);
                        stream.emit_field("repeated_bool_wrapper", &self.repeated_bool_wrapper);
                        stream.emit_field("repeated_int32_wrapper", &self.repeated_int32_wrapper);
                        stream.emit_field("repeated_int64_wrapper", &self.repeated_int64_wrapper);
                        stream.emit_field("repeated_uint32_wrapper", &self.repeated_uint32_wrapper);
                        stream.emit_field("repeated_uint64_wrapper", &self.repeated_uint64_wrapper);
                        stream.emit_field("repeated_float_wrapper", &self.repeated_float_wrapper);
                        stream.emit_field("repeated_double_wrapper", &self.repeated_double_wrapper);
                        stream.emit_field("repeated_string_wrapper", &self.repeated_string_wrapper);
                        stream.emit_field("repeated_bytes_wrapper", &self.repeated_bytes_wrapper);
                        stream.emit_field("optional_duration", &self.optional_duration);
                        stream.emit_field("optional_timestamp", &self.optional_timestamp);
                        stream.emit_field("optional_field_mask", &self.optional_field_mask);
                        stream.emit_field("optional_struct", &self.optional_struct);
                        stream.emit_field("optional_any", &self.optional_any);
                        stream.emit_field("optional_value", &self.optional_value);
                        stream.emit_field("optional_null_value", &self.optional_null_value);
                        stream.emit_field("repeated_duration", &self.repeated_duration);
                        stream.emit_field("repeated_timestamp", &self.repeated_timestamp);
                        stream.emit_field("repeated_fieldmask", &self.repeated_fieldmask);
                        stream.emit_field("repeated_struct", &self.repeated_struct);
                        stream.emit_field("repeated_any", &self.repeated_any);
                        stream.emit_field("repeated_value", &self.repeated_value);
                        stream.emit_field("repeated_list_value", &self.repeated_list_value);
                        stream.emit_field("fieldname1", &self.fieldname1);
                        stream.emit_field("field_name2", &self.field_name2);
                        stream.emit_field("_field_name3", &self._field_name3);
                        stream.emit_field("field__name4_", &self.field__name4_);
                        stream.emit_field("field0name5", &self.field0name5);
                        stream.emit_field("field_0_name6", &self.field_0_name6);
                        stream.emit_field("fieldName7", &self.fieldName7);
                        stream.emit_field("FieldName8", &self.FieldName8);
                        stream.emit_field("field_Name9", &self.field_Name9);
                        stream.emit_field("Field_Name10", &self.Field_Name10);
                        stream.emit_field("FIELD_NAME11", &self.FIELD_NAME11);
                        stream.emit_field("FIELD_name12", &self.FIELD_name12);
                        stream.emit_field("__field_name13", &self.__field_name13);
                        stream.emit_field("__Field_name14", &self.__Field_name14);
                        stream.emit_field("field__name15", &self.field__name15);
                        stream.emit_field("field__Name16", &self.field__Name16);
                        stream.emit_field("field_name17__", &self.field_name17__);
                        stream.emit_field("Field_name18__", &self.Field_name18__);
                        stream.emit_oneof(&self.oneof_field);
                    }
                }

                pub struct TestAllTypesProto3NestedMessage {
                    #[field(1u32, "a", varint, singular)]
                    pub a: i32,
                    #[field(2u32, "corecursive", nested, optional)]
                    pub corecursive: Option<Box<TestAllTypesProto3>>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for TestAllTypesProto3NestedMessage {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "TestAllTypesProto3NestedMessage",
                            "a",
                            &&self.a,
                            "corecursive",
                            &&self.corecursive,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for TestAllTypesProto3NestedMessage {
                    #[inline]
                    fn default() -> TestAllTypesProto3NestedMessage {
                        TestAllTypesProto3NestedMessage {
                            a: ::core::default::Default::default(),
                            corecursive: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for TestAllTypesProto3NestedMessage {
                    #[inline]
                    fn clone(&self) -> TestAllTypesProto3NestedMessage {
                        TestAllTypesProto3NestedMessage {
                            a: ::core::clone::Clone::clone(&self.a),
                            corecursive: ::core::clone::Clone::clone(&self.corecursive),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestAllTypesProto3NestedMessage {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestAllTypesProto3NestedMessage {
                    #[inline]
                    fn eq(&self, other: &TestAllTypesProto3NestedMessage) -> bool {
                        self.a == other.a & &self.corecursive == other.corecursive
                    }
                }

                impl binformat::BinProto for TestAllTypesProto3NestedMessage {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.a,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            18u32 => binformat::merge_optional(
                                &mut self.corecursive,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.a,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_optional(
                            &self.corecursive,
                            18u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                    }
                }

                impl textformat::TextProto for TestAllTypesProto3NestedMessage {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "a" => self.a.merge_text(stream),
                            "corecursive" => self.corecursive.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("a", &self.a);
                        stream.emit_field("corecursive", &self.corecursive);
                    }
                }

                pub struct ForeignMessage {
                    #[field(1u32, "c", varint, singular)]
                    pub c: i32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ForeignMessage {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ForeignMessage",
                            "c",
                            &&self.c,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for ForeignMessage {
                    #[inline]
                    fn default() -> ForeignMessage {
                        ForeignMessage {
                            c: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ForeignMessage {
                    #[inline]
                    fn clone(&self) -> ForeignMessage {
                        ForeignMessage {
                            c: ::core::clone::Clone::clone(&self.c),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ForeignMessage {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ForeignMessage {
                    #[inline]
                    fn eq(&self, other: &ForeignMessage) -> bool {
                        self.c == other.c
                    }
                }

                impl binformat::BinProto for ForeignMessage {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.c,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.c,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for ForeignMessage {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "c" => self.c.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("c", &self.c);
                    }
                }

                pub struct NullHypothesisProto3 {}

                #[automatically_derived]
                impl ::core::fmt::Debug for NullHypothesisProto3 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "NullHypothesisProto3")
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for NullHypothesisProto3 {
                    #[inline]
                    fn default() -> NullHypothesisProto3 {
                        NullHypothesisProto3 {}
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for NullHypothesisProto3 {
                    #[inline]
                    fn clone(&self) -> NullHypothesisProto3 {
                        NullHypothesisProto3 {}
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for NullHypothesisProto3 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for NullHypothesisProto3 {
                    #[inline]
                    fn eq(&self, other: &NullHypothesisProto3) -> bool {
                        true
                    }
                }

                impl binformat::BinProto for NullHypothesisProto3 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {}
                }

                impl textformat::TextProto for NullHypothesisProto3 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {}
                }

                pub struct EnumOnlyProto3 {}

                #[automatically_derived]
                impl ::core::fmt::Debug for EnumOnlyProto3 {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "EnumOnlyProto3")
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for EnumOnlyProto3 {
                    #[inline]
                    fn default() -> EnumOnlyProto3 {
                        EnumOnlyProto3 {}
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for EnumOnlyProto3 {
                    #[inline]
                    fn clone(&self) -> EnumOnlyProto3 {
                        EnumOnlyProto3 {}
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for EnumOnlyProto3 {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for EnumOnlyProto3 {
                    #[inline]
                    fn eq(&self, other: &EnumOnlyProto3) -> bool {
                        true
                    }
                }

                impl binformat::BinProto for EnumOnlyProto3 {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {}
                }

                impl textformat::TextProto for EnumOnlyProto3 {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {}
                }
            }

            pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
                test_messages_proto3::register_types(registry);
            }
        }

        pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
            proto2::register_types(registry);
            proto3::register_types(registry);
        }
    }

    pub mod google {
        pub mod protobuf {
            pub mod r#struct {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct NullValue(pub u32);

                #[automatically_derived]
                impl ::core::default::Default for NullValue {
                    #[inline]
                    fn default() -> NullValue {
                        NullValue(::core::default::Default::default())
                    }
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for NullValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NullValue", &&self.0)
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for NullValue {
                    #[inline]
                    fn clone(&self) -> NullValue {
                        let _: ::core::clone::AssertParamIsClone<u32>;
                        *self
                    }
                }

                #[automatically_derived]
                impl ::core::marker::Copy for NullValue {}

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for NullValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for NullValue {
                    #[inline]
                    fn eq(&self, other: &NullValue) -> bool {
                        self.0 == other.0
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralEq for NullValue {}

                #[automatically_derived]
                impl ::core::cmp::Eq for NullValue {
                    #[inline]
                    #[doc(hidden)]
                    #[no_coverage]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<u32>;
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::PartialOrd for NullValue {
                    #[inline]
                    fn partial_cmp(
                        &self,
                        other: &NullValue,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
                    }
                }

                #[automatically_derived]
                impl ::core::cmp::Ord for NullValue {
                    #[inline]
                    fn cmp(&self, other: &NullValue) -> ::core::cmp::Ordering {
                        ::core::cmp::Ord::cmp(&self.0, &other.0)
                    }
                }

                impl NullValue {
                    pub const NULL_VALUE: NullValue = NullValue(0u32);
                }

                impl From<u32> for NullValue {
                    fn from(v: u32) -> Self {
                        Self(v)
                    }
                }

                impl From<NullValue> for u32 {
                    fn from(v: NullValue) -> Self {
                        v.0
                    }
                }

                impl textformat::TextField for NullValue {
                    fn merge_value(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "NULL_VALUE" => *self = Self::from(0u32),
                            name => return textformat::unknown(name),
                        }
                        Ok(())
                    }
                    fn emit_value(&self, stream: &mut textformat::OutputStream) {
                        match self.0 {
                            0u32 => stream.ident("NULL_VALUE"),
                            other => stream.disp(&other),
                        }
                    }
                }

                pub struct Struct {
                    #[field(1u32, "fields", map(string, nested), singular)]
                    pub fields: ::std::collections::BTreeMap<String, Value>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Struct {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Struct",
                            "fields",
                            &&self.fields,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Struct {
                    #[inline]
                    fn default() -> Struct {
                        Struct {
                            fields: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Struct {
                    #[inline]
                    fn clone(&self) -> Struct {
                        Struct {
                            fields: ::core::clone::Clone::clone(&self.fields),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Struct {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Struct {
                    #[inline]
                    fn eq(&self, other: &Struct) -> bool {
                        self.fields == other.fields
                    }
                }

                impl binformat::BinProto for Struct {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_map(
                                &mut self.fields,
                                stream,
                                binformat::InputStream::string,
                                binformat::InputStream::nested,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_map(
                            &self.fields,
                            10u32,
                            10u32,
                            18u32,
                            stream,
                            binformat::OutputStream::string,
                            binformat::OutputStream::nested,
                        );
                    }
                }

                impl textformat::TextProto for Struct {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "fields" => self.fields.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("fields", &self.fields);
                    }
                }

                pub enum ValueOneOfKind {
                    #[field(1u32, "null_value", protoenum, raw)]
                    NullValue(NullValue),
                    #[field(2u32, "number_value", fixed64, raw)]
                    NumberValue(f64),
                    #[field(3u32, "string_value", string, raw)]
                    StringValue(String),
                    #[field(4u32, "bool_value", bool, raw)]
                    BoolValue(bool),
                    #[field(5u32, "struct_value", nested, raw)]
                    StructValue(Struct),
                    #[field(6u32, "list_value", nested, raw)]
                    ListValue(ListValue),
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ValueOneOfKind {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        match self {
                            ValueOneOfKind::NullValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "NullValue",
                                    &__self_0,
                                )
                            }
                            ValueOneOfKind::NumberValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "NumberValue",
                                    &__self_0,
                                )
                            }
                            ValueOneOfKind::StringValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "StringValue",
                                    &__self_0,
                                )
                            }
                            ValueOneOfKind::BoolValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "BoolValue",
                                    &__self_0,
                                )
                            }
                            ValueOneOfKind::StructValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "StructValue",
                                    &__self_0,
                                )
                            }
                            ValueOneOfKind::ListValue(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "ListValue",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ValueOneOfKind {
                    #[inline]
                    fn clone(&self) -> ValueOneOfKind {
                        match self {
                            ValueOneOfKind::NullValue(__self_0) => {
                                ValueOneOfKind::NullValue(::core::clone::Clone::clone(__self_0))
                            }
                            ValueOneOfKind::NumberValue(__self_0) => {
                                ValueOneOfKind::NumberValue(::core::clone::Clone::clone(__self_0))
                            }
                            ValueOneOfKind::StringValue(__self_0) => {
                                ValueOneOfKind::StringValue(::core::clone::Clone::clone(__self_0))
                            }
                            ValueOneOfKind::BoolValue(__self_0) => {
                                ValueOneOfKind::BoolValue(::core::clone::Clone::clone(__self_0))
                            }
                            ValueOneOfKind::StructValue(__self_0) => {
                                ValueOneOfKind::StructValue(::core::clone::Clone::clone(__self_0))
                            }
                            ValueOneOfKind::ListValue(__self_0) => {
                                ValueOneOfKind::ListValue(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ValueOneOfKind {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ValueOneOfKind {
                    #[inline]
                    fn eq(&self, other: &ValueOneOfKind) -> bool {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                        __self_tag == __arg1_tag
                            && match (self, other) {
                            (
                                ValueOneOfKind::NullValue(__self_0),
                                ValueOneOfKind::NullValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ValueOneOfKind::NumberValue(__self_0),
                                ValueOneOfKind::NumberValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ValueOneOfKind::StringValue(__self_0),
                                ValueOneOfKind::StringValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ValueOneOfKind::BoolValue(__self_0),
                                ValueOneOfKind::BoolValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ValueOneOfKind::StructValue(__self_0),
                                ValueOneOfKind::StructValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            (
                                ValueOneOfKind::ListValue(__self_0),
                                ValueOneOfKind::ListValue(__arg1_0),
                            ) => *__self_0 == *__arg1_0,
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    }
                }

                impl ValueOneOfKind {
                    fn make_NullValue_mut(&mut self) -> &mut NullValue {
                        loop {
                            match self {
                                Self::NullValue(v) => return v,
                                _ => *self = Self::NullValue(Default::default()),
                            }
                        }
                    }
                    fn make_NumberValue_mut(&mut self) -> &mut f64 {
                        loop {
                            match self {
                                Self::NumberValue(v) => return v,
                                _ => *self = Self::NumberValue(Default::default()),
                            }
                        }
                    }
                    fn make_StringValue_mut(&mut self) -> &mut String {
                        loop {
                            match self {
                                Self::StringValue(v) => return v,
                                _ => *self = Self::StringValue(Default::default()),
                            }
                        }
                    }
                    fn make_BoolValue_mut(&mut self) -> &mut bool {
                        loop {
                            match self {
                                Self::BoolValue(v) => return v,
                                _ => *self = Self::BoolValue(Default::default()),
                            }
                        }
                    }
                    fn make_StructValue_mut(&mut self) -> &mut Struct {
                        loop {
                            match self {
                                Self::StructValue(v) => return v,
                                _ => *self = Self::StructValue(Default::default()),
                            }
                        }
                    }
                    fn make_ListValue_mut(&mut self) -> &mut ListValue {
                        loop {
                            match self {
                                Self::ListValue(v) => return v,
                                _ => *self = Self::ListValue(Default::default()),
                            }
                        }
                    }
                }

                impl binformat::BinProto for ValueOneOfKind {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                self.make_NullValue_mut(),
                                stream,
                                binformat::InputStream::protoenum,
                            ),
                            17u32 => binformat::merge_single(
                                self.make_NumberValue_mut(),
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            26u32 => binformat::merge_single(
                                self.make_StringValue_mut(),
                                stream,
                                binformat::InputStream::string,
                            ),
                            32u32 => binformat::merge_single(
                                self.make_BoolValue_mut(),
                                stream,
                                binformat::InputStream::bool,
                            ),
                            42u32 => binformat::merge_single(
                                self.make_StructValue_mut(),
                                stream,
                                binformat::InputStream::nested,
                            ),
                            50u32 => binformat::merge_single(
                                self.make_ListValue_mut(),
                                stream,
                                binformat::InputStream::nested,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        match self {
                            Self::NullValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    8u32,
                                    stream,
                                    binformat::OutputStream::protoenum,
                                );
                            }
                            Self::NumberValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    17u32,
                                    stream,
                                    binformat::OutputStream::fixed64,
                                );
                            }
                            Self::StringValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    26u32,
                                    stream,
                                    binformat::OutputStream::string,
                                );
                            }
                            Self::BoolValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    32u32,
                                    stream,
                                    binformat::OutputStream::bool,
                                );
                            }
                            Self::StructValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    42u32,
                                    stream,
                                    binformat::OutputStream::nested,
                                );
                            }
                            Self::ListValue(v) => {
                                binformat::emit_raw(
                                    v,
                                    50u32,
                                    stream,
                                    binformat::OutputStream::nested,
                                );
                            }
                        }
                    }
                }

                impl textformat::TextProto for ValueOneOfKind {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "null_value" => self.make_NullValue_mut().merge_text(stream),
                            "number_value" => self.make_NumberValue_mut().merge_text(stream),
                            "string_value" => self.make_StringValue_mut().merge_text(stream),
                            "bool_value" => self.make_BoolValue_mut().merge_text(stream),
                            "struct_value" => self.make_StructValue_mut().merge_text(stream),
                            "list_value" => self.make_ListValue_mut().merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        match self {
                            Self::NullValue(v) => stream.emit_field("null_value", v),
                            Self::NumberValue(v) => stream.emit_field("number_value", v),
                            Self::StringValue(v) => stream.emit_field("string_value", v),
                            Self::BoolValue(v) => stream.emit_field("bool_value", v),
                            Self::StructValue(v) => stream.emit_field("struct_value", v),
                            Self::ListValue(v) => stream.emit_field("list_value", v),
                        }
                    }
                }

                impl Default for ValueOneOfKind {
                    fn default() -> Self {
                        Self::NullValue(Default::default())
                    }
                }

                pub struct Value {
                    #[oneof([1u32, 2u32, 3u32, 4u32, 5u32, 6u32, ], ["null_value", "number_value", "string_value", "bool_value", "struct_value", "list_value", ])]
                    pub kind: Option<ValueOneOfKind>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Value {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Value",
                            "kind",
                            &&self.kind,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Value {
                    #[inline]
                    fn default() -> Value {
                        Value {
                            kind: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Value {
                    #[inline]
                    fn clone(&self) -> Value {
                        Value {
                            kind: ::core::clone::Clone::clone(&self.kind),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Value {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Value {
                    #[inline]
                    fn eq(&self, other: &Value) -> bool {
                        self.kind == other.kind
                    }
                }

                impl binformat::BinProto for Value {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 | 9u32 | 10u32 | 11u32 | 12u32 | 13u32 | 14u32 | 15u32 | 16u32
                            | 17u32 | 18u32 | 19u32 | 20u32 | 21u32 | 22u32 | 23u32 | 24u32
                            | 25u32 | 26u32 | 27u32 | 28u32 | 29u32 | 30u32 | 31u32 | 32u32
                            | 33u32 | 34u32 | 35u32 | 36u32 | 37u32 | 38u32 | 39u32 | 40u32
                            | 41u32 | 42u32 | 43u32 | 44u32 | 45u32 | 46u32 | 47u32 | 48u32
                            | 49u32 | 50u32 | 51u32 | 52u32 | 53u32 | 54u32 | 55u32 => {
                                binformat::merge_oneof(&mut self.kind, tag, stream)
                            }
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_oneof(&self.kind, stream);
                    }
                }

                impl textformat::TextProto for Value {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "null_value" | "number_value" | "string_value" | "bool_value"
                            | "struct_value" | "list_value" => self.kind.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_oneof(&self.kind);
                    }
                }

                pub struct ListValue {
                    #[field(1u32, "values", nested, repeated)]
                    pub values: Vec<Value>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for ListValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ListValue",
                            "values",
                            &&self.values,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for ListValue {
                    #[inline]
                    fn default() -> ListValue {
                        ListValue {
                            values: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for ListValue {
                    #[inline]
                    fn clone(&self) -> ListValue {
                        ListValue {
                            values: ::core::clone::Clone::clone(&self.values),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ListValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for ListValue {
                    #[inline]
                    fn eq(&self, other: &ListValue) -> bool {
                        self.values == other.values
                    }
                }

                impl binformat::BinProto for ListValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_repeated(
                                &mut self.values,
                                stream,
                                binformat::InputStream::nested,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_repeated(
                            &self.values,
                            10u32,
                            stream,
                            binformat::OutputStream::nested,
                        );
                    }
                }

                impl textformat::TextProto for ListValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "values" => self.values.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("values", &self.values);
                    }
                }
            }

            pub mod any {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct Any {
                    #[field(1u32, "type_url", string, singular)]
                    pub type_url: String,
                    #[field(2u32, "value", bytes, singular)]
                    pub value: Vec<u8>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Any {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Any",
                            "type_url",
                            &&self.type_url,
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Any {
                    #[inline]
                    fn default() -> Any {
                        Any {
                            type_url: ::core::default::Default::default(),
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Any {
                    #[inline]
                    fn clone(&self) -> Any {
                        Any {
                            type_url: ::core::clone::Clone::clone(&self.type_url),
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Any {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Any {
                    #[inline]
                    fn eq(&self, other: &Any) -> bool {
                        self.type_url == other.type_url & &self.value == other.value
                    }
                }

                impl binformat::BinProto for Any {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_single(
                                &mut self.type_url,
                                stream,
                                binformat::InputStream::string,
                            ),
                            18u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.type_url,
                            10u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                        binformat::emit_single(
                            &self.value,
                            18u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                    }
                }

                impl textformat::TextProto for Any {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "type_url" => self.type_url.merge_text(stream),
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("type_url", &self.type_url);
                        stream.emit_field("value", &self.value);
                    }
                }
            }

            pub mod field_mask {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct FieldMask {
                    #[field(1u32, "paths", string, repeated)]
                    pub paths: Vec<String>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for FieldMask {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "FieldMask",
                            "paths",
                            &&self.paths,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for FieldMask {
                    #[inline]
                    fn default() -> FieldMask {
                        FieldMask {
                            paths: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for FieldMask {
                    #[inline]
                    fn clone(&self) -> FieldMask {
                        FieldMask {
                            paths: ::core::clone::Clone::clone(&self.paths),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for FieldMask {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for FieldMask {
                    #[inline]
                    fn eq(&self, other: &FieldMask) -> bool {
                        self.paths == other.paths
                    }
                }

                impl binformat::BinProto for FieldMask {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_repeated(
                                &mut self.paths,
                                stream,
                                binformat::InputStream::string,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_repeated(
                            &self.paths,
                            10u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                    }
                }

                impl textformat::TextProto for FieldMask {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "paths" => self.paths.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("paths", &self.paths);
                    }
                }
            }

            pub mod timestamp {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct Timestamp {
                    #[field(1u32, "seconds", varint, singular)]
                    pub seconds: i64,
                    #[field(2u32, "nanos", varint, singular)]
                    pub nanos: i32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Timestamp {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Timestamp",
                            "seconds",
                            &&self.seconds,
                            "nanos",
                            &&self.nanos,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Timestamp {
                    #[inline]
                    fn default() -> Timestamp {
                        Timestamp {
                            seconds: ::core::default::Default::default(),
                            nanos: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Timestamp {
                    #[inline]
                    fn clone(&self) -> Timestamp {
                        Timestamp {
                            seconds: ::core::clone::Clone::clone(&self.seconds),
                            nanos: ::core::clone::Clone::clone(&self.nanos),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Timestamp {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Timestamp {
                    #[inline]
                    fn eq(&self, other: &Timestamp) -> bool {
                        self.seconds == other.seconds & &self.nanos == other.nanos
                    }
                }

                impl binformat::BinProto for Timestamp {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.seconds,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            16u32 => binformat::merge_single(
                                &mut self.nanos,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.seconds,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.nanos,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for Timestamp {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "seconds" => self.seconds.merge_text(stream),
                            "nanos" => self.nanos.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("seconds", &self.seconds);
                        stream.emit_field("nanos", &self.nanos);
                    }
                }
            }

            pub mod duration {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct Duration {
                    #[field(1u32, "seconds", varint, singular)]
                    pub seconds: i64,
                    #[field(2u32, "nanos", varint, singular)]
                    pub nanos: i32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Duration {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Duration",
                            "seconds",
                            &&self.seconds,
                            "nanos",
                            &&self.nanos,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Duration {
                    #[inline]
                    fn default() -> Duration {
                        Duration {
                            seconds: ::core::default::Default::default(),
                            nanos: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Duration {
                    #[inline]
                    fn clone(&self) -> Duration {
                        Duration {
                            seconds: ::core::clone::Clone::clone(&self.seconds),
                            nanos: ::core::clone::Clone::clone(&self.nanos),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Duration {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Duration {
                    #[inline]
                    fn eq(&self, other: &Duration) -> bool {
                        self.seconds == other.seconds & &self.nanos == other.nanos
                    }
                }

                impl binformat::BinProto for Duration {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.seconds,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            16u32 => binformat::merge_single(
                                &mut self.nanos,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.seconds,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                        binformat::emit_single(
                            &self.nanos,
                            16u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for Duration {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "seconds" => self.seconds.merge_text(stream),
                            "nanos" => self.nanos.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("seconds", &self.seconds);
                        stream.emit_field("nanos", &self.nanos);
                    }
                }
            }

            pub mod wrappers {
                use ::protokit::*;

                pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {}

                pub struct DoubleValue {
                    #[field(1u32, "value", fixed64, singular)]
                    pub value: f64,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for DoubleValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "DoubleValue",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for DoubleValue {
                    #[inline]
                    fn default() -> DoubleValue {
                        DoubleValue {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for DoubleValue {
                    #[inline]
                    fn clone(&self) -> DoubleValue {
                        DoubleValue {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for DoubleValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for DoubleValue {
                    #[inline]
                    fn eq(&self, other: &DoubleValue) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for DoubleValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            9u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::fixed64,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            9u32,
                            stream,
                            binformat::OutputStream::fixed64,
                        );
                    }
                }

                impl textformat::TextProto for DoubleValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct FloatValue {
                    #[field(1u32, "value", fixed32, singular)]
                    pub value: f32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for FloatValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "FloatValue",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for FloatValue {
                    #[inline]
                    fn default() -> FloatValue {
                        FloatValue {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for FloatValue {
                    #[inline]
                    fn clone(&self) -> FloatValue {
                        FloatValue {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for FloatValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for FloatValue {
                    #[inline]
                    fn eq(&self, other: &FloatValue) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for FloatValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            13u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::fixed32,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            13u32,
                            stream,
                            binformat::OutputStream::fixed32,
                        );
                    }
                }

                impl textformat::TextProto for FloatValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct Int64Value {
                    #[field(1u32, "value", varint, singular)]
                    pub value: i64,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Int64Value {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Int64Value",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Int64Value {
                    #[inline]
                    fn default() -> Int64Value {
                        Int64Value {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Int64Value {
                    #[inline]
                    fn clone(&self) -> Int64Value {
                        Int64Value {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Int64Value {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Int64Value {
                    #[inline]
                    fn eq(&self, other: &Int64Value) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for Int64Value {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for Int64Value {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct UInt64Value {
                    #[field(1u32, "value", varint, singular)]
                    pub value: u64,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for UInt64Value {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "UInt64Value",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for UInt64Value {
                    #[inline]
                    fn default() -> UInt64Value {
                        UInt64Value {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for UInt64Value {
                    #[inline]
                    fn clone(&self) -> UInt64Value {
                        UInt64Value {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for UInt64Value {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for UInt64Value {
                    #[inline]
                    fn eq(&self, other: &UInt64Value) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for UInt64Value {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for UInt64Value {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct Int32Value {
                    #[field(1u32, "value", varint, singular)]
                    pub value: i32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for Int32Value {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Int32Value",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for Int32Value {
                    #[inline]
                    fn default() -> Int32Value {
                        Int32Value {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for Int32Value {
                    #[inline]
                    fn clone(&self) -> Int32Value {
                        Int32Value {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Int32Value {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for Int32Value {
                    #[inline]
                    fn eq(&self, other: &Int32Value) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for Int32Value {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for Int32Value {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct UInt32Value {
                    #[field(1u32, "value", varint, singular)]
                    pub value: u32,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for UInt32Value {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "UInt32Value",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for UInt32Value {
                    #[inline]
                    fn default() -> UInt32Value {
                        UInt32Value {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for UInt32Value {
                    #[inline]
                    fn clone(&self) -> UInt32Value {
                        UInt32Value {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for UInt32Value {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for UInt32Value {
                    #[inline]
                    fn eq(&self, other: &UInt32Value) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for UInt32Value {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::varint,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            8u32,
                            stream,
                            binformat::OutputStream::varint,
                        );
                    }
                }

                impl textformat::TextProto for UInt32Value {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct BoolValue {
                    #[field(1u32, "value", bool, singular)]
                    pub value: bool,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for BoolValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "BoolValue",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for BoolValue {
                    #[inline]
                    fn default() -> BoolValue {
                        BoolValue {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for BoolValue {
                    #[inline]
                    fn clone(&self) -> BoolValue {
                        BoolValue {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for BoolValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for BoolValue {
                    #[inline]
                    fn eq(&self, other: &BoolValue) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for BoolValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            8u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::bool,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            8u32,
                            stream,
                            binformat::OutputStream::bool,
                        );
                    }
                }

                impl textformat::TextProto for BoolValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct StringValue {
                    #[field(1u32, "value", string, singular)]
                    pub value: String,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for StringValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "StringValue",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for StringValue {
                    #[inline]
                    fn default() -> StringValue {
                        StringValue {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for StringValue {
                    #[inline]
                    fn clone(&self) -> StringValue {
                        StringValue {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for StringValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for StringValue {
                    #[inline]
                    fn eq(&self, other: &StringValue) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for StringValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::string,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            10u32,
                            stream,
                            binformat::OutputStream::string,
                        );
                    }
                }

                impl textformat::TextProto for StringValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }

                pub struct BytesValue {
                    #[field(1u32, "value", bytes, singular)]
                    pub value: Vec<u8>,
                }

                #[automatically_derived]
                impl ::core::fmt::Debug for BytesValue {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "BytesValue",
                            "value",
                            &&self.value,
                        )
                    }
                }

                #[automatically_derived]
                impl ::core::default::Default for BytesValue {
                    #[inline]
                    fn default() -> BytesValue {
                        BytesValue {
                            value: ::core::default::Default::default(),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::clone::Clone for BytesValue {
                    #[inline]
                    fn clone(&self) -> BytesValue {
                        BytesValue {
                            value: ::core::clone::Clone::clone(&self.value),
                        }
                    }
                }

                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for BytesValue {}

                #[automatically_derived]
                impl ::core::cmp::PartialEq for BytesValue {
                    #[inline]
                    fn eq(&self, other: &BytesValue) -> bool {
                        self.value == other.value
                    }
                }

                impl binformat::BinProto for BytesValue {
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        stream: &mut binformat::InputStream,
                    ) -> binformat::Result<()> {
                        match tag {
                            10u32 => binformat::merge_single(
                                &mut self.value,
                                stream,
                                binformat::InputStream::bytes,
                            ),
                            other => stream.skip(other),
                        }
                    }
                    fn encode(&self, stream: &mut binformat::OutputStream) {
                        binformat::emit_single(
                            &self.value,
                            10u32,
                            stream,
                            binformat::OutputStream::bytes,
                        );
                    }
                }

                impl textformat::TextProto for BytesValue {
                    fn merge_field(
                        &mut self,
                        stream: &mut textformat::InputStream,
                    ) -> textformat::Result<()> {
                        match stream.field() {
                            "value" => self.value.merge_text(stream),
                            name => textformat::unknown(name),
                        }
                    }
                    fn encode(&self, stream: &mut textformat::OutputStream) {
                        stream.emit_field("value", &self.value);
                    }
                }
            }

            pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
                r#struct::register_types(registry);
                any::register_types(registry);
                field_mask::register_types(registry);
                timestamp::register_types(registry);
                duration::register_types(registry);
                wrappers::register_types(registry);
            }
        }

        pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
            protobuf::register_types(registry);
        }
    }

    pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
        conformance::register_types(registry);
        protobuf_test_messages::register_types(registry);
        google::register_types(registry);
    }
}

use gen::protobuf_test_messages::proto2::test_messages_proto2::TestAllTypesProto2;
use gen::protobuf_test_messages::proto3::test_messages_proto3::TestAllTypesProto3;

enum Output {
    Proto2(TestAllTypesProto2),
    Proto3(TestAllTypesProto3),
}

#[automatically_derived]
impl ::core::fmt::Debug for Output {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Output::Proto2(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Proto2", &__self_0)
            }
            Output::Proto3(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Proto3", &__self_0)
            }
        }
    }
}

fn octal(d: &[u8]) -> String {
    d.iter()
        .map(|v| {
            let res = ::alloc::fmt::format(format_args!("\\{0:0>3o}", v));
            res
        })
        .collect::<Vec<_>>()
        .join("")
}

fn input(payload: ConformanceRequestOneOfPayload, proto3: bool) -> anyhow::Result<Output> {
    let txt;
    let out = match (&payload, proto3) {
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), false) => {
            txt = octal(pb);
            Ok(Output::Proto2(binformat::decode::<TestAllTypesProto2>(
                &pb,
            )?))
        }
        (ConformanceRequestOneOfPayload::ProtobufPayload(pb), true) => {
            txt = octal(pb);
            Ok(Output::Proto3(binformat::decode::<TestAllTypesProto3>(
                &pb,
            )?))
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
        (other, _) => ::core::panicking::panic_fmt(format_args!("Unknown payload {0:?}", other)),
    }?;
    {
        ::std::io::_eprint(format_args!("Req: {0} => {1:?}\n", txt, out));
    };
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
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (Ok(Output::Proto3(v)), WireFormat::TEXT_FORMAT) => {
            let out = textformat::encode(&v, &Registry::default()).unwrap();
            ConformanceResponseOneOfResult::TextPayload(out)
        }
        (_, WireFormat::JSON) => ConformanceResponseOneOfResult::Skipped("No json".to_string()),
        (Err(e), _) => ConformanceResponseOneOfResult::ParseError(e.to_string()),
        _ => ::core::panicking::panic("explicit panic"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn run_rust(data: *const u8, len: u32, odata: &mut u8, olen: u32) -> u32 {
    let data = from_raw_parts(data, len as usize);
    let req = protokit::binformat::decode::<conformance::ConformanceRequest>(&data).unwrap();
    let out = if let Some(ConformanceRequestOneOfPayload::JsonPayload(_)) = req.payload {
        ConformanceResponse {
            result: Some(ConformanceResponseOneOfResult::Skipped(
                "No json support".to_string(),
            )),
            ..Default::default()
        }
    } else if req.message_type.contains("Proto3") | | req.message_type.contains("Proto2") {
        let out = input(req.payload.unwrap(), req.message_type.contains("Proto3"));
        let data_out = output(out, req.requested_output_format);
        ConformanceResponse {
            result: Some(data_out),
            ..Default::default()
        }
    } else if req.message_type.contains("FailureSet") {
        let fs = FailureSet {
            failure: ::alloc::vec::Vec::new(),
            ..Default::default()
        };
        ConformanceResponse {
            result: Some(ConformanceResponseOneOfResult::ProtobufPayload(
                binformat::encode(&fs).unwrap(),
            )),
            ..Default::default()
        }
    } else {
        ::core::panicking::panic("explicit panic")
    };
    let out = binformat::encode(&out).unwrap();
    let outslice = from_raw_parts_mut(odata, olen as usize);
    outslice[0..out.len()].copy_from_slice(&out);
    return out.len() as u32;
}

extern crate test;

#[cfg(test)]
#[rustc_test_marker = "test1"]
pub const test1: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test1"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::UnitTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test1())),
};

fn test1() {
    let a = binformat::decode::<TestAllTypesProto3>(&[
        0o202, 0o007, 0o014, 0o022, 0o012, 0o010, 0o001, 0o020, 0o001, 0o310, 0o005, 0o001, 0o310,
        0o005, 0o001,
    ])
        .unwrap();
    let b = binformat::encode(&a).unwrap();
}

#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test1])
}