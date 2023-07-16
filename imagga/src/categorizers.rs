use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Categorizers {
    pub categorizers: Vec<Categorizer>,
}

#[derive(Serialize, Deserialize)]
pub struct Categorizer {
    pub id: String,
    pub labels: Vec<String>,
    pub title: String,
}
