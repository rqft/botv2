use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ColorsOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
    pub extract_overall_colors: u8,
    pub extract_object_colors: u8,
    pub overall_count: u8,
    pub separated_count: u8,
    pub deterministic: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Colors {
    pub colors: ColorsBody,
}

#[derive(Serialize, Deserialize)]
pub struct ColorsBody {
    pub object_percentage: f64,
    pub color_percent_threshold: f64,
    pub color_variance: f64,

    pub background_colors: Option<Vec<Color>>,
    pub foreground_colors: Option<Vec<Colors>>,
    pub image_colors: Option<Vec<Color>>,
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub closest_palette_color: String,
    pub closest_palette_color_html_code: String,
    pub closest_palette_color_parent: String,
    pub closest_pallete_distance: f64,
    pub html_code: String,
    pub percent: f64,
}
