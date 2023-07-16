use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct RawOutput<T> {
    pub result: T,
    pub status: RawStatus,
}

impl<T> RawOutput<T> {
    pub fn result(self) -> Result<T, Error> {
        match self.status.kind {
            RawStatusKind::Error => Err(Error::Imagga(self.status.text)),
            RawStatusKind::Success => Ok(self.result),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RawStatus {
    pub text: String,
    #[serde(rename = "type")]
    pub kind: RawStatusKind,
}

#[derive(Debug)]
pub enum RawStatusKind {
    Success,
    Error,
}

impl Serialize for RawStatusKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Error => "error",
            Self::Success => "success",
        })
    }
}

impl<'a> Deserialize<'a> for RawStatusKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        use serde::de::Error;
        match String::deserialize(deserializer)?.as_str() {
            "error" => Ok(RawStatusKind::Error),
            "success" => Ok(RawStatusKind::Success),
            c => Err(D::Error::custom(c)),
        }
    }
}
