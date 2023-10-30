use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
#[rustfmt::skip]
pub enum BidiClass {
    #[serde(rename = "AL")] ArabicLetter,
    #[serde(rename = "AN")] ArabicNumber,
    #[serde(rename = "B")] ParagraphSeparator,
    #[serde(rename = "BN")] BoundaryNeutral,
    #[serde(rename = "CS")] CommonSeparator,
    #[serde(rename = "EN")] EuropeanNumber,
    #[serde(rename = "ES")] EuropeanSeparator,
    #[serde(rename = "ET")] EuropeanTerminator,
    #[serde(rename = "FSI")] FirstStrongIsolate,
    #[serde(rename = "L")] LeftToRight,
    #[serde(rename = "LRE")] LeftToRightEmbedding,
    #[serde(rename = "LRI")] LeftToRightIsolate,
    #[serde(rename = "LRO")] LeftToRightOverride,
    #[serde(rename = "NSM")] NonspacingMark,
    #[serde(rename = "ON")] OtherNeutral,
    #[serde(rename = "PDF")] PopDirectionalFormat,
    #[serde(rename = "PDI")] PopDirectionalIsolate,
    #[serde(rename = "R")] RightToLeft,
    #[serde(rename = "RLE")] RightToLeftEmbedding,
    #[serde(rename = "RLI")] RightToLeftIsolate,
    #[serde(rename = "RLO")] RightToLeftOverride,
    #[serde(rename = "S")] SegmentSeparator,
    #[serde(rename = "WS")] WhiteSpace,
}
