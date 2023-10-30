use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use serde::Deserialize;

use crate::{
    common::{Context, Output},
    ext::{Do, RegexExt},
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// combine two emojis via Google Emoji Kitchen
pub async fn combine(context: Context<'_>, emoji_a: String, emoji_b: String) -> Output {
    context.say(get_emoji_mix_url(emoji_a, emoji_b)?).await?;
    Ok(())
}

pub const BASE_URL: &str = "https://www.gstatic.com/android/keyboard/emojikitchen";

pub fn google_request_url_emoji_part(emoji: impl Display) -> String {
    format!("{emoji}")
        .split('-')
        .map(|x| format!("u{}", x.to_lowercase()))
        .collect::<Vec<_>>()
        .join("-")
}

pub fn to_unicode(input: impl Display) -> Option<String> {
    let i = format!("{input}");

    if i.clone().test("^[\\da-fA-F]+$]") {
        Some(i.to_lowercase())
    } else if i.clone().test("\\p{Emoji}") {
        i.chars()
            .next()
            .map(|x| format!("{:x}", crate::ext::To::to::<u32>(x)))
    } else {
        None
    }
}

pub fn emoji_compatibility_data() -> HashMap<String, Vec<EmojiData>> {
    crate::huge_lists::emoji_compatibility_data()
}

pub fn supported_emojis() -> Vec<&'static str> {
    crate::huge_lists::supported_emojis()
}

#[derive(Clone, Debug, Deserialize)]
pub struct EmojiData {
    #[serde(rename = "leftEmoji")]
    pub left_emoji: String,
    #[serde(rename = "rightEmoji")]
    pub right_emoji: String,
    pub date: String,
}

pub fn check_supported(emoji: impl Display) -> Option<Vec<EmojiData>> {
    let u = to_unicode(emoji);

    u.and_then(|x| emoji_compatibility_data().get(x.as_str()).cloned())
}

pub fn google_request_url(emoji_mix_data: EmojiData) -> String {
    let leup = google_request_url_emoji_part(&emoji_mix_data.left_emoji);
    let reup = google_request_url_emoji_part(&emoji_mix_data.right_emoji);

    format!(
        "{BASE_URL}/{}/{leup}/{leup}_{reup}.png",
        emoji_mix_data.date
    )
}

pub fn get_emoji_combo(left_emoji: impl Display, right_emoji: impl Display) -> Option<EmojiData> {
    emoji_compatibility_data()
        .get(format!("{right_emoji}").as_str())
        .map(|x| {
            x.iter()
                .filter(|x| {
                    (x.left_emoji == format!("{left_emoji}")
                        && (x.right_emoji == format!("{right_emoji}")))
                        || (x.left_emoji == format!("{right_emoji}")
                            && (x.right_emoji == format!("{left_emoji}")))
                })
                .cloned()
                .collect::<Vec<_>>()
                .do_mut(|f| {
                    f.sort_by(|a, b| {
                        if a.date > b.date {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                })
        })
        .and_then(|x| x.get(0).cloned())
}

pub fn get_emoji_mix_url(
    left_emoji: impl Display,
    right_emoji: impl Display,
) -> Result<String, &'static str> {
    let le = to_unicode(left_emoji);
    let re = to_unicode(right_emoji);

    le.as_ref().ok_or("left emoji argument is incompatible")?;
    re.as_ref().ok_or("right emoji argument is incompatible")?;

    // dbg!(&le);
    if !supported_emojis().contains(&le.as_ref().unwrap().as_str()) {
        return Err("left emoji argument is not supported");
    }

    if !supported_emojis().contains(&re.as_ref().unwrap().as_str()) {
        return Err("left emoji argument is not supported");
    }

    get_emoji_combo(le.as_ref().unwrap(), re.as_ref().unwrap())
        .ok_or("this emoji combination is not supported")
        .map(google_request_url)
}
