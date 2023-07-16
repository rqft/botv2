use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub width: u32,
    pub height: u32,
    pub xmin: u32,
    pub ymin: u32,
    pub xmax: u32,
    pub ymax: u32,
}

