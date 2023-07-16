use model::{QueryOptions, QueryOutputHolder};

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
}
