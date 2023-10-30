#![feature(trait_alias, async_closure)]
use std::time::Duration;

use common::{Data, Error};
use poise::{EditTracker, Event, PrefixFrameworkOptions};
use regex::Regex;
use serenity::builder::CreateAllowedMentions;

mod args;
mod commands;
mod common;
mod embed_preset;
mod emojis;
mod ext;
mod get_image;
mod huge_lists;
mod paginator;
// mod wrap;
#[tokio::main]
pub async fn main() {
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::plot::plot(),
                commands::plot::plot_slash(),
                commands::image_url::url(),
                commands::tags::tags(),
                commands::categories::categories(),
                commands::ocr::ocr(),
                commands::plot::math(),
                commands::wolfram_alpha::wa(),
                commands::wolfram_alpha::answer(),
                commands::wolfram_alpha::steps(),
                commands::test::test(),
                commands::combine::combine(),
                commands::gplot::gplot(),
                commands::gplot::gplot_slash(),
                commands::info::info(),
                commands::colours::colors(),
                commands::rng::flip(),
                commands::rng::roll(),
                commands::rng::pick(),
                commands::latex::latex(),
                commands::test::crame(),
            ],
            allowed_mentions: Some(
                CreateAllowedMentions::default()
                    .replied_user(false)
                    .empty_users()
                    .empty_roles()
                    .to_owned(),
            ),
            prefix_options: PrefixFrameworkOptions {
                case_insensitive_commands: true,
                execute_untracked_edits: true,
                mention_as_prefix: true,
                edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(5 * 60))),
                prefix: Some("~".to_owned()),
                ..Default::default()
            },
            reply_callback: Some(|_, y| {
                y.allowed_mentions(|x| x.replied_user(false)).reply(true);
            }),
            event_handler: |c, e, v, u| {
                Box::pin(async move {
                    if let Event::Message { new_message } = e {
                        if new_message.guild_id.map(|x| x.0) == Some(1028450117512085616)
                            || new_message.guild_id.is_none()
                        {
                            let mat =
                                Regex::new("https?:\\/\\/(x\\.com|twitter\\.com)\\/(.+)").unwrap();
                            let url = &new_message.content;
                            if mat.is_match(&url) {
                                new_message
                                    .reply(c, mat.replace_all(&url, "https://fxtwitter.com/$2"))
                                    .await?;
                            };
                        }
                    };

                    Ok(())
                })
            },

            ..Default::default()
        })
        .token(dotenv::var("d_token").expect("missing d_token"))
        .intents(serenity::model::gateway::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    ucd: ucd::Ucd::new(),
                    req: reqwest::Client::new(),
                    imagga: imagga::Imagga::new(dotenv::var("i_token").expect("missing i_token")),
                    wolfram: wa::Wolfram::new(dotenv::var("w_token").expect("missing w_token")),
                })
            })
        });

    framework.run().await.unwrap();
}
