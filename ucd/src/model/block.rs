use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum Block {
    #[serde(rename = "Adlam")]
    Adlam,
    #[serde(rename = "Aegean_Numbers")]
    AegeanNumbers,
    #[serde(rename = "Ahom")]
    Ahom,
    #[serde(rename = "Alchemical")]
    AlchemicalSymbols,
    #[serde(rename = "Alphabetic_PF")]
    AlphabeticPresentationForms,
    #[serde(rename = "Anatolian_Hieroglyphs")]
    AnatolianHieroglyphs,
    #[serde(rename = "Ancient_Greek_Music")]
    AncientGreekMusicalNotation,
    #[serde(rename = "Ancient_Greek_Numbers")]
    AncientGreekNumbers,
    #[serde(rename = "Ancient_Symbols")]
    AncientSymbols,
    #[serde(rename = "Arabic")]
    Arabic,
    #[serde(rename = "Arabic_Ext_A")]
    ArabicExtendedA,
    #[serde(rename = "Arabic_Ext_B")]
    ArabicExtendedB,
    #[serde(rename = "Arabic_Ext_C")]
    ArabicExtendedC,
    #[serde(rename = "Arabic_Math")]
    ArabicMathematicalAlphabeticSymbols,
    #[serde(rename = "Arabic_PF_A")]
    ArabicPresentationFormsA,
    #[serde(rename = "Arabic_PF_B")]
    ArabicPresentationFormsB,
    #[serde(rename = "Arabic_Sup")]
    ArabicSupplement,
    #[serde(rename = "Armenian")]
    Armenian,
    #[serde(rename = "Arrows")]
    Arrows,
    #[serde(rename = "ASCII")]
    BasicLatin,
    #[serde(rename = "Avestan")]
    Avestan,
    #[serde(rename = "Balinese")]
    Balinese,
    #[serde(rename = "Bamum")]
    Bamum,
    #[serde(rename = "Bamum_Sup")]
    BamumSupplement,
    #[serde(rename = "Bassa_Vah")]
    BassaVah,
    #[serde(rename = "Batak")]
    Batak,
    #[serde(rename = "Bengali")]
    Bengali,
    #[serde(rename = "Bhaiksuki")]
    Bhaiksuki,
    #[serde(rename = "Block_Elements")]
    BlockElements,
    #[serde(rename = "Bopomofo")]
    Bopomofo,
    #[serde(rename = "Bopomofo_Ext")]
    BopomofoExtended,
    #[serde(rename = "Box_Drawing")]
    BoxDrawing,
    #[serde(rename = "Brahmi")]
    Brahmi,
    #[serde(rename = "Braille")]
    BraillePatterns,
    #[serde(rename = "Buginese")]
    Buginese,
    #[serde(rename = "Buhid")]
    Buhid,
    #[serde(rename = "Byzantine_Music")]
    ByzantineMusicalSymbols,
    #[serde(rename = "Carian")]
    Carian,
    #[serde(rename = "Caucasian_Albanian")]
    CaucasianAlbanian,
    #[serde(rename = "Chakma")]
    Chakma,
    #[serde(rename = "Cham")]
    Cham,
    #[serde(rename = "Cherokee")]
    Cherokee,
    #[serde(rename = "Cherokee_Sup")]
    CherokeeSupplement,
    #[serde(rename = "Chess_Symbols")]
    ChessSymbols,
    #[serde(rename = "Chorasmian")]
    Chorasmian,
    #[serde(rename = "CJK")]
    CJKUnifiedIdeographs,
    #[serde(rename = "CJK_Compat")]
    CJKCompatibility,
    #[serde(rename = "CJK_Compat_Forms")]
    CJKCompatibilityForms,
    #[serde(rename = "CJK_Compat_Ideographs")]
    CJKCompatibilityIdeographs,
    #[serde(rename = "CJK_Compat_Ideographs_Sup")]
    CJKCompatibilityIdeographsSupplement,
    #[serde(rename = "CJK_Ext_A")]
    CJKUnifiedIdeographsExtensionA,
    #[serde(rename = "CJK_Ext_B")]
    CJKUnifiedIdeographsExtensionB,
    #[serde(rename = "CJK_Ext_C")]
    CJKUnifiedIdeographsExtensionC,
    #[serde(rename = "CJK_Ext_D")]
    CJKUnifiedIdeographsExtensionD,
    #[serde(rename = "CJK_Ext_E")]
    CJKUnifiedIdeographsExtensionE,
    #[serde(rename = "CJK_Ext_F")]
    CJKUnifiedIdeographsExtensionF,
    #[serde(rename = "CJK_Ext_G")]
    CJKUnifiedIdeographsExtensionG,
    #[serde(rename = "CJK_Ext_H")]
    CJKUnifiedIdeographsExtensionH,
    #[serde(rename = "CJK_Ext_I")]
    CJKUnifiedIdeographsExtensionI,
    #[serde(rename = "CJK_Radicals_Sup")]
    CJKRadicalsSupplement,
    #[serde(rename = "CJK_Strokes")]
    CJKStrokes,
    #[serde(rename = "CJK_Symbols")]
    CJKSymbolsAndPunctuation,
    #[serde(rename = "Compat_Jamo")]
    HangulCompatibilityJamo,
    #[serde(rename = "Control_Pictures")]
    ControlPictures,
    #[serde(rename = "Coptic")]
    Coptic,
    #[serde(rename = "Coptic_Epact_Numbers")]
    CopticEpactNumbers,
    #[serde(rename = "Counting_Rod")]
    CountingRodNumerals,
    #[serde(rename = "Cuneiform")]
    Cuneiform,
    #[serde(rename = "Cuneiform_Numbers")]
    CuneiformNumbersAndPunctuation,
    #[serde(rename = "Currency_Symbols")]
    CurrencySymbols,
    #[serde(rename = "Cypriot_Syllabary")]
    CypriotSyllabary,
    #[serde(rename = "Cypro_Minoan")]
    CyproMinoan,
    #[serde(rename = "Cyrillic")]
    Cyrillic,
    #[serde(rename = "Cyrillic_Ext_A")]
    CyrillicExtendedA,
    #[serde(rename = "Cyrillic_Ext_B")]
    CyrillicExtendedB,
    #[serde(rename = "Cyrillic_Ext_C")]
    CyrillicExtendedC,
    #[serde(rename = "Cyrillic_Ext_D")]
    CyrillicExtendedD,
    #[serde(rename = "Cyrillic_Sup")]
    CyrillicSupplement,
    #[serde(rename = "Deseret")]
    Deseret,
    #[serde(rename = "Devanagari")]
    Devanagari,
    #[serde(rename = "Devanagari_Ext")]
    DevanagariExtended,
    #[serde(rename = "Devanagari_Ext_A")]
    DevanagariExtendedA,
    #[serde(rename = "Diacriticals")]
    CombiningDiacriticalMarks,
    #[serde(rename = "Diacriticals_Ext")]
    CombiningDiacriticalMarksExtended,
    #[serde(rename = "Diacriticals_For_Symbols")]
    CombiningDiacriticalMarksForSymbols,
    #[serde(rename = "Diacriticals_Sup")]
    CombiningDiacriticalMarksSupplement,
    #[serde(rename = "Dingbats")]
    Dingbats,
    #[serde(rename = "Dives_Akuru")]
    DivesAkuru,
    #[serde(rename = "Dogra")]
    Dogra,
    #[serde(rename = "Domino")]
    DominoTiles,
    #[serde(rename = "Duployan")]
    Duployan,
    #[serde(rename = "Early_Dynastic_Cuneiform")]
    EarlyDynasticCuneiform,
    #[serde(rename = "Egyptian_Hieroglyph_Format_Controls")]
    EgyptianHieroglyphFormatControls,
    #[serde(rename = "Egyptian_Hieroglyphs")]
    EgyptianHieroglyphs,
    #[serde(rename = "Elbasan")]
    Elbasan,
    #[serde(rename = "Elymaic")]
    Elymaic,
    #[serde(rename = "Emoticons")]
    Emoticons,
    #[serde(rename = "Enclosed_Alphanum")]
    EnclosedAlphanumerics,
    #[serde(rename = "Enclosed_Alphanum_Sup")]
    EnclosedAlphanumericSupplement,
    #[serde(rename = "Enclosed_CJK")]
    EnclosedCJKLettersAndMonths,
    #[serde(rename = "Enclosed_Ideographic_Sup")]
    EnclosedIdeographicSupplement,
    #[serde(rename = "Ethiopic")]
    Ethiopic,
    #[serde(rename = "Ethiopic_Ext")]
    EthiopicExtended,
    #[serde(rename = "Ethiopic_Ext_A")]
    EthiopicExtendedA,
    #[serde(rename = "Ethiopic_Ext_B")]
    EthiopicExtendedB,
    #[serde(rename = "Ethiopic_Sup")]
    EthiopicSupplement,
    #[serde(rename = "Geometric_Shapes")]
    GeometricShapes,
    #[serde(rename = "Geometric_Shapes_Ext")]
    GeometricShapesExtended,
    #[serde(rename = "Georgian")]
    Georgian,
    #[serde(rename = "Georgian_Ext")]
    GeorgianExtended,
    #[serde(rename = "Georgian_Sup")]
    GeorgianSupplement,
    #[serde(rename = "Glagolitic")]
    Glagolitic,
    #[serde(rename = "Glagolitic_Sup")]
    GlagoliticSupplement,
    #[serde(rename = "Gothic")]
    Gothic,
    #[serde(rename = "Grantha")]
    Grantha,
    #[serde(rename = "Greek")]
    GreekAndCoptic,
    #[serde(rename = "Greek_Ext")]
    GreekExtended,
    #[serde(rename = "Gujarati")]
    Gujarati,
    #[serde(rename = "Gunjala_Gondi")]
    GunjalaGondi,
    #[serde(rename = "Gurmukhi")]
    Gurmukhi,
    #[serde(rename = "Half_And_Full_Forms")]
    HalfwidthAndFullwidthForms,
    #[serde(rename = "Half_Marks")]
    CombiningHalfMarks,
    #[serde(rename = "Hangul")]
    HangulSyllables,
    #[serde(rename = "Hanifi_Rohingya")]
    HanifiRohingya,
    #[serde(rename = "Hanunoo")]
    Hanunoo,
    #[serde(rename = "Hatran")]
    Hatran,
    #[serde(rename = "Hebrew")]
    Hebrew,
    #[serde(rename = "High_PU_Surrogates")]
    HighPrivateUseSurrogates,
    #[serde(rename = "High_Surrogates")]
    HighSurrogates,
    #[serde(rename = "Hiragana")]
    Hiragana,
    #[serde(rename = "IDC")]
    IdeographicDescriptionCharacters,
    #[serde(rename = "Ideographic_Symbols")]
    IdeographicSymbolsAndPunctuation,
    #[serde(rename = "Imperial_Aramaic")]
    ImperialAramaic,
    #[serde(rename = "Indic_Number_Forms")]
    CommonIndicNumberForms,
    #[serde(rename = "Indic_Siyaq_Numbers")]
    IndicSiyaqNumbers,
    #[serde(rename = "Inscriptional_Pahlavi")]
    InscriptionalPahlavi,
    #[serde(rename = "Inscriptional_Parthian")]
    InscriptionalParthian,
    #[serde(rename = "IPA_Ext")]
    IPAExtensions,
    #[serde(rename = "Jamo")]
    HangulJamo,
    #[serde(rename = "Jamo_Ext_A")]
    HangulJamoExtendedA,
    #[serde(rename = "Jamo_Ext_B")]
    HangulJamoExtendedB,
    #[serde(rename = "Javanese")]
    Javanese,
    #[serde(rename = "Kaithi")]
    Kaithi,
    #[serde(rename = "Kaktovik_Numerals")]
    KaktovikNumerals,
    #[serde(rename = "Kana_Ext_A")]
    KanaExtendedA,
    #[serde(rename = "Kana_Ext_B")]
    KanaExtendedB,
    #[serde(rename = "Kana_Sup")]
    KanaSupplement,
    #[serde(rename = "Kanbun")]
    Kanbun,
    #[serde(rename = "Kangxi")]
    KangxiRadicals,
    #[serde(rename = "Kannada")]
    Kannada,
    #[serde(rename = "Katakana")]
    Katakana,
    #[serde(rename = "Katakana_Ext")]
    KatakanaPhoneticExtensions,
    #[serde(rename = "Kawi")]
    Kawi,
    #[serde(rename = "Kayah_Li")]
    KayahLi,
    #[serde(rename = "Kharoshthi")]
    Kharoshthi,
    #[serde(rename = "Khitan_Small_Script")]
    KhitanSmallScript,
    #[serde(rename = "Khmer")]
    Khmer,
    #[serde(rename = "Khmer_Symbols")]
    KhmerSymbols,
    #[serde(rename = "Khojki")]
    Khojki,
    #[serde(rename = "Khudawadi")]
    Khudawadi,
    #[serde(rename = "Lao")]
    Lao,
    #[serde(rename = "Latin_1_Sup")]
    Latin1Supplement,
    #[serde(rename = "Latin_Ext_A")]
    LatinExtendedA,
    #[serde(rename = "Latin_Ext_Additional")]
    LatinExtendedAdditional,
    #[serde(rename = "Latin_Ext_B")]
    LatinExtendedB,
    #[serde(rename = "Latin_Ext_C")]
    LatinExtendedC,
    #[serde(rename = "Latin_Ext_D")]
    LatinExtendedD,
    #[serde(rename = "Latin_Ext_E")]
    LatinExtendedE,
    #[serde(rename = "Latin_Ext_F")]
    LatinExtendedF,
    #[serde(rename = "Latin_Ext_G")]
    LatinExtendedG,
    #[serde(rename = "Lepcha")]
    Lepcha,
    #[serde(rename = "Letterlike_Symbols")]
    LetterlikeSymbols,
    #[serde(rename = "Limbu")]
    Limbu,
    #[serde(rename = "Linear_A")]
    LinearA,
    #[serde(rename = "Linear_B_Ideograms")]
    LinearBIdeograms,
    #[serde(rename = "Linear_B_Syllabary")]
    LinearBSyllabary,
    #[serde(rename = "Lisu")]
    Lisu,
    #[serde(rename = "Lisu_Sup")]
    LisuSupplement,
    #[serde(rename = "Low_Surrogates")]
    LowSurrogates,
    #[serde(rename = "Lycian")]
    Lycian,
    #[serde(rename = "Lydian")]
    Lydian,
    #[serde(rename = "Mahajani")]
    Mahajani,
    #[serde(rename = "Mahjong")]
    MahjongTiles,
    #[serde(rename = "Makasar")]
    Makasar,
    #[serde(rename = "Malayalam")]
    Malayalam,
    #[serde(rename = "Mandaic")]
    Mandaic,
    #[serde(rename = "Manichaean")]
    Manichaean,
    #[serde(rename = "Marchen")]
    Marchen,
    #[serde(rename = "Masaram_Gondi")]
    MasaramGondi,
    #[serde(rename = "Math_Alphanum")]
    MathematicalAlphanumericSymbols,
    #[serde(rename = "Math_Operators")]
    MathematicalOperators,
    #[serde(rename = "Mayan_Numerals")]
    MayanNumerals,
    #[serde(rename = "Medefaidrin")]
    Medefaidrin,
    #[serde(rename = "Meetei_Mayek")]
    MeeteiMayek,
    #[serde(rename = "Meetei_Mayek_Ext")]
    MeeteiMayekExtensions,
    #[serde(rename = "Mende_Kikakui")]
    MendeKikakui,
    #[serde(rename = "Meroitic_Cursive")]
    MeroiticCursive,
    #[serde(rename = "Meroitic_Hieroglyphs")]
    MeroiticHieroglyphs,
    #[serde(rename = "Miao")]
    Miao,
    #[serde(rename = "Misc_Arrows")]
    MiscellaneousSymbolsAndArrows,
    #[serde(rename = "Misc_Math_Symbols_A")]
    MiscellaneousMathematicalSymbolsA,
    #[serde(rename = "Misc_Math_Symbols_B")]
    MiscellaneousMathematicalSymbolsB,
    #[serde(rename = "Misc_Pictographs")]
    MiscellaneousSymbolsAndPictographs,
    #[serde(rename = "Misc_Symbols")]
    MiscellaneousSymbols,
    #[serde(rename = "Misc_Technical")]
    MiscellaneousTechnical,
    #[serde(rename = "Modi")]
    Modi,
    #[serde(rename = "Modifier_Letters")]
    SpacingModifierLetters,
    #[serde(rename = "Modifier_Tone_Letters")]
    ModifierToneLetters,
    #[serde(rename = "Mongolian")]
    Mongolian,
    #[serde(rename = "Mongolian_Sup")]
    MongolianSupplement,
    #[serde(rename = "Mro")]
    Mro,
    #[serde(rename = "Multani")]
    Multani,
    #[serde(rename = "Music")]
    MusicalSymbols,
    #[serde(rename = "Myanmar")]
    Myanmar,
    #[serde(rename = "Myanmar_Ext_A")]
    MyanmarExtendedA,
    #[serde(rename = "Myanmar_Ext_B")]
    MyanmarExtendedB,
    #[serde(rename = "Nabataean")]
    Nabataean,
    #[serde(rename = "Nag_Mundari")]
    NagMundari,
    #[serde(rename = "Nandinagari")]
    Nandinagari,
    #[serde(rename = "NB")]
    NoBlock,
    #[serde(rename = "New_Tai_Lue")]
    NewTaiLue,
    #[serde(rename = "Newa")]
    Newa,
    #[serde(rename = "NKo")]
    NKo,
    #[serde(rename = "Number_Forms")]
    NumberForms,
    #[serde(rename = "Nushu")]
    Nushu,
    #[serde(rename = "Nyiakeng_Puachue_Hmong")]
    NyiakengPuachueHmong,
    #[serde(rename = "OCR")]
    OpticalCharacterRecognition,
    #[serde(rename = "Ogham")]
    Ogham,
    #[serde(rename = "Ol_Chiki")]
    OlChiki,
    #[serde(rename = "Old_Hungarian")]
    OldHungarian,
    #[serde(rename = "Old_Italic")]
    OldItalic,
    #[serde(rename = "Old_North_Arabian")]
    OldNorthArabian,
    #[serde(rename = "Old_Permic")]
    OldPermic,
    #[serde(rename = "Old_Persian")]
    OldPersian,
    #[serde(rename = "Old_Sogdian")]
    OldSogdian,
    #[serde(rename = "Old_South_Arabian")]
    OldSouthArabian,
    #[serde(rename = "Old_Turkic")]
    OldTurkic,
    #[serde(rename = "Old_Uyghur")]
    OldUyghur,
    #[serde(rename = "Oriya")]
    Oriya,
    #[serde(rename = "Ornamental_Dingbats")]
    OrnamentalDingbats,
    #[serde(rename = "Osage")]
    Osage,
    #[serde(rename = "Osmanya")]
    Osmanya,
    #[serde(rename = "Ottoman_Siyaq_Numbers")]
    OttomanSiyaqNumbers,
    #[serde(rename = "Pahawh_Hmong")]
    PahawhHmong,
    #[serde(rename = "Palmyrene")]
    Palmyrene,
    #[serde(rename = "Pau_Cin_Hau")]
    PauCinHau,
    #[serde(rename = "Phags_Pa")]
    PhagsPa,
    #[serde(rename = "Phaistos")]
    PhaistosDisc,
    #[serde(rename = "Phoenician")]
    Phoenician,
    #[serde(rename = "Phonetic_Ext")]
    PhoneticExtensions,
    #[serde(rename = "Phonetic_Ext_Sup")]
    PhoneticExtensionsSupplement,
    #[serde(rename = "Playing_Cards")]
    PlayingCards,
    #[serde(rename = "Psalter_Pahlavi")]
    PsalterPahlavi,
    #[serde(rename = "PUA")]
    PrivateUseArea,
    #[serde(rename = "Punctuation")]
    GeneralPunctuation,
    #[serde(rename = "Rejang")]
    Rejang,
    #[serde(rename = "Rumi")]
    RumiNumeralSymbols,
    #[serde(rename = "Runic")]
    Runic,
    #[serde(rename = "Samaritan")]
    Samaritan,
    #[serde(rename = "Saurashtra")]
    Saurashtra,
    #[serde(rename = "Sharada")]
    Sharada,
    #[serde(rename = "Shavian")]
    Shavian,
    #[serde(rename = "Shorthand_Format_Controls")]
    ShorthandFormatControls,
    #[serde(rename = "Siddham")]
    Siddham,
    #[serde(rename = "Sinhala")]
    Sinhala,
    #[serde(rename = "Sinhala_Archaic_Numbers")]
    SinhalaArchaicNumbers,
    #[serde(rename = "Small_Forms")]
    SmallFormVariants,
    #[serde(rename = "Small_Kana_Ext")]
    SmallKanaExtension,
    #[serde(rename = "Sogdian")]
    Sogdian,
    #[serde(rename = "Sora_Sompeng")]
    SoraSompeng,
    #[serde(rename = "Soyombo")]
    Soyombo,
    #[serde(rename = "Specials")]
    Specials,
    #[serde(rename = "Sundanese")]
    Sundanese,
    #[serde(rename = "Sundanese_Sup")]
    SundaneseSupplement,
    #[serde(rename = "Sup_Arrows_A")]
    SupplementalArrowsA,
    #[serde(rename = "Sup_Arrows_B")]
    SupplementalArrowsB,
    #[serde(rename = "Sup_Arrows_C")]
    SupplementalArrowsC,
    #[serde(rename = "Sup_Math_Operators")]
    SupplementalMathematicalOperators,
    #[serde(rename = "Sup_PUA_A")]
    SupplementaryPrivateUseAreaA,
    #[serde(rename = "Sup_PUA_B")]
    SupplementaryPrivateUseAreaB,
    #[serde(rename = "Sup_Punctuation")]
    SupplementalPunctuation,
    #[serde(rename = "Sup_Symbols_And_Pictographs")]
    SupplementalSymbolsAndPictographs,
    #[serde(rename = "Super_And_Sub")]
    SuperscriptsAndSubscripts,
    #[serde(rename = "Sutton_SignWriting")]
    SuttonSignWriting,
    #[serde(rename = "Syloti_Nagri")]
    SylotiNagri,
    #[serde(rename = "Symbols_And_Pictographs_Ext_A")]
    SymbolsAndPictographsExtendedA,
    #[serde(rename = "Symbols_For_Legacy_Computing")]
    SymbolsForLegacyComputing,
    #[serde(rename = "Syriac")]
    Syriac,
    #[serde(rename = "Syriac_Sup")]
    SyriacSupplement,
    #[serde(rename = "Tagalog")]
    Tagalog,
    #[serde(rename = "Tagbanwa")]
    Tagbanwa,
    #[serde(rename = "Tags")]
    Tags,
    #[serde(rename = "Tai_Le")]
    TaiLe,
    #[serde(rename = "Tai_Tham")]
    TaiTham,
    #[serde(rename = "Tai_Viet")]
    TaiViet,
    #[serde(rename = "Tai_Xuan_Jing")]
    TaiXuanJingSymbols,
    #[serde(rename = "Takri")]
    Takri,
    #[serde(rename = "Tamil")]
    Tamil,
    #[serde(rename = "Tamil_Sup")]
    TamilSupplement,
    #[serde(rename = "Tangsa")]
    Tangsa,
    #[serde(rename = "Tangut")]
    Tangut,
    #[serde(rename = "Tangut_Components")]
    TangutComponents,
    #[serde(rename = "Tangut_Sup")]
    TangutSupplement,
    #[serde(rename = "Telugu")]
    Telugu,
    #[serde(rename = "Thaana")]
    Thaana,
    #[serde(rename = "Thai")]
    Thai,
    #[serde(rename = "Tibetan")]
    Tibetan,
    #[serde(rename = "Tifinagh")]
    Tifinagh,
    #[serde(rename = "Tirhuta")]
    Tirhuta,
    #[serde(rename = "Toto")]
    Toto,
    #[serde(rename = "Transport_And_Map")]
    TransportAndMapSymbols,
    #[serde(rename = "UCAS")]
    UnifiedCanadianAboriginalSyllabics,
    #[serde(rename = "UCAS_Ext")]
    UnifiedCanadianAboriginalSyllabicsExtended,
    #[serde(rename = "UCAS_Ext_A")]
    UnifiedCanadianAboriginalSyllabicsExtendedA,
    #[serde(rename = "Ugaritic")]
    Ugaritic,
    #[serde(rename = "Vai")]
    Vai,
    #[serde(rename = "Vedic_Ext")]
    VedicExtensions,
    #[serde(rename = "Vertical_Forms")]
    VerticalForms,
    #[serde(rename = "Vithkuqi")]
    Vithkuqi,
    #[serde(rename = "VS")]
    VariationSelectors,
    #[serde(rename = "VS_Sup")]
    VariationSelectorsSupplement,
    #[serde(rename = "Wancho")]
    Wancho,
    #[serde(rename = "Warang_Citi")]
    WarangCiti,
    #[serde(rename = "Yezidi")]
    Yezidi,
    #[serde(rename = "Yi_Radicals")]
    YiRadicals,
    #[serde(rename = "Yi_Syllables")]
    YiSyllables,
    #[serde(rename = "Yijing")]
    YijingHexagramSymbols,
    #[serde(rename = "Zanabazar_Square")]
    ZanabazarSquare,
}
