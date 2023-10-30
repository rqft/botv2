use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum BidiPairedBracketType {
    #[serde(rename = "c")]
    Close,
    #[serde(rename = "o")]
    Open,
    #[serde(rename = "n")]
    None,
}
