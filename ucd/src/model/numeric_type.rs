use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum NumericType {
    #[serde(rename = "De")]
    Decimal,
    #[serde(rename = "Di")]
    Digit,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Nu")]
    Numeric,
}
