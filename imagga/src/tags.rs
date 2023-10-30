use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone)]
pub struct TagsOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
    // developer note: english.
    // language: Option<String>,
    pub verbose: Option<u8>,
    pub limit: Option<u64>,
    pub threshold: Option<f64>,
    pub decrease_parents: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawTags {
    pub tags: Vec<RawTag>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawTag {
    pub confidence: f64,
    pub tag: RawTagContent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawTagContent {
    // d-tag language
    pub en: String,
}

#[derive(Clone)]
pub struct Tags {
    pub tags: HashMap<String, f64>,
}

impl From<RawTags> for Tags {
    fn from(value: RawTags) -> Self {
        let mut tags = HashMap::new();

        for tag in value.tags {
            tags.insert(tag.tag.en, tag.confidence);
        }

        Self { tags }
    }
}
