use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct BarcodesOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Barcodes {
    pub barcodes: Vec<Barcode>,
}

#[derive(Serialize, Deserialize)]
pub struct Barcode {
    pub data: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
}
