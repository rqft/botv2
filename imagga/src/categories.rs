use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Clone)]
pub struct CategoriesOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RawCategories {
    pub categories: Vec<RawCategory>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RawCategory {
    pub confidence: f64,
    pub name: RawCategoryContent,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RawCategoryContent {
    // d-tag language
    pub en: String,
}

#[derive(Debug, Default, Clone)]
pub struct Categories {
    pub categories: HashMap<String, f64>,
}

impl From<RawCategories> for Categories {
    fn from(value: RawCategories) -> Self {
        let mut categories = HashMap::new();

        for category in value.categories {
            categories.insert(category.name.en, category.confidence);
        }

        Self { categories }
    }
}
