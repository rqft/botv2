use serde::{Deserialize, Serialize};

use crate::coordinates::Coordinates;

#[derive(Serialize, Clone)]
pub struct FacesDetectionsOptions {
    pub image_url: Option<String>,
    pub image_upload_id: Option<String>,
    pub return_face_id: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FacesDetections {
    pub faces: Vec<FaceDetection>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FaceDetection {
    pub confidence: f64,
    pub coordinates: Coordinates,
    pub face_id: String,
}
