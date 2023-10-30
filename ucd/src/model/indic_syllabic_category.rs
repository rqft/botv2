use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum IndicSyllabicCategory {
    #[serde(rename = "Avagraha")]
    Avagraha,
    #[serde(rename = "Bindu")]
    Bindu,
    #[serde(rename = "Brahmi_Joining_Number")]
    BrahmiJoiningNumber,
    #[serde(rename = "Cantillation_Mark")]
    CantillationMark,
    #[serde(rename = "Consonant")]
    Consonant,
    #[serde(rename = "Consonant_Dead")]
    ConsonantDead,
    #[serde(rename = "Consonant_Final")]
    ConsonantFinal,
    #[serde(rename = "Consonant_Head_Letter")]
    ConsonantHeadLetter,
    #[serde(rename = "Consonant_Initial_Postfixed")]
    ConsonantInitialPostfixed,
    #[serde(rename = "Consonant_Killer")]
    ConsonantKiller,
    #[serde(rename = "Consonant_Medial")]
    ConsonantMedial,
    #[serde(rename = "Consonant_Placeholder")]
    ConsonantPlaceholder,
    #[serde(rename = "Consonant_Preceding_Repha")]
    ConsonantPrecedingRepha,
    #[serde(rename = "Consonant_Prefixed")]
    ConsonantPrefixed,
    #[serde(rename = "Consonant_Subjoined")]
    ConsonantSubjoined,
    #[serde(rename = "Consonant_Succeeding_Repha")]
    ConsonantSucceedingRepha,
    #[serde(rename = "Consonant_With_Stacker")]
    ConsonantWithStacker,
    #[serde(rename = "Gemination_Mark")]
    GeminationMark,
    #[serde(rename = "Invisible_Stacker")]
    InvisibleStacker,
    #[serde(rename = "Joiner")]
    Joiner,
    #[serde(rename = "Modifying_Letter")]
    ModifyingLetter,
    #[serde(rename = "Non_Joiner")]
    NonJoiner,
    #[serde(rename = "Nukta")]
    Nukta,
    #[serde(rename = "Number")]
    Number,
    #[serde(rename = "Number_Joiner")]
    NumberJoiner,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Pure_Killer")]
    PureKiller,
    #[serde(rename = "Register_Shifter")]
    RegisterShifter,
    #[serde(rename = "Syllable_Modifier")]
    SyllableModifier,
    #[serde(rename = "Tone_Letter")]
    ToneLetter,
    #[serde(rename = "Tone_Mark")]
    ToneMark,
    #[serde(rename = "Virama")]
    Virama,
    #[serde(rename = "Visarga")]
    Visarga,
    #[serde(rename = "Vowel")]
    Vowel,
    #[serde(rename = "Vowel_Dependent")]
    VowelDependent,
    #[serde(rename = "Vowel_Independent")]
    VowelIndependent,
}
