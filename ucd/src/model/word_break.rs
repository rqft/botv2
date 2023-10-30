use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum WordBreak {
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "DQ")]
    DoubleQuote,
    #[serde(rename = "EB")]
    EBase,
    #[serde(rename = "EBG")]
    EBaseGAZ,
    #[serde(rename = "EM")]
    EModifier,
    #[serde(rename = "EX")]
    ExtendNumLet,
    #[serde(rename = "Extend")]
    Extend,
    #[serde(rename = "FO")]
    Format,
    #[serde(rename = "GAZ")]
    GlueAfterZwj,
    #[serde(rename = "HL")]
    HebrewLetter,
    #[serde(rename = "KA")]
    Katakana,
    #[serde(rename = "LE")]
    ALetter,
    #[serde(rename = "LF")]
    LF,
    #[serde(rename = "MB")]
    MidNumLet,
    #[serde(rename = "ML")]
    MidLetter,
    #[serde(rename = "MN")]
    MidNum,
    #[serde(rename = "NL")]
    Newline,
    #[serde(rename = "NU")]
    Numeric,
    #[serde(rename = "RI")]
    RegionalIndicator,
    #[serde(rename = "SQ")]
    SingleQuote,
    #[serde(rename = "WSegSpace")]
    WSegSpace,
    #[serde(rename = "XX")]
    Other,
    #[serde(rename = "ZWJ")]
    ZWJ,
}
