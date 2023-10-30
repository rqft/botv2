use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum YN {
    Y,
    N,
}

impl From<bool> for YN {
    fn from(value: bool) -> Self {
        if value {
            YN::Y
        } else {
            YN::N
        }
    }
}

impl From<YN> for bool {
    fn from(value: YN) -> Self {
        value == YN::Y
    }
}
