
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// Single value
    One(T),
    /// Array of values
    Vec(Vec<T>),
}

impl<T> OneOrMany<T> {
    pub fn into_values(self) -> Vec<T> {
        match self {
            Self::One(val) => vec![val],
            Self::Vec(vec) => vec,
        }
    }

    pub fn values(&self) -> Vec<&T> {
        match self {
            Self::One(val) => vec![&val],
            Self::Vec(vec) => vec.iter().collect::<Vec<_>>(),
        }
    }

    pub fn values_mut(&mut self) -> Vec<&mut T> {
        match self {
            Self::One(val) => vec![val],
            Self::Vec(vec) => vec.iter_mut().collect::<Vec<_>>(),
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<&T> {
        self.values().into_iter()
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.into_values().into_iter()
    }
}
