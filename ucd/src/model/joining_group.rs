use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum JoiningGroup {
    #[serde(rename = "African_Feh")]
    AfricanFeh,
    #[serde(rename = "African_Noon")]
    AfricanNoon,
    #[serde(rename = "African_Qaf")]
    AfricanQaf,
    #[serde(rename = "Ain")]
    Ain,
    #[serde(rename = "Alaph")]
    Alaph,
    #[serde(rename = "Alef")]
    Alef,
    #[serde(rename = "Beh")]
    Beh,
    #[serde(rename = "Beth")]
    Beth,
    #[serde(rename = "Burushaski_Yeh_Barree")]
    BurushaskiYehBarree,
    #[serde(rename = "Dal")]
    Dal,
    #[serde(rename = "Dalath_Rish")]
    DalathRish,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "Farsi_Yeh")]
    FarsiYeh,
    #[serde(rename = "Fe")]
    Fe,
    #[serde(rename = "Feh")]
    Feh,
    #[serde(rename = "Final_Semkath")]
    FinalSemkath,
    #[serde(rename = "Gaf")]
    Gaf,
    #[serde(rename = "Gamal")]
    Gamal,
    #[serde(rename = "Hah")]
    Hah,
    #[serde(rename = "Hanifi_Rohingya_Kinna_Ya")]
    HanifiRohingyaKinnaYa,
    #[serde(rename = "Hanifi_Rohingya_Pa")]
    HanifiRohingyaPa,
    #[serde(rename = "He")]
    He,
    #[serde(rename = "Heh")]
    Heh,
    #[serde(rename = "Heh_Goal")]
    HehGoal,
    #[serde(rename = "Heth")]
    Heth,
    #[serde(rename = "Kaf")]
    Kaf,
    #[serde(rename = "Kaph")]
    Kaph,
    #[serde(rename = "Khaph")]
    Khaph,
    #[serde(rename = "Knotted_Heh")]
    KnottedHeh,
    #[serde(rename = "Lam")]
    Lam,
    #[serde(rename = "Lamadh")]
    Lamadh,
    #[serde(rename = "Malayalam_Bha")]
    MalayalamBha,
    #[serde(rename = "Malayalam_Ja")]
    MalayalamJa,
    #[serde(rename = "Malayalam_Lla")]
    MalayalamLla,
    #[serde(rename = "Malayalam_Llla")]
    MalayalamLlla,
    #[serde(rename = "Malayalam_Nga")]
    MalayalamNga,
    #[serde(rename = "Malayalam_Nna")]
    MalayalamNna,
    #[serde(rename = "Malayalam_Nnna")]
    MalayalamNnna,
    #[serde(rename = "Malayalam_Nya")]
    MalayalamNya,
    #[serde(rename = "Malayalam_Ra")]
    MalayalamRa,
    #[serde(rename = "Malayalam_Ssa")]
    MalayalamSsa,
    #[serde(rename = "Malayalam_Tta")]
    MalayalamTta,
    #[serde(rename = "Manichaean_Aleph")]
    ManichaeanAleph,
    #[serde(rename = "Manichaean_Ayin")]
    ManichaeanAyin,
    #[serde(rename = "Manichaean_Beth")]
    ManichaeanBeth,
    #[serde(rename = "Manichaean_Daleth")]
    ManichaeanDaleth,
    #[serde(rename = "Manichaean_Dhamedh")]
    ManichaeanDhamedh,
    #[serde(rename = "Manichaean_Five")]
    ManichaeanFive,
    #[serde(rename = "Manichaean_Gimel")]
    ManichaeanGimel,
    #[serde(rename = "Manichaean_Heth")]
    ManichaeanHeth,
    #[serde(rename = "Manichaean_Hundred")]
    ManichaeanHundred,
    #[serde(rename = "Manichaean_Kaph")]
    ManichaeanKaph,
    #[serde(rename = "Manichaean_Lamedh")]
    ManichaeanLamedh,
    #[serde(rename = "Manichaean_Mem")]
    ManichaeanMem,
    #[serde(rename = "Manichaean_Nun")]
    ManichaeanNun,
    #[serde(rename = "Manichaean_One")]
    ManichaeanOne,
    #[serde(rename = "Manichaean_Pe")]
    ManichaeanPe,
    #[serde(rename = "Manichaean_Qoph")]
    ManichaeanQoph,
    #[serde(rename = "Manichaean_Resh")]
    ManichaeanResh,
    #[serde(rename = "Manichaean_Sadhe")]
    ManichaeanSadhe,
    #[serde(rename = "Manichaean_Samekh")]
    ManichaeanSamekh,
    #[serde(rename = "Manichaean_Taw")]
    ManichaeanTaw,
    #[serde(rename = "Manichaean_Ten")]
    ManichaeanTen,
    #[serde(rename = "Manichaean_Teth")]
    ManichaeanTeth,
    #[serde(rename = "Manichaean_Thamedh")]
    ManichaeanThamedh,
    #[serde(rename = "Manichaean_Twenty")]
    ManichaeanTwenty,
    #[serde(rename = "Manichaean_Waw")]
    ManichaeanWaw,
    #[serde(rename = "Manichaean_Yodh")]
    ManichaeanYodh,
    #[serde(rename = "Manichaean_Zayin")]
    ManichaeanZayin,
    #[serde(rename = "Meem")]
    Meem,
    #[serde(rename = "Mim")]
    Mim,
    #[serde(rename = "No_Joining_Group")]
    NoJoiningGroup,
    #[serde(rename = "Noon")]
    Noon,
    #[serde(rename = "Nun")]
    Nun,
    #[serde(rename = "Nya")]
    Nya,
    #[serde(rename = "Pe")]
    Pe,
    #[serde(rename = "Qaf")]
    Qaf,
    #[serde(rename = "Qaph")]
    Qaph,
    #[serde(rename = "Reh")]
    Reh,
    #[serde(rename = "Reversed_Pe")]
    ReversedPe,
    #[serde(rename = "Rohingya_Yeh")]
    RohingyaYeh,
    #[serde(rename = "Sad")]
    Sad,
    #[serde(rename = "Sadhe")]
    Sadhe,
    #[serde(rename = "Seen")]
    Seen,
    #[serde(rename = "Semkath")]
    Semkath,
    #[serde(rename = "Shin")]
    Shin,
    #[serde(rename = "Straight_Waw")]
    StraightWaw,
    #[serde(rename = "Swash_Kaf")]
    SwashKaf,
    #[serde(rename = "Syriac_Waw")]
    SyriacWaw,
    #[serde(rename = "Tah")]
    Tah,
    #[serde(rename = "Taw")]
    Taw,
    #[serde(rename = "Teh_Marbuta")]
    TehMarbuta,
    #[serde(rename = "Teh_Marbuta_Goal")]
    HamzaOnHehGoal,
    #[serde(rename = "Teth")]
    Teth,
    #[serde(rename = "Thin_Yeh")]
    ThinYeh,
    #[serde(rename = "Vertical_Tail")]
    VerticalTail,
    #[serde(rename = "Waw")]
    Waw,
    #[serde(rename = "Yeh")]
    Yeh,
    #[serde(rename = "Yeh_Barree")]
    YehBarree,
    #[serde(rename = "Yeh_With_Tail")]
    YehWithTail,
    #[serde(rename = "Yudh")]
    Yudh,
    #[serde(rename = "Yudh_He")]
    YudhHe,
    #[serde(rename = "Zain")]
    Zain,
    #[serde(rename = "Zhain")]
    Zhain,
}
