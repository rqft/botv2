use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum Script {
    #[serde(rename = "Adlm")]
    Adlam,
    #[serde(rename = "Aghb")]
    CaucasianAlbanian,
    #[serde(rename = "Ahom")]
    Ahom,
    #[serde(rename = "Arab")]
    Arabic,
    #[serde(rename = "Armi")]
    ImperialAramaic,
    #[serde(rename = "Armn")]
    Armenian,
    #[serde(rename = "Avst")]
    Avestan,
    #[serde(rename = "Bali")]
    Balinese,
    #[serde(rename = "Bamu")]
    Bamum,
    #[serde(rename = "Bass")]
    BassaVah,
    #[serde(rename = "Batk")]
    Batak,
    #[serde(rename = "Beng")]
    Bengali,
    #[serde(rename = "Bhks")]
    Bhaiksuki,
    #[serde(rename = "Bopo")]
    Bopomofo,
    #[serde(rename = "Brah")]
    Brahmi,
    #[serde(rename = "Brai")]
    Braille,
    #[serde(rename = "Bugi")]
    Buginese,
    #[serde(rename = "Buhd")]
    Buhid,
    #[serde(rename = "Cakm")]
    Chakma,
    #[serde(rename = "Cans")]
    CanadianAboriginal,
    #[serde(rename = "Cari")]
    Carian,
    #[serde(rename = "Cham")]
    Cham,
    #[serde(rename = "Cher")]
    Cherokee,
    #[serde(rename = "Chrs")]
    Chorasmian,
    #[serde(rename = "Copt")]
    Coptic,
    #[serde(rename = "Cpmn")]
    CyproMinoan,
    #[serde(rename = "Cprt")]
    Cypriot,
    #[serde(rename = "Cyrl")]
    Cyrillic,
    #[serde(rename = "Deva")]
    Devanagari,
    #[serde(rename = "Diak")]
    DivesAkuru,
    #[serde(rename = "Dogr")]
    Dogra,
    #[serde(rename = "Dsrt")]
    Deseret,
    #[serde(rename = "Dupl")]
    Duployan,
    #[serde(rename = "Egyp")]
    EgyptianHieroglyphs,
    #[serde(rename = "Elba")]
    Elbasan,
    #[serde(rename = "Elym")]
    Elymaic,
    #[serde(rename = "Ethi")]
    Ethiopic,
    #[serde(rename = "Geor")]
    Georgian,
    #[serde(rename = "Glag")]
    Glagolitic,
    #[serde(rename = "Gong")]
    GunjalaGondi,
    #[serde(rename = "Gonm")]
    MasaramGondi,
    #[serde(rename = "Goth")]
    Gothic,
    #[serde(rename = "Gran")]
    Grantha,
    #[serde(rename = "Grek")]
    Greek,
    #[serde(rename = "Gujr")]
    Gujarati,
    #[serde(rename = "Guru")]
    Gurmukhi,
    #[serde(rename = "Hang")]
    Hangul,
    #[serde(rename = "Hani")]
    Han,
    #[serde(rename = "Hano")]
    Hanunoo,
    #[serde(rename = "Hatr")]
    Hatran,
    #[serde(rename = "Hebr")]
    Hebrew,
    #[serde(rename = "Hira")]
    Hiragana,
    #[serde(rename = "Hluw")]
    AnatolianHieroglyphs,
    #[serde(rename = "Hmng")]
    PahawhHmong,
    #[serde(rename = "Hmnp")]
    NyiakengPuachueHmong,
    #[serde(rename = "Hrkt")]
    KatakanaOrHiragana,
    #[serde(rename = "Hung")]
    OldHungarian,
    #[serde(rename = "Ital")]
    OldItalic,
    #[serde(rename = "Java")]
    Javanese,
    #[serde(rename = "Kali")]
    KayahLi,
    #[serde(rename = "Kana")]
    Katakana,
    #[serde(rename = "Kawi")]
    Kawi,
    #[serde(rename = "Khar")]
    Kharoshthi,
    #[serde(rename = "Khmr")]
    Khmer,
    #[serde(rename = "Khoj")]
    Khojki,
    #[serde(rename = "Kits")]
    KhitanSmallScript,
    #[serde(rename = "Knda")]
    Kannada,
    #[serde(rename = "Kthi")]
    Kaithi,
    #[serde(rename = "Lana")]
    TaiTham,
    #[serde(rename = "Laoo")]
    Lao,
    #[serde(rename = "Latn")]
    Latin,
    #[serde(rename = "Lepc")]
    Lepcha,
    #[serde(rename = "Limb")]
    Limbu,
    #[serde(rename = "Lina")]
    LinearA,
    #[serde(rename = "Linb")]
    LinearB,
    #[serde(rename = "Lisu")]
    Lisu,
    #[serde(rename = "Lyci")]
    Lycian,
    #[serde(rename = "Lydi")]
    Lydian,
    #[serde(rename = "Mahj")]
    Mahajani,
    #[serde(rename = "Maka")]
    Makasar,
    #[serde(rename = "Mand")]
    Mandaic,
    #[serde(rename = "Mani")]
    Manichaean,
    #[serde(rename = "Marc")]
    Marchen,
    #[serde(rename = "Medf")]
    Medefaidrin,
    #[serde(rename = "Mend")]
    MendeKikakui,
    #[serde(rename = "Merc")]
    MeroiticCursive,
    #[serde(rename = "Mero")]
    MeroiticHieroglyphs,
    #[serde(rename = "Mlym")]
    Malayalam,
    #[serde(rename = "Modi")]
    Modi,
    #[serde(rename = "Mong")]
    Mongolian,
    #[serde(rename = "Mroo")]
    Mro,
    #[serde(rename = "Mtei")]
    MeeteiMayek,
    #[serde(rename = "Mult")]
    Multani,
    #[serde(rename = "Mymr")]
    Myanmar,
    #[serde(rename = "Nagm")]
    NagMundari,
    #[serde(rename = "Nand")]
    Nandinagari,
    #[serde(rename = "Narb")]
    OldNorthArabian,
    #[serde(rename = "Nbat")]
    Nabataean,
    #[serde(rename = "Newa")]
    Newa,
    #[serde(rename = "Nkoo")]
    Nko,
    #[serde(rename = "Nshu")]
    Nushu,
    #[serde(rename = "Ogam")]
    Ogham,
    #[serde(rename = "Olck")]
    OlChiki,
    #[serde(rename = "Orkh")]
    OldTurkic,
    #[serde(rename = "Orya")]
    Oriya,
    #[serde(rename = "Osge")]
    Osage,
    #[serde(rename = "Osma")]
    Osmanya,
    #[serde(rename = "Ougr")]
    OldUyghur,
    #[serde(rename = "Palm")]
    Palmyrene,
    #[serde(rename = "Pauc")]
    PauCinHau,
    #[serde(rename = "Perm")]
    OldPermic,
    #[serde(rename = "Phag")]
    PhagsPa,
    #[serde(rename = "Phli")]
    InscriptionalPahlavi,
    #[serde(rename = "Phlp")]
    PsalterPahlavi,
    #[serde(rename = "Phnx")]
    Phoenician,
    #[serde(rename = "Plrd")]
    Miao,
    #[serde(rename = "Prti")]
    InscriptionalParthian,
    #[serde(rename = "Rjng")]
    Rejang,
    #[serde(rename = "Rohg")]
    HanifiRohingya,
    #[serde(rename = "Runr")]
    Runic,
    #[serde(rename = "Samr")]
    Samaritan,
    #[serde(rename = "Sarb")]
    OldSouthArabian,
    #[serde(rename = "Saur")]
    Saurashtra,
    #[serde(rename = "Sgnw")]
    SignWriting,
    #[serde(rename = "Shaw")]
    Shavian,
    #[serde(rename = "Shrd")]
    Sharada,
    #[serde(rename = "Sidd")]
    Siddham,
    #[serde(rename = "Sind")]
    Khudawadi,
    #[serde(rename = "Sinh")]
    Sinhala,
    #[serde(rename = "Sogd")]
    Sogdian,
    #[serde(rename = "Sogo")]
    OldSogdian,
    #[serde(rename = "Sora")]
    SoraSompeng,
    #[serde(rename = "Soyo")]
    Soyombo,
    #[serde(rename = "Sund")]
    Sundanese,
    #[serde(rename = "Sylo")]
    SylotiNagri,
    #[serde(rename = "Syrc")]
    Syriac,
    #[serde(rename = "Tagb")]
    Tagbanwa,
    #[serde(rename = "Takr")]
    Takri,
    #[serde(rename = "Tale")]
    TaiLe,
    #[serde(rename = "Talu")]
    NewTaiLue,
    #[serde(rename = "Taml")]
    Tamil,
    #[serde(rename = "Tang")]
    Tangut,
    #[serde(rename = "Tavt")]
    TaiViet,
    #[serde(rename = "Telu")]
    Telugu,
    #[serde(rename = "Tfng")]
    Tifinagh,
    #[serde(rename = "Tglg")]
    Tagalog,
    #[serde(rename = "Thaa")]
    Thaana,
    #[serde(rename = "Thai")]
    Thai,
    #[serde(rename = "Tibt")]
    Tibetan,
    #[serde(rename = "Tirh")]
    Tirhuta,
    #[serde(rename = "Tnsa")]
    Tangsa,
    #[serde(rename = "Toto")]
    Toto,
    #[serde(rename = "Ugar")]
    Ugaritic,
    #[serde(rename = "Vaii")]
    Vai,
    #[serde(rename = "Vith")]
    Vithkuqi,
    #[serde(rename = "Wara")]
    WarangCiti,
    #[serde(rename = "Wcho")]
    Wancho,
    #[serde(rename = "Xpeo")]
    OldPersian,
    #[serde(rename = "Xsux")]
    Cuneiform,
    #[serde(rename = "Yezi")]
    Yezidi,
    #[serde(rename = "Yiii")]
    Yi,
    #[serde(rename = "Zanb")]
    ZanabazarSquare,
    #[serde(rename = "Zinh")]
    Inherited,
    #[serde(rename = "Zyyy")]
    Common,
    #[serde(rename = "Zzzz")]
    Unknown,
}
