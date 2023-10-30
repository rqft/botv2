use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum JamoShortName {
    #[serde(rename = "")]
    None,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "AE")]
    AE,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "BB")]
    BB,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "DD")]
    DD,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "EO")]
    EO,
    #[serde(rename = "EU")]
    EU,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "GG")]
    GG,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "H")]
    H,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "J")]
    J,
    #[serde(rename = "JJ")]
    JJ,
    #[serde(rename = "K")]
    K,
    #[serde(rename = "L")]
    L,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "LG")]
    LG,
    #[serde(rename = "LH")]
    LH,
    #[serde(rename = "LM")]
    LM,
    #[serde(rename = "LP")]
    LP,
    #[serde(rename = "LS")]
    LS,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "N")]
    N,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NH")]
    NH,
    #[serde(rename = "NJ")]
    NJ,
    #[serde(rename = "O")]
    O,
    #[serde(rename = "OE")]
    OE,
    #[serde(rename = "P")]
    P,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "SS")]
    SS,
    #[serde(rename = "T")]
    T,
    #[serde(rename = "U")]
    U,
    #[serde(rename = "WA")]
    WA,
    #[serde(rename = "WAE")]
    WAE,
    #[serde(rename = "WE")]
    WE,
    #[serde(rename = "WEO")]
    WEO,
    #[serde(rename = "WI")]
    WI,
    #[serde(rename = "YA")]
    YA,
    #[serde(rename = "YAE")]
    YAE,
    #[serde(rename = "YE")]
    YE,
    #[serde(rename = "YEO")]
    YEO,
    #[serde(rename = "YI")]
    YI,
    #[serde(rename = "YO")]
    YO,
    #[serde(rename = "YU")]
    YU,
}
