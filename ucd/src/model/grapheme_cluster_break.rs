use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum GraphemeClusterBreak {
    #[serde(rename = "CN")]
    Control,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "EB")]
    EBase,
    #[serde(rename = "EBG")]
    EBaseGAZ,
    #[serde(rename = "EM")]
    EModifier,
    #[serde(rename = "EX")]
    Extend,
    #[serde(rename = "GAZ")]
    GlueAfterZwj,
    #[serde(rename = "L")]
    L,
    #[serde(rename = "LF")]
    LF,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "LVT")]
    LVT,
    #[serde(rename = "PP")]
    Prepend,
    #[serde(rename = "RI")]
    RegionalIndicator,
    #[serde(rename = "SM")]
    SpacingMark,
    #[serde(rename = "T")]
    T,
    #[serde(rename = "V")]
    V,
    #[serde(rename = "XX")]
    Other,
    #[serde(rename = "ZWJ")]
    ZWJ,
}
