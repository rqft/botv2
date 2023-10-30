use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum JoiningType {
    #[serde(rename = "C")]
    JoinCausing,
    #[serde(rename = "D")]
    DualJoining,
    #[serde(rename = "L")]
    LeftJoining,
    #[serde(rename = "R")]
    RightJoining,
    #[serde(rename = "T")]
    Transparent,
    #[serde(rename = "U")]
    NonJoining,
}
