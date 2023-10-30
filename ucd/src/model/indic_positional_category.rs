use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum IndicPositionalCategory {
    #[serde(rename = "Bottom")]
    Bottom,
    #[serde(rename = "Bottom_And_Left")]
    BottomAndLeft,
    #[serde(rename = "Bottom_And_Right")]
    BottomAndRight,
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Left_And_Right")]
    LeftAndRight,
    #[serde(rename = "NA")]
    NA,
    #[serde(rename = "Overstruck")]
    Overstruck,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "Top")]
    Top,
    #[serde(rename = "Top_And_Bottom")]
    TopAndBottom,
    #[serde(rename = "Top_And_Bottom_And_Left")]
    TopAndBottomAndLeft,
    #[serde(rename = "Top_And_Bottom_And_Right")]
    TopAndBottomAndRight,
    #[serde(rename = "Top_And_Left")]
    TopAndLeft,
    #[serde(rename = "Top_And_Left_And_Right")]
    TopAndLeftAndRight,
    #[serde(rename = "Top_And_Right")]
    TopAndRight,
    #[serde(rename = "Visual_Order_Left")]
    VisualOrderLeft,
}
