use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum GeneralCategory {
    #[serde(rename = "Cc")]
    Control,
    #[serde(rename = "Cf")]
    Format,
    #[serde(rename = "Co")]
    PrivateUse,
    #[serde(rename = "Cs")]
    Surrrogate,
    #[serde(rename = "Ll")]
    LowercaseLetter,
    #[serde(rename = "Lm")]
    ModifierLetter,
    #[serde(rename = "Lo")]
    OtherLetter,
    #[serde(rename = "Lt")]
    TitlecaseLetter,
    #[serde(rename = "Lu")]
    UppercaseLetter,
    #[serde(rename = "Mc")]
    SpacingMark,
    #[serde(rename = "Me")]
    EnclosingMark,
    #[serde(rename = "Mn")]
    NonspacingMark,
    #[serde(rename = "Nd")]
    DecimalNumber,
    #[serde(rename = "Nl")]
    LetterNumber,
    #[serde(rename = "No")]
    OtherNumber,
    #[serde(rename = "Pc")]
    ConnectorPunctuation,
    #[serde(rename = "Pd")]
    DashPunctuation,
    #[serde(rename = "Pe")]
    ClosePunctuation,
    #[serde(rename = "Pf")]
    FinalPunctuation,
    #[serde(rename = "Pi")]
    InitialPunctuation,
    #[serde(rename = "Po")]
    OtherPunctuation,
    #[serde(rename = "Ps")]
    OpenPunctuation,
    #[serde(rename = "Sc")]
    CurrencySymbol,
    #[serde(rename = "Sk")]
    ModifierSymbol,
    #[serde(rename = "Sm")]
    MathSymbol,
    #[serde(rename = "So")]
    OtherSymbol,
    #[serde(rename = "Zl")]
    LineSeparator,
    #[serde(rename = "Zp")]
    ParagraphSeparator,
    #[serde(rename = "Zs")]
    SpaceSeparator,
}
