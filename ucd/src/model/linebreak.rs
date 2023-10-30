use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum Linebreak {
    #[serde(rename = "AI")]
    Ambiguous,
    #[serde(rename = "AK")]
    Aksara,
    #[serde(rename = "AL")]
    Alphabetic,
    #[serde(rename = "AP")]
    AksaraPrebase,
    #[serde(rename = "AS")]
    AksaraStart,
    #[serde(rename = "B2")]
    BreakBoth,
    #[serde(rename = "BA")]
    BreakAfter,
    #[serde(rename = "BB")]
    BreakBefore,
    #[serde(rename = "BK")]
    MandatoryBreak,
    #[serde(rename = "CB")]
    ContingentBreak,
    #[serde(rename = "CJ")]
    ConditionalJapaneseStarter,
    #[serde(rename = "CL")]
    ClosePunctuation,
    #[serde(rename = "CM")]
    CombiningMark,
    #[serde(rename = "CP")]
    CloseParenthesis,
    #[serde(rename = "CR")]
    CarriageReturn,
    #[serde(rename = "EB")]
    EBase,
    #[serde(rename = "EM")]
    EModifier,
    #[serde(rename = "EX")]
    Exclamation,
    #[serde(rename = "GL")]
    Glue,
    #[serde(rename = "H2")]
    H2,
    #[serde(rename = "H3")]
    H3,
    #[serde(rename = "HL")]
    HebrewLetter,
    #[serde(rename = "HY")]
    Hyphen,
    #[serde(rename = "ID")]
    Ideographic,
    #[serde(rename = "IN")]
    Inseparable,
    #[serde(rename = "IS")]
    InfixNumeric,
    #[serde(rename = "JL")]
    JL,
    #[serde(rename = "JT")]
    JT,
    #[serde(rename = "JV")]
    JV,
    #[serde(rename = "LF")]
    LineFeed,
    #[serde(rename = "NL")]
    NextLine,
    #[serde(rename = "NS")]
    Nonstarter,
    #[serde(rename = "NU")]
    Numeric,
    #[serde(rename = "OP")]
    OpenPunctuation,
    #[serde(rename = "PO")]
    PostfixNumeric,
    #[serde(rename = "PR")]
    PrefixNumeric,
    #[serde(rename = "QU")]
    Quotation,
    #[serde(rename = "RI")]
    RegionalIndicator,
    #[serde(rename = "SA")]
    ComplexContext,
    #[serde(rename = "SG")]
    Surrogate,
    #[serde(rename = "SP")]
    Space,
    #[serde(rename = "SY")]
    BreakSymbols,
    #[serde(rename = "VF")]
    ViramaFinal,
    #[serde(rename = "VI")]
    Virama,
    #[serde(rename = "WJ")]
    WordJoiner,
    #[serde(rename = "XX")]
    Unknown,
    #[serde(rename = "ZW")]
    ZWSpace,
    #[serde(rename = "ZWJ")]
    ZWJ,
}
