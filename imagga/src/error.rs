use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Imagga(String),
    Reqwest(reqwest::Error),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Imagga(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Imagga(v) => v.clone(),
                Error::Reqwest(v) => v.to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}
