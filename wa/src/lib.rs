use std::fmt::Display;

use model::{QueryOptions, QueryOutputHolder};
use reqwest::StatusCode;

pub mod model;
pub struct Wolfram {
    app_id: String,
    client: reqwest::Client,
}

impl Wolfram {
    pub fn new(app_id: String) -> Self {
        Self {
            app_id,
            client: reqwest::Client::new(),
        }
    }

    pub async fn query(&self, options: QueryOptions) -> Result<QueryOutputHolder, reqwest::Error> {
        self.client
            .get("https://api.wolframalpha.com/v2/query")
            .query(&options)
            .query(&[("appid", &self.app_id), ("output", &"json".to_string())])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn short_answer(
        &self,
        input: impl Display,
        units: impl Display,
    ) -> Result<Option<String>, reqwest::Error> {
        let r = self
            .client
            .get("https://api.wolframalpha.com/v1/result")
            .query(&[
                ("appid", &self.app_id),
                ("i", &format!("{input}")),
                ("units", &format!("{units}")),
            ])
            .send()
            .await?;

        if r.status() == StatusCode::NOT_IMPLEMENTED {
            return Ok(None);
        }

        r.text().await.map(Some)
    }
}
