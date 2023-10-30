use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct CroppingsOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
    pub resolution: Option<String>,
    pub no_scaling: Option<u8>,
    pub rect_percentage: Option<f64>,
    // i dont know how to recieve raster image, crop it yourself
    // image_result: Option<u8>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Croppings {
    pub croppings: Vec<Cropping>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cropping {
    pub target_height: u32,
    pub target_width: u32,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}
