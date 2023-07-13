use poise::{serenity_prelude::StickerFormatType, PrefixContext};
use regex::Regex;

use crate::common::{to_code_point_for_twemoji, Context};

// use crate::common::Context;

#[async_recursion::async_recursion]
pub async fn find_media_urls<'a>(
    kind: Vec<String>,
    context: &'a Context<'a>,
    text: Option<String>,
    in_search: bool,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    if let poise::Context::Prefix(b) = context {
        let msg = b.msg;

        for attachment in &msg.attachments {
            if kind.iter().any(|x| attachment.url.ends_with(x)) {
                out.push(attachment.proxy_url.clone());
            }
        }

        if text == Some("-".to_string()) {
            return out;
        }

        if let Some(re) = &msg.referenced_message {
            out.extend(
                find_media_urls(
                    kind.clone(),
                    &poise::Context::Prefix(PrefixContext { msg: re, ..*b }),
                    Some(re.content.clone()),
                    true,
                )
                .await,
            )
        }

        for embed in &msg.embeds {
            if kind
                .iter()
                .any(|x| ["png", "jpg", "jpeg", "webp"].contains(&&**x))
            {
                if let Some(i) = &embed.image {
                    out.push(i.url.clone());
                }

                if let Some(i) = &embed.thumbnail {
                    out.push(i.url.clone());
                }
            }
        }

        for sticker_item in &msg.sticker_items {
            if kind
                .iter()
                .any(|x| ["png", "jpg", "jpeg", "webp"].contains(&&**x))
                && sticker_item.format_type == StickerFormatType::Png
            {
                out.push(sticker_item.image_url().unwrap())
            }
        }
    }

    if let Some(txt) = &text {
        let uri = url::Url::parse(txt);

        if let Ok(url) = uri {
            if kind.iter().any(|x| {
                ["wav", "mp3", "flac", "ogg"].contains(&&**x)
                    && url.path_segments().unwrap().last().unwrap().ends_with(x)
            }) {
                out.push(txt.to_string());
            }

            if kind.iter().any(|x| {
                ["png", "jpg", "jpeg", "webp"].contains(&&**x)
                    && url.path_segments().unwrap().last().unwrap().ends_with(x)
            }) {
                out.push(txt.to_string());
            }

            if kind.iter().any(|x| {
                ["mp4", "m4a"].contains(&&**x)
                    && url.path_segments().unwrap().last().unwrap().ends_with(x)
            }) {
                out.push(txt.to_string());
            }
        }

        let users = context.serenity_context().cache.users();
        let found = users.iter().find(|x| {
            x.tag().to_lowercase().contains(&txt.to_lowercase())
                || x.id.to_string() == txt.replace(['<', '@', '!', '>'], "")
        });

        if let Some(found) = found {
            if kind
                .iter()
                .any(|x| ["png", "jpg", "jpeg", "webp"].contains(&&**x))
            {
                out.push(
                    found
                        .avatar_url()
                        .unwrap_or_else(|| found.default_avatar_url()),
                );
            }
        }

        let emoji_regex = Regex::new("<a?:.+:(\\d+)>").expect("guaranteed valid regex?");

        if let Some(cap) = emoji_regex.captures(txt) {
            if let Some(v) = cap.get(0) {
                if kind
                    .iter()
                    .any(|x| ["png", "jpg", "jpeg", "webp", "gif"].contains(&&**x))
                {
                    out.push(format!(
                        "https://cdn.discordapp.com/emojis/{}.{}",
                        v.as_str(),
                        if txt.starts_with("<a") { "gif" } else { "png" }
                    ))
                }
            }
        }

        let unicode_emoji_regex = Regex::new("\\p{Emoji}").expect("nope.");

        if let Some(cap) = dbg!(unicode_emoji_regex.captures(txt)) {
            if let Some(v) = cap.get(0) {
                if kind
                    .iter()
                    .any(|x| ["png", "jpg", "jpeg", "webp", "gif"].contains(&&**x))
                {
                    out.push(format!(
                        "https://cdn.notsobot.com/twemoji/512x512/{}.png",
                        to_code_point_for_twemoji(v.as_str())
                    ))
                }
            }
        }
    }

    // dbg!(in_search, &text);
    if let Context::Prefix(v) = context {
        let msg = v.msg;
        if !in_search
            && (text.is_none() || text == Some("".to_string()) || text == Some("^".to_string()))
        {
            let q = format!("?before={}&limit=25", msg.id);
            let messages = context
                .serenity_context()
                .http
                .get_messages(msg.channel_id.into(), &q)
                .await
                .unwrap();

            for msg in messages {
                // println!("{}", msg.id);
                out.extend(
                    find_media_urls(
                        kind.clone(),
                        &poise::Context::Prefix(PrefixContext { msg: &msg, ..*v }),
                        Some(msg.content.clone()),
                        true,
                    )
                    .await,
                );
            }
        }
    }

    out
}
