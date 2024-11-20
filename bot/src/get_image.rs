use poise::{
    serenity_prelude::{Context as SerenityContext, Message, StickerFormatType, User},
    ApplicationContext, PopArgument, PrefixContext,
};
use regex::Regex;
use serenity::http::CacheHttp;
use url::Url;

use crate::common::{Context, Output};
use crate::{common::to_code_point_for_twemoji, ext::RegexExt};

pub enum ContextOrMessage<'a> {
    Context(Context<'a>),
    Message(Message),
}

#[async_recursion::async_recursion]
pub async fn crawl_urls<'a>(
    kind: &[&str],
    context: &'a ContextOrMessage<'a>,
    sc: SerenityContext,
    text: Option<String>,
    in_search: bool,
    size: Option<usize>,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

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

        let users = sc.cache.users();
        let found = users.iter().find(|x| {
            x.tag().to_lowercase().contains(&txt.to_lowercase())
                || x.id.to_string() == txt.replace(['<', '@', '!', '>'], "")
        });

        if let Some(found) = found {
            let u_url = found
                .avatar_url()
                .unwrap_or_else(|| found.default_avatar_url());

            use crate::ext::RegexExt;
            if !in_search {
                let ur = u_url
                    .str_replace("[?&]size=\\d*$", "")
                    .str_replace("\\.webp", ".png");
                if (ur.ends_with("png") && is_kind(kind, "png"))
                    || (ur.ends_with("gif") && is_kind(kind, "gif"))
                {
                    out.push(ur);
                }
            }
        }

        let emoji_regex = Regex::new("<a?:.+:(\\d+)>").expect("guaranteed valid regex?");

        if let Some(cap) = emoji_regex.captures(txt) {
            if let Some(v) = cap.get(1) {
                if (txt.starts_with("<a") && is_kind(kind, "gif"))
                    || (!txt.starts_with("<a") && is_kind(kind, "gif"))
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

        if let Some(cap) = unicode_emoji_regex.captures(txt) {
            if let Some(v) = cap.get(0) {
                if is_kind(kind, "png") {
                    if !in_search {
                        out.push(format!(
                            "https://cdn.notsobot.com/twemoji/512x512/{}.png",
                            to_code_point_for_twemoji(v.as_str())
                        ))
                    }
                }
            }
        }
    }

    let msg = match context {
        ContextOrMessage::Context(c) => match c {
            Context::Prefix(p) => Some(p.msg),
            Context::Application(a) => None,
        },

        ContextOrMessage::Message(m) => Some(m),
    };

    if let Some(msg) = msg {
        for attachment in &msg.attachments {
            let u = Url::parse(&attachment.url).unwrap();
            if kind.iter().any(|x| u.path().ends_with(x)) || kind.is_empty() {
                out.push(attachment.proxy_url.clone());
            }
        }

        if text == Some("-".to_string()) {
            return out;
        }

        if let Some(re) = &msg.referenced_message {
            out.extend(
                crawl_urls(
                    kind.clone(),
                    &ContextOrMessage::Message(*re.clone()),
                    sc.clone(),
                    Some(re.content.clone()),
                    true,
                    size,
                )
                .await,
            )
        }

        for embed in &msg.embeds {
            {
                if let Some(i) = &embed.image {
                    let u = Url::parse(&i.url).unwrap();
                    if (kind.iter().any(|x| u.path().ends_with(x)) || kind.is_empty()) {
                        out.push(i.url.clone());
                    }
                }

                if let Some(i) = &embed.thumbnail {
                    let u = Url::parse(&i.url).unwrap();
                    if (kind.iter().any(|x| u.path().ends_with(x)) || kind.is_empty()) {
                        out.push(i.url.clone());
                    }
                }
            }
        }

        for sticker_item in &msg.sticker_items {
            if (is_kind(kind, "png") && sticker_item.format_type == StickerFormatType::Png) {
                out.push(sticker_item.image_url().unwrap())
            }
        }
    }

    let id = match context {
        ContextOrMessage::Context(c) => c.id(),
        ContextOrMessage::Message(m) => m.id.0,
    };

    let chid = match context {
        ContextOrMessage::Context(c) => c.channel_id(),
        ContextOrMessage::Message(c) => c.channel_id,
    };

    if !in_search
        && (text.is_none() || text == Some(String::new()) || text == Some("^".to_string()))
    {
        let q = format!("?before={}&limit=25", id);
        let mut messages = sc.http().get_messages(chid.into(), &q).await.unwrap();

        messages.sort_by(|a, b| {
            a.id.0
                .partial_cmp(&b.id.0)
                .unwrap_or(std::cmp::Ordering::Equal).reverse()
        });

        for msg in messages {
            // println!("{}", msg.id);
            out.extend(
                crawl_urls(
                    kind.clone(),
                    &ContextOrMessage::Message(msg),
                    sc.clone(),
                    None,
                    true,
                    size,
                )
                .await,
            );
        }
    }

    dbg!(&out);

    out
}

pub fn is_kind(haystack: &[&str], needle: &str) -> bool {
    haystack.is_empty() || haystack.contains(&needle)
}

pub async fn find_media_urls<'a>(
    kind: &[&str],
    context: &'a Context<'a>,
    text: Option<String>,
    size: Option<usize>,
) -> Vec<String> {
    return crawl_urls(
        kind,
        &ContextOrMessage::Context(*context),
        context.serenity_context().clone(),
        text,
        false,
        size,
    )
    .await;
}

pub async fn get_media_data<'a>(
    kind: &[&str],
    context: &'a Context<'a>,
    text: Option<String>,
    size: Option<usize>,
) -> Output<bytes::Bytes> {
    let v = find_media_urls(kind, context, text, size).await;
    let urls = v.first().ok_or("no media found")?;

    Ok(context.data().req.get(urls).send().await?.bytes().await?)
}
