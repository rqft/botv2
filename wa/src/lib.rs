use std::fmt::Display;

use model::{QueryOptions, QueryOutputHolder};
use reqwest::StatusCode;

pub mod error;
pub mod model;
pub mod util;
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

    pub async fn query(&self, options: QueryOptions) -> Result<QueryOutputHolder, error::Error> {
        let r = self
            .client
            .get("https://api.wolframalpha.com/v2/query")
            .query(&options)
            .query(&[("appid", &self.app_id), ("output", &"json".to_string())])
            .build()?;
        dbg!(r.url());
        let v = self.client.execute(r).await?;
        // let y = r.url().clone();

        v.json::<serde_json::Value>()
            .await
            .map_err(error::Error::from)
            .and_then(|x| serde_json::from_value(x).map_err(|y| y.into()))
    }

    pub async fn short_answer(
        &self,
        input: impl Display,
        units: impl Display,
    ) -> Result<Option<String>, error::Error> {
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

        r.text().await.map(Some).map_err(|x| x.into())
    }
}
