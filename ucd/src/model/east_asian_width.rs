use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum EastAsianWidth {
    #[serde(rename = "A")]
    Ambiguous,
    #[serde(rename = "F")]
    Fullwidth,
    #[serde(rename = "H")]
    Halfwidth,
    #[serde(rename = "N")]
    Neutral,
    #[serde(rename = "Na")]
    Narrow,
    #[serde(rename = "W")]
    Wide,
}
