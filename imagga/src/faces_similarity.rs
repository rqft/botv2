use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct FacesSimilarityOptions {
    pub face_id: String,
    pub second_face_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FacesSimilarity {
    pub score: f64,
}
