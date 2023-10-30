use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum DecompositionType {
    #[serde(rename = "can")]
    Canonical,
    #[serde(rename = "com")]
    Compat,
    #[serde(rename = "enc")]
    Circle,
    #[serde(rename = "fin")]
    Final,
    #[serde(rename = "font")]
    Font,
    #[serde(rename = "fra")]
    Fraction,
    #[serde(rename = "init")]
    Initial,
    #[serde(rename = "iso")]
    Isolated,
    #[serde(rename = "med")]
    Medial,
    #[serde(rename = "nar")]
    Narrow,
    #[serde(rename = "nb")]
    Nobreak,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sml")]
    Small,
    #[serde(rename = "sqr")]
    Square,
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "sup")]
    Super,
    #[serde(rename = "vert")]
    Vertical,
    #[serde(rename = "wide")]
    Wide,
}
