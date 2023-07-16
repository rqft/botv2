#![feature(trait_alias)]
use std::time::Duration;

use common::{Data, Error};
use poise::{EditTracker, PrefixFrameworkOptions};
use serenity::builder::CreateAllowedMentions;

mod args;
mod commands;
mod common;
mod embed_preset;
mod ext;
mod get_image;
pub mod paginator;
// mod wrap;
#[tokio::main]
pub async fn main() {
    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::plot::plot(),
                commands::plot::plot_slash(),
                commands::image_url::image_url(),
                commands::image_tags::image_tags(),
                commands::ocr::ocr(),
                commands::plot::math(),
                commands::wolfram_alpha::wa(),
                commands::test::test()
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
            ..Default::default()
        })
        .token(dotenv::var("d_token").expect("missing d_token"))
        .intents(serenity::model::gateway::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    imagga: imagga::Imagga::new(dotenv::var("i_token").expect("missing i_token")),
                    wolfram: wa::Wolfram::new(dotenv::var("w_token").expect("missing w_token")),
                })
            })
        });

    framework.run().await.unwrap();
}
