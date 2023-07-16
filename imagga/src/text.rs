use serde::{Deserialize, Serialize};

use crate::coordinates::Coordinates;

#[derive(Serialize)]
pub struct TextOptions {
    pub image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub text: Vec<TextContent>,
}

#[derive(Serialize, Deserialize)]
pub struct TextContent {
    pub data: String,
    pub coordinates: Coordinates,
}
