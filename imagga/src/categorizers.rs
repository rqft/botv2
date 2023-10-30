use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Categorizers {
    pub categorizers: Vec<Categorizer>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Categorizer {
    pub id: String,
    pub labels: Vec<String>,
    pub title: String,
}
