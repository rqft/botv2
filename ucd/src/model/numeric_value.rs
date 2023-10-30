use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct NumericValue {
    pub numerator: u32,
    pub denominator: u32,
}
