use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum HangulSyllableType {
    #[serde(rename = "L")]
    LeadingJamo,
    #[serde(rename = "LV")]
    LVSyllable,
    #[serde(rename = "LVT")]
    LVTSyllable,
    #[serde(rename = "NA")]
    NotApplicable,
    #[serde(rename = "T")]
    TrailingJamo,
    #[serde(rename = "V")]
    VowelJamo,
}
