use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum SentenceBreak {
    #[serde(rename = "AT")]
    ATerm,
    #[serde(rename = "CL")]
    Close,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "EX")]
    Extend,
    #[serde(rename = "FO")]
    Format,
    #[serde(rename = "LE")]
    OLetter,
    #[serde(rename = "LF")]
    LF,
    #[serde(rename = "LO")]
    Lower,
    #[serde(rename = "NU")]
    Numeric,
    #[serde(rename = "SC")]
    SContinue,
    #[serde(rename = "SE")]
    Sep,
    #[serde(rename = "SP")]
    Sp,
    #[serde(rename = "ST")]
    STerm,
    #[serde(rename = "UP")]
    Upper,
    #[serde(rename = "XX")]
    Other,
}
