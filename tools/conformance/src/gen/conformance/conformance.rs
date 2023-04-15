#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WireFormat(pub u32);
#[protoenum]
impl WireFormat {
    #[var(0u32, "UNSPECIFIED")]
    pub const UNSPECIFIED: WireFormat = WireFormat(0u32);
    #[var(1u32, "PROTOBUF")]
    pub const PROTOBUF: WireFormat = WireFormat(1u32);
    #[var(2u32, "JSON")]
    pub const JSON: WireFormat = WireFormat(2u32);
    #[var(3u32, "JSPB")]
    pub const JSPB: WireFormat = WireFormat(3u32);
    #[var(4u32, "TEXT_FORMAT")]
    pub const TEXT_FORMAT: WireFormat = WireFormat(4u32);
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestCategory(pub u32);
#[protoenum]
impl TestCategory {
    #[var(0u32, "UNSPECIFIED_TEST")]
    pub const UNSPECIFIED_TEST: TestCategory = TestCategory(0u32);
    #[var(1u32, "BINARY_TEST")]
    pub const BINARY_TEST: TestCategory = TestCategory(1u32);
    #[var(2u32, "JSON_TEST")]
    pub const JSON_TEST: TestCategory = TestCategory(2u32);
    #[var(3u32, "JSON_IGNORE_UNKNOWN_PARSING_TEST")]
    pub const JSON_IGNORE_UNKNOWN_PARSING_TEST: TestCategory = TestCategory(3u32);
    #[var(4u32, "JSPB_TEST")]
    pub const JSPB_TEST: TestCategory = TestCategory(4u32);
    #[var(5u32, "TEXT_FORMAT_TEST")]
    pub const TEXT_FORMAT_TEST: TestCategory = TestCategory(5u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct FailureSet {
    #[field(1u32, "failure", string, repeated)]
    pub failure: Vec<String>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum ConformanceRequestOneOfPayload {
    #[field(1u32, "protobuf_payload", bytes, raw)]
    ProtobufPayload(Vec<u8>),
    #[field(2u32, "json_payload", string, raw)]
    JsonPayload(String),
    #[field(7u32, "jspb_payload", string, raw)]
    JspbPayload(String),
    #[field(8u32, "text_payload", string, raw)]
    TextPayload(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ConformanceRequestOneOfPayload {
    fn default() -> Self {
        Self::ProtobufPayload(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
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
    #[oneof(
        [1u32,
        2u32,
        7u32,
        8u32,
        ],
        ["protobuf_payload",
        "json_payload",
        "jspb_payload",
        "text_payload",
        ]
    )]
    pub payload: Option<ConformanceRequestOneOfPayload>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Clone, PartialEq, Proto)]
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
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ConformanceResponseOneOfResult {
    fn default() -> Self {
        Self::ParseError(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ConformanceResponse {
    #[oneof(
        [1u32,
        6u32,
        9u32,
        2u32,
        3u32,
        4u32,
        5u32,
        7u32,
        8u32,
        ],
        ["parse_error",
        "serialize_error",
        "timeout_error",
        "runtime_error",
        "protobuf_payload",
        "json_payload",
        "skipped",
        "jspb_payload",
        "text_payload",
        ]
    )]
    pub result: Option<ConformanceResponseOneOfResult>,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct JspbEncodingConfig {
    #[field(1u32, "use_jspb_array_any_format", bool, singular)]
    pub use_jspb_array_any_format: bool,
    #[unknown]
    pub unknown: binformat::UnknownFieldsOwned,
}
