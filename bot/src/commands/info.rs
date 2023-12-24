use std::{fmt::Display, marker::PhantomData};

use poise::serenity_prelude::{
    ActivityType, Channel, Emoji, EmojiId, GenericId, Guild, Member, Message, Role, User,
};
use regex::Regex;
use serenity::{
    builder::CreateEmbed,
    model::{id, Permissions},
    prelude::Mentionable,
};
use ucd::model::{
    bidi_paired_bracket_type::BidiPairedBracketType, decomposition_type::DecompositionType,
    hangul_syllable_type::HangulSyllableType, indic_positional_category::IndicPositionalCategory,
    indic_syllabic_category::IndicSyllabicCategory, jamo_short_name::JamoShortName,
    joining_group::JoiningGroup, numeric_type::NumericType,
};

use crate::{
    common::{yn, Context, Desc, Output, BAR, DELVE, DERIVE, TAB, TAIL},
    embed_preset::user,
    emojis,
    ext::{Fmt, RegexExt},
    paginator::{Inter, Paginator, PaginatorOptions},
};

#[derive(Clone)]
pub enum Target {
    User(Box<User>, Option<Box<Member>>),
    Guild(Box<Guild>),
    Channel(Box<Channel>),
    Role(Box<Role>),
    Message(Box<Message>),
    Emoji(Box<Emoji>),
    Snowflake(GenericId),
    Char(char),
    Number(f64),
    Text(String),
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Channel(a), Self::Channel(b)) => a.id() == b.id(),
            (Self::Char(a), Self::Char(b)) => a == b,
            (Self::Emoji(a), Self::Emoji(b)) => a.id == b.id,
            (Self::Guild(a), Self::Guild(b)) => a.id == b.id,
            (Self::Message(a), Self::Message(b)) => a.id == b.id,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::Role(a), Self::Role(b)) => a.id == b.id,
            (Self::Snowflake(a), Self::Snowflake(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::User(a, _), Self::User(b, _)) => a.id == b.id,
            _ => false,
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::Channel(c) => format!("Channel({})", c.mention()),
            Self::Char(c) => format!("Char({c})"),
            Self::Emoji(e) => format!("Emoji({})", e.mention()),
            Self::Guild(g) => format!("Guild({})", g.id),
            Self::Message(m) => format!("Message({})", m.link()),
            Self::Number(c) => format!("Number({c})"),
            Self::Role(r) => format!("Role({})", r.mention()),
            Self::Snowflake(s) => format!("Snowflake({s})"),
            Self::Text(t) => format!("Text({t:?})"),
            Self::User(u, ..) => format!("User({})", u.mention()),
            // _ => "...".to_string(),
        };

        write!(f, "{x}")
    }
}

pub async fn parse(context: Context<'_>, text: String) -> Vec<Target> {
    let mut matches = vec![];

    let uid = Regex::new("\\D").unwrap();
    matches.extend(
        context
            .serenity_context()
            .cache
            .users()
            .iter()
            .filter(|x| {
                (uid.replace_all(&text, "") == x.id.to_string()
                    || x.tag().to_lowercase().contains(&text.to_lowercase()))
            })
            .filter(|x| {
                context
                    .guild()
                    .map(|y| y.members.contains_key(&x.id))
                    .unwrap_or(x.id == context.author().id)
            })
            .map(|x| {
                Target::User(
                    Box::new(x.value().clone()),
                    context
                        .guild()
                        .and_then(|g| g.members.get(&x.id).cloned())
                        .map(Box::new),
                )
            }),
    );

    if let Ok(i) = uid.replace_all(&text, "").parse() {
        if let Ok(p) = context.serenity_context().http.get_user(i).await {
            matches.push(Target::User(Box::new(p), None));
        }
    }

    if text.test("<a?:\\w+:\\d{16,}>") {
        let emoji_id = text
            .str_replace("<a?:\\w+:(\\d{16,})>", "$1")
            .parse::<u64>()
            .unwrap()
            .into();

        for g in context.serenity_context().cache.guilds().iter() {
            let test = g.emoji(&context.serenity_context().http, emoji_id).await;

            if let Ok(t) = test {
                matches.push(Target::Emoji(Box::new(t)));
                break;
            }
        }
    }

    if text.test("^\\d{16,}") {
        matches.push(Target::Snowflake(text.parse::<u64>().unwrap().into()));
    }

    if let Ok(p) = text.parse::<f64>() {
        matches.push(Target::Number(p))
    }
    if text.chars().count() == 1 {
        matches.push(Target::Char(text.chars().next().unwrap()))
    }
    matches.push(Target::Text(text));
    matches.dedup_by(|a, b| a == b);
    matches
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// return information about pretty much anything
pub async fn info(context: Context<'_>, text: Option<String>) -> Output {
    let targets = if let Some(t) = text {
        parse(context, t).await
    } else {
        vec![Target::User(
            Box::new(context.author().clone()),
            context
                .author_member()
                .await
                .map(|x| Box::new(x.into_owned())),
        )]
    };

    let mut pages = Vec::new();

    for target in targets {
        pages.push(gen_page(&context, &target, CreateEmbed::default()).await?);
    }

    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| &pages[page - 1],
            on_page: |page: usize, data: &CreateEmbed, mut ctx: Inter| {
                ctx.embed(|b| {
                    let x = user(context, b)
                        .footer(|x| x.text(format!("Page {}/{}", page, pages.len())));

                    x.0.extend(data.0.clone());
                    x
                });

                ctx
            },

            none: PhantomData,
            page_limit: pages.len(),
            targets: None,
        },
    );

    pn.start().await;
    Ok(())
}

pub async fn gen_page<'a>(
    context: &Context<'_>,
    data: &Target,
    mut x: CreateEmbed,
) -> Result<CreateEmbed, Box<dyn std::error::Error + Send + Sync>> {
    match data {
        Target::Emoji(e) => {
            x.title("Emoji Information");
            let mut txt = Desc::new();
            txt.emoji(emojis::RichActivity, "**Id**", e.id);
            txt.emoji(emojis::Pencil, "**Name**", e.id);
            txt.emoji(
                emojis::Pictures,
                "**Animated**",
                if e.animated { "Yes" } else { "No" },
            );
            txt.emoji(
                if e.available {
                    emojis::Allow
                } else {
                    emojis::Deny
                },
                "**Available**",
                if e.available { "Yes" } else { "No" },
            );
            txt.emoji(
                emojis::Person,
                "**Managed**",
                if e.managed { "Yes" } else { "No" },
            );
        }
        Target::Text(c) => {
            x.title("Text Information");
            let mut txt = Desc::new();

            txt.field("**Length (Byte count)**", c.len());
            txt.field("**Character count**", c.chars().count());
            txt.field("**Is ASCII?**", yn(c.is_ascii()));
            txt.field("**UTF-16 Byte count**", c.encode_utf16().count());
            txt.field("**Is Empty?**", yn(c.is_empty()));
            txt.field("**Line Count**", c.lines().count());

            x.description(txt.finish());
        }
        Target::Number(c) => {
            x.title("Number Information");

            let mut ftxt = Desc::new();

            number::text(*c, &mut ftxt);

            number::f_sets(*c, &mut ftxt);
            number::f_closest_frac(*c, &mut ftxt);
            number::f_operations(*c, &mut ftxt);
            x.description(ftxt.finish());

            if c.abs().floor() == *c {
                let mut txt = Desc::new();
                let n = c.abs() as u64;
                number::english(n, &mut txt);
                number::prime_factorization(n, &mut txt);
                number::divisors(n, &mut txt);
                number::representations(n, &mut txt);
                x.field("Integer Information", txt.finish(), false);
            }
        }
        Target::Char(c) => {
            x.title("Character Information");
            let data = context
                .data()
                .ucd
                .dec(*c as u32)
                .await?
                .ok_or(format!("Unable to get data for '{c:?}'"))?;

            let name = if !data.name.is_empty() {
                data.name
            } else if !data.name1.is_empty() {
                data.name1
            } else {
                c.to_string()
            };
            x.thumbnail(format!(
                "https://fileformat.info/info/unicode/char/{:x}/{}.png",
                data.code_point,
                name.replace(' ', "_").to_lowercase()
            ));

            let mut txt = Desc::new();

            txt.field("**Name**", name);

            txt.field("**Age**", data.age);
            txt.field("**Version**", data.version);
            txt.field("**Block**", format!("{:?}", data.block));
            txt.field("**Script**", format!("{:?}", data.script));
            txt.field(
                "**General Category**",
                format!("{:?}", data.general_category),
            );
            txt.field("**Code Point**", format!("U+{:0>4x}", data.code_point));
            if data.combining_class != 0 {
                txt.field(
                    "**Combining Class**",
                    char::from_u32(data.combining_class).unwrap(),
                );
            }

            txt.nl();

            if data.decomposition_type != DecompositionType::None {
                txt.field(
                    "**Decomposition**",
                    data.decomposition
                        .into_iter()
                        .map(char::from_u32)
                        .map(Option::unwrap)
                        .collect::<String>(),
                );
                txt.field(
                    "**Decomposition Type**",
                    format!("{:?}", data.decomposition_type),
                );
                txt.nl();
            }

            txt.field("**Bi-Directional Class**", format!("{:?}", data.bidi_class));
            if let Some(glyph) = data.bidi_mirrored_glyph {
                txt.field(
                    "**Bi-Directional Mirrored Glyph**",
                    format!("{}", char::from_u32(glyph).unwrap()),
                );
            }

            if data.bidi_paired_bracket_type != BidiPairedBracketType::None {
                txt.field(
                    "**Bi-Directional Paired Bracket**",
                    char::from_u32(data.bidi_paired_bracket).unwrap(),
                );
                txt.field(
                    "**Bi-Directional Paired Bracket Type**",
                    format!("{:?}", data.bidi_paired_bracket_type),
                );
            }
            txt.nl();

            txt.field("**Line Break**", format!("{:?}", data.linebreak));
            txt.field(
                "**Grapheme Cluster Break**",
                format!("{:?}", data.grapheme_cluster_break),
            );
            txt.field("**Sentence Break**", format!("{:?}", data.sentence_break));
            txt.field("**Word Break**", format!("{:?}", data.word_break));

            txt.nl();

            txt.field(
                "**East Asian Width**",
                format!("{:?}", data.east_asian_width),
            );
            if data.hangul_syllable_type != HangulSyllableType::NotApplicable {
                txt.field(
                    "**Hangul Syllable Type**",
                    format!("{:?}", data.hangul_syllable_type),
                );
            }
            txt.nl();

            if let Some(imc) = data.indic_matra_category {
                txt.field("**Indic Matra Category**", imc);
            }

            if data.indic_positional_category != IndicPositionalCategory::NA {
                txt.field(
                    "**Indic Positional Category**",
                    format!("{:?}", data.indic_positional_category),
                );
            }

            if data.indic_syllabic_category != IndicSyllabicCategory::Other {
                txt.field(
                    "**Indic Syllabic Category**",
                    format!("{:?}", data.indic_positional_category),
                );
            }

            txt.nl();

            if data.jamo_short_name != JamoShortName::None {
                txt.field("**Jamo Short Name**", format!("{:?}", data.jamo_short_name));
                txt.nl();
            }

            if data.joining_group != JoiningGroup::NoJoiningGroup {
                txt.field("**Joining Group**", format!("{:?}", data.joining_group));
                txt.field("**Joining Type**", format!("{:?}", data.joining_type));
                txt.nl();
            }

            if data.numeric_type != NumericType::None {
                txt.field("**Numeric Type**", format!("{:?}", data.numeric_type));
                if let Some(n) = data.numeric_value {
                    txt.field(
                        "**Numeric Value**",
                        format!("{}/{}", n.numerator, n.denominator),
                    );
                }
                txt.nl();
            }

            x.description(txt.finish());

            let mut tags = vec![];

            if data.alphabetic {
                tags.push("Alphabetic");
            }
            if data.ascii_hex_digit {
                tags.push("ASCII Hex Digit");
            }
            if data.bidi_control {
                tags.push("Bi-Directional Control Character");
            }
            if data.bidi_m {
                tags.push("Bi-Directional Marker");
            }
            if data.case_ignorable {
                tags.push("Case-Ignorable");
            }
            if data.cased {
                tags.push("Cased");
            }
            if data.composition_exclusion {
                tags.push("Composition Exclusion");
            }
            if data.composition_exclusion_full {
                tags.push("Full Composition Exclusion");
            }
            if data.dash {
                tags.push("Dash");
            }
            if data.default_ignorable_code_point {
                tags.push("Default Ignorable Code Point");
            }
            if data.deprecated {
                tags.push("Deprecated");
            }
            if data.diacritic {
                tags.push("Diacritic");
            }
            if data.extender {
                tags.push("Extender");
            }
            if data.grapheme_base {
                tags.push("Grapheme Base");
            }
            if data.grapheme_extend {
                tags.push("Grapheme Extend");
            }
            if data.hex_digit {
                tags.push("Hex Digit");
            }
            if data.id_continue {
                tags.push("ID Continue");
            }
            if data.id_start {
                tags.push("ID Start");
            }
            if data.ideographic {
                tags.push("Ideographic");
            }
            if data.ids_binary_operator {
                tags.push("IDS Binary Operator");
            }
            if data.ids_trinary_operator {
                tags.push("IDS Trinary Operator");
            }
            if data.join_control {
                tags.push("Join Control");
            }
            if data.logical_order_exception {
                tags.push("Logical Order Exception");
            }
            if data.lowercase {
                tags.push("Lowercase");
            }
            if data.math {
                tags.push("Math");
            }
            if data.noncharacter {
                tags.push("Non-Character");
            }
            if data.pattern_syntax {
                tags.push("Pattern (Syntax)");
            }
            if data.pattern_white_space {
                tags.push("Pattern (White Space)");
            }
            if data.quick_check_nfc.into() {
                tags.push("Quick Check NFC");
            }
            if data.quick_check_nfd.into() {
                tags.push("Quick Check NFD");
            }
            if data.quick_check_nfkc.into() {
                tags.push("Quick Check NFKC");
            }
            if data.quick_check_nfkd.into() {
                tags.push("Quick Check NFKD");
            }
            if data.quotation_mark {
                tags.push("Quotation Mark");
            }
            if data.radical {
                tags.push("Radical");
            }
            if data.sentence_terminal {
                tags.push("Sentence Terminal");
            }
            if data.soft_dotted {
                tags.push("Soft Dotten");
            }
            if data.terminal_punctuation {
                tags.push("Terminal Punctuation");
            }
            if data.unified_ideographic {
                tags.push("Unified Ideographic");
            }
            if data.uppercase {
                tags.push("Uppercase");
            }
            if data.variation_selector {
                tags.push("Variation Selector");
            }
            if data.white_space {
                tags.push("White Space");
            }
            if data.xid_continue {
                tags.push("XID Continue");
            }
            if data.xid_start {
                tags.push("XID Start");
            }

            x.field("Tags", tags.join(", "), false);

            if data.lowercase_mapping.len() > 0 {
                x.field(
                    "Lowercase Mapping",
                    format!(
                        "{} ({})",
                        data.lowercase_mapping
                            .into_iter()
                            .map(char::from_u32)
                            .map(Option::unwrap)
                            .collect::<String>(),
                        char::from_u32(data.simple_lowercase_mapping).unwrap()
                    ),
                    true,
                );
            }
            if data.uppercase_mapping.len() > 0 {
                x.field(
                    "Uppercase Mapping",
                    format!(
                        "{} ({})",
                        data.uppercase_mapping
                            .into_iter()
                            .map(char::from_u32)
                            .map(Option::unwrap)
                            .collect::<String>(),
                        char::from_u32(data.simple_uppercase_mapping).unwrap()
                    ),
                    true,
                );
            }
            if data.titlecase_mapping.len() > 0 {
                x.field(
                    "Title-Case Mapping",
                    format!(
                        "{} ({})",
                        data.titlecase_mapping
                            .into_iter()
                            .map(char::from_u32)
                            .map(Option::unwrap)
                            .collect::<String>(),
                        char::from_u32(data.simple_titlecase_mapping).unwrap()
                    ),
                    true,
                );
            }
        }
        Target::User(c, m) => {
            x.title("User Information");
            x.thumbnail(m.as_ref().map(|x| x.face()).unwrap_or(c.face()));

            let mut txt = Desc::new();
            txt.emoji(emojis::RichActivity, "**Id**", format!("`{}`", c.id));
            txt.emoji(
                emojis::At,
                "**Profile**",
                format!(
                    "[{}]({}) ({})",
                    if c.discriminator == 0 {
                        "@".to_string() + &c.name
                    } else {
                        c.tag()
                    },
                    format!("https://discord.com/@me/users/{}", c.id),
                    c.mention()
                ),
            );

            txt.emoji(
                emojis::ImagePlaceholder,
                "**Avatar**",
                format!(
                    "[Main]({}) | [Default]({}){}",
                    c.face(),
                    c.default_avatar_url(),
                    m.as_ref()
                        .and_then(|x| x.avatar_url())
                        .map(|x| format!(" | [Server]({x})"))
                        .unwrap_or("".to_string())
                ),
            );

            txt.emoji(
                emojis::EmojiFrequentCategory,
                "**Created At**",
                format!("<t:{0}:f> (<t:{0}:R>)", c.created_at().unix_timestamp()),
            );

            x.description(txt.finish());

            if let Some(m) = m {
                if let Some(g) = context.serenity_context().cache.guild(m.guild_id) {
                    if let Some(p) = g.presences.get(&m.user.id) {
                        let mut desc = vec![];

                        desc.push(format!(
                            "{} {}",
                            user::status_emoji(&p.status),
                            user::status_text(&p.status)
                        ));

                        let mut i = 0;
                        for activity in &p.activities {
                            i += 1;
                            if activity.kind == ActivityType::Custom {
                                desc.push(format!(
                                    "{} {}",
                                    activity
                                        .emoji
                                        .as_ref()
                                        .map(|x| if let Some(t) = x.id {
                                            format!(
                                                "<{}:{}:{}>",
                                                if x.animated == Some(true) { "a" } else { "" },
                                                x.name,
                                                t
                                            )
                                        } else {
                                            x.name.clone()
                                        })
                                        .unwrap_or("".to_string()),
                                    activity.state.clone().unwrap_or("".to_string())
                                ));
                                continue;
                            }

                            let c = if i == p.activities.len() {
                                DERIVE
                            } else {
                                DELVE
                            };

                            desc.push(format!(
                                "{c} {} **{}**",
                                user::activity_kind_text(&activity.kind),
                                activity.name
                            ));

                            if let Some(d) = &activity.details {
                                desc.push(format!(
                                    "{}{TAB}{DERIVE} {}",
                                    if i == p.activities.len() { TAB } else { BAR },
                                    d
                                ))
                            }
                        }
                        x.field(format!("{TAIL} Presence"), desc.join("\n"), false);
                    }
                }

                let mut mdesc = Desc::new();

                if let Some(n) = &m.nick {
                    mdesc.emoji(
                        emojis::Edit,
                        "**Nickname**",
                        format!("`{}`", n.str_replace("`", "'")),
                    );
                };

                if let Some(j) = m.joined_at {
                    mdesc.emoji(
                        emojis::EmojiFrequentCategory,
                        "**Joined At**",
                        format!("<t:{0}:f> (<t:{0}:R>)", j.unix_timestamp(),),
                    );
                }

                if let Some(p) = m.premium_since {
                    mdesc.emoji(
                        emojis::PremiumGuildSubscriberBadge,
                        "**Boosting Since**",
                        format!("<t:{0}:f> (<t:{0}:R>)", p.unix_timestamp()),
                    );
                }

                mdesc.nl();

                let roles = m.roles(&context.serenity_context().cache);
                if let Some(r) = roles {
                    if !r.is_empty() {
                        mdesc.emoji(
                            emojis::ShieldStar,
                            "**Roles**",
                            r.iter()
                                .map(|x| x.mention().to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                        );
                    }
                } else if !m.roles.is_empty() {
                    mdesc.emoji(
                        emojis::ShieldStar,
                        "**Roles**",
                        m.roles
                            .iter()
                            .map(|x| x.mention().to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }

                x.field(format!("{TAIL} Member Information"), mdesc.finish(), false);

                if let Some(p) = m
                    .permissions(&context.serenity_context().cache)
                    .ok()
                    .or(m.permissions)
                {
                    x.field(
                        format!("{TAIL} Permissions"),
                        p.difference(
                            Permissions::CREATE_INSTANT_INVITE
                                | Permissions::ADD_REACTIONS
                                | Permissions::STREAM
                                | Permissions::VIEW_CHANNEL
                                | Permissions::SEND_MESSAGES
                                | Permissions::SEND_TTS_MESSAGES
                                | Permissions::EMBED_LINKS
                                | Permissions::ATTACH_FILES
                                | Permissions::READ_MESSAGE_HISTORY
                                | Permissions::USE_EXTERNAL_EMOJIS
                                | Permissions::CONNECT
                                | Permissions::SPEAK
                                | Permissions::USE_VAD
                                | Permissions::CHANGE_NICKNAME
                                | Permissions::VIEW_GUILD_INSIGHTS
                                | Permissions::VIEW_AUDIT_LOG
                                | Permissions::PRIORITY_SPEAKER
                                | Permissions::USE_SLASH_COMMANDS
                                | Permissions::REQUEST_TO_SPEAK
                                | Permissions::CREATE_PUBLIC_THREADS
                                | Permissions::CREATE_PRIVATE_THREADS
                                | Permissions::USE_EMBEDDED_ACTIVITIES
                                | Permissions::USE_EXTERNAL_STICKERS
                                | Permissions::SEND_MESSAGES_IN_THREADS,
                        )
                        .get_permission_names()
                        .join(", "),
                        false,
                    );
                }
            }
        }

        Target::Snowflake(c) => {
            x.title("Snowflake Information");

            let mut txt = Desc::new();

            txt.field("**ID**", c);
            txt.field("**Process ID**", (c.as_u64() & 0x1f000) >> 12);
            txt.field("**Worker ID**", (c.as_u64() & 0x3e0000) >> 17);

            txt.field("**Increment**", c.as_u64() & 0xfff);

            txt.field(
                "**Timestamp**",
                format!("<t:{0}:f> (<t:{0}:R>)", c.created_at().unix_timestamp()),
            );

            x.description(txt.finish());
        }

        c => {
            x.description(format!("Unfinished page for a {c}"));
        }
    };

    Ok(x)
}

mod number {
    use crate::{
        common::{format_radix, AsVec, Desc, FrequencyTable},
        ext::FloatExt,
    };

    pub fn english(f: u64, txt: &mut Desc) {
        txt.field("English Form", name(f));
        txt.nl();
    }

    pub fn text(f: f64, txt: &mut Desc) {
        txt.field(
            "Text Form",
            f.to_string()
                .chars()
                .map(|x| match x {
                    '0' => "zero".to_string(),
                    '1' => "one".to_string(),
                    '2' => "two".to_string(),
                    '3' => "three".to_string(),
                    '4' => "four".to_string(),
                    '5' => "five".to_string(),
                    '6' => "six".to_string(),
                    '7' => "seven".to_string(),
                    '8' => "eight".to_string(),
                    '9' => "nine".to_string(),
                    '.' => "point".to_string(),
                    '-' => "negative".to_string(),
                    _ => x.to_string(),
                })
                .collect::<Vec<_>>()
                .join(", "),
        );
        txt.nl();
    }

    pub fn name(f: u64) -> String {
        const ONES: [&str; 20] = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        const TENS: [&str; 10] = [
            "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
            "ninety",
        ];
        const ORDERS: [&str; 13] = [
            "zero",
            "thousand",
            "million",
            "billion",
            "trillion",
            "quadrillion",
            "quintillion",
            "sextillion",
            "septillion",
            "octillion",
            "nonillion",
            "decillion",
            "undecillion",
        ];

        match f {
            0..=19 => ONES[f as usize].to_string(),
            20..=99 => {
                let u = f / 10;
                match f % 10 {
                    0 => TENS[u as usize].to_string(),
                    l => format!("{}-{}", TENS[u as usize], name(l)),
                }
            }
            100..=999 => format_num(f, 100, "hundred"),
            _ => {
                let (d, o) = std::iter::successors(Some(1u64), |v| v.checked_mul(1000))
                    .zip(ORDERS.iter())
                    .find(|&(e, _)| e > f / 1000)
                    .unwrap();

                format_num(f, d, o)
            }
        }
    }

    fn format_num(n: u64, d: u64, o: &str) -> String {
        match (n / d, n % d) {
            (u, 0) => format!("{} {}", name(u), o),
            (u, l) => format!("{} {}, {}", name(u), o, name(l)),
        }
    }

    pub fn prime_factorization(mut f: u64, txt: &mut Desc) {
        if f >= 1_000_000 {
            return;
        };

        let mut factors = FrequencyTable::new();
        let mut d = 2;

        while f >= 2 {
            if f % d == 0 {
                factors.add(d);
                f /= d;
            } else {
                d += 1;
            }
        }

        txt.field(
            "Prime Factorization",
            factors
                .into_iter()
                .as_vec(|mut x| {
                    x.sort_by_key(|(y, _)| *y);
                    x
                })
                .map(|(x, n)| {
                    if n == 1 {
                        x.to_string()
                    } else {
                        format!("{x}{}", to_subscript(n))
                    }
                })
                .collect::<Vec<_>>()
                .join(" \u{00d7} "),
        );
        txt.nl();
    }

    pub fn to_subscript(t: usize) -> String {
        t.to_string()
            .chars()
            .map(|x| match x {
                '0' => '⁰',
                '2' => '²',
                '1' => '¹',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                _ => unsafe { std::hint::unreachable_unchecked() },
            })
            .collect::<String>()
    }

    pub fn divisors(n: u64, txt: &mut Desc) {
        let v = (1..=n).filter(|&x| n % x == 0);

        if n < 1000000 && n != 0 {
            if v.clone().count() <= 144 {
                txt.field(
                    format!("Divisors ({})", v.clone().count()),
                    v.clone()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
            let max = v.sum::<u64>() - n;
            txt.field("Sum of Divisors", max);
            txt.field(
                "Abundancy",
                match max {
                    c if c > n => format!("Abundant (+{})", c - n),
                    c if c < n => format!("Deficient (-{})", n - c),
                    _ => "Perfect".to_string(),
                },
            );
            txt.nl();
        }
    }

    pub fn f_sets(n: f64, txt: &mut Desc) {
        let mut values = vec!["Real", "Rational"];

        if n.abs().floor() == n.abs() {
            values.push("Integer");
            if n == n.abs() {
                values.push("Whole");
                if n != 0.0 {
                    values.push("Natural");
                }
            }
        }

        txt.field("Containing Sets", values.join(", "));
        txt.nl();
    }

    pub fn f_closest_frac(n: f64, txt: &mut Desc) {
        use fraction::Fraction as F;
        if n != n.round() {
            txt.field("Fraction in lowest terms", F::from(n));
            txt.nl();
        }
    }

    pub fn representations(n: u64, txt: &mut Desc) {
        txt.field("Representations", "");
        txt.field_quote("Binary", format_radix(n, 2));
        txt.field_quote("Octal", format_radix(n, 8));
        txt.field_quote("Duodecimal", format_radix(n, 12));
        txt.field_quote("Hexadecimal", format_radix(n, 16));
        txt.nl();
    }

    pub fn f_operations(n: f64, txt: &mut Desc) {
        txt.field("Operations", "");
        txt.field_quote("Sine", format!("`{}`", n.sin()));
        txt.field_quote("Cosine", format!("`{}`", n.cos()));
        txt.field_quote("Tangent", format!("`{}`", n.tan()));
        if n.is_close_by(n.abs(), 1e-10) {
            txt.field_quote("Absolute Value", format!("`{}`", n.abs()));
        }
        txt.field_quote("Square", format!("`{}`", n.powi(2)));
        if n < 0.0 {
            txt.field_quote("Square Root", format!("`{}i`", n.abs().sqrt()));
        } else {
            txt.field_quote("Square Root", format!("`{}`", n.sqrt()));
        }
        txt.field_quote("Cube", format!("`{}`", n.powi(3)));
        txt.field_quote("Cube Root", format!("`{}`", n.cbrt()));
        if n.round() != n {
            txt.field_quote("Nearest Integer", n.round());

            if n.round() == n.floor() {
                txt.field_quote("Next Integer", n.ceil());
            } else {
                txt.field_quote("Last Integer", n.floor());
            }
        }
        txt.field_quote(
            "Degrees to Radians",
            format!("`{}\u{02b3}`", n.to_radians()),
        );
        txt.field_quote(
            "Radians to Degrees",
            format!("`{}\u{00b0}`", n.to_degrees()),
        );

        txt.nl();
    }
}

mod user {
    use poise::serenity_prelude::{ActivityType, OnlineStatus};
    use serenity::model::Permissions;

    use crate::emojis;

    pub fn status_emoji(online_status: &OnlineStatus) -> &'static str {
        match online_status {
            OnlineStatus::DoNotDisturb => emojis::DoNotDisturb,
            OnlineStatus::Idle => emojis::Idle,
            OnlineStatus::Invisible => emojis::Offline,
            OnlineStatus::Offline => emojis::Offline,
            OnlineStatus::Online => emojis::Online,
            _ => emojis::RichActivity,
        }
    }

    pub fn status_text(online_status: &OnlineStatus) -> &'static str {
        match online_status {
            OnlineStatus::DoNotDisturb => "Do Not Disturb",
            OnlineStatus::Idle => "Idle",
            OnlineStatus::Invisible => "Invisible",
            OnlineStatus::Offline => "Offline",
            OnlineStatus::Online => "Online",
            _ => emojis::RichActivity,
        }
    }

    pub fn activity_kind_text(activity_type: &ActivityType) -> &'static str {
        match activity_type {
            ActivityType::Competing => "Competing",
            ActivityType::Custom => "Custom",
            ActivityType::Listening => "Listening",
            ActivityType::Playing => "Playing",
            ActivityType::Streaming => "Streaming",
            ActivityType::Unknown | _ => "Unknown",
            ActivityType::Watching => "Competing",
        }
    }
}
