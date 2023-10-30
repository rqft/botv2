use reqwest::StatusCode;
pub use serde;
pub use serde_json;
use std::fmt::Display;

pub mod model;

pub struct Ucd {
    client: reqwest::Client,
}

impl Ucd {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn hex(
        &self,
        codepoint: impl Display,
    ) -> Result<Option<model::codepoint::Codepoint>, reqwest::Error> {
        let res = self
            .client
            .get(format!(
                "https://ucdapi.org/unicode/latest/codepoint/hex/{codepoint}"
            ))
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let j = res.text().await?;

        // dbg!(serde_json::from_str::<serde_json::Value>(&j).unwrap());
        Ok(serde_json::from_str::<model::codepoint::Codepoint>(&j)
            .map(Some)
            .unwrap())
    }

    pub async fn dec(
        &self,
        codepoint: impl Display,
    ) -> Result<Option<model::codepoint::Codepoint>, reqwest::Error> {
        let res = self
            .client
            .get(format!(
                "https://ucdapi.org/unicode/latest/codepoint/dec/{codepoint}"
            ))
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        res.json::<model::codepoint::Codepoint>().await.map(Some)
    }

    pub async fn chars(
        &self,
        chars: impl Display,
    ) -> Result<Vec<model::codepoint::Codepoint>, reqwest::Error> {
        let res = self
            .client
            .get(format!("https://ucdapi.org/unicode/latest/chars/{chars}"))
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(vec![]);
        }

        res.json::<Vec<model::codepoint::Codepoint>>().await
    }

    pub async fn chars_attribute(
        &self,
        chars: impl Display,
        attribute: impl Display,
    ) -> Result<Vec<serde_json::Value>, reqwest::Error> {
        let res = self
            .client
            .get(format!(
                "https://ucdapi.org/unicode/latest/chars/{chars}/{attribute}"
            ))
            .send()
            .await?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(vec![]);
        }

        res.json::<Vec<_>>().await
    }
}

impl Default for Ucd {
    fn default() -> Self {
        Self::new()
    }
}
