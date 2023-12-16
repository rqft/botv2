#![feature(trait_alias)]
#![allow(warnings)]
use std::time::Duration;

use common::{Data, Error};
use poise::{EditTracker, Event, PrefixFrameworkOptions};
use regex::Regex;
use serenity::builder::CreateAllowedMentions;

mod args;
pub mod chart;
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
                // commands::wolfram_alpha::steps(),
                commands::test::cli(),
                commands::combine::combine(),
                // commands::gplot::gplot(),
                // commands::gplot::gplot_slash(),
                commands::info::info(),
                commands::colours::colors(),
                commands::rng::flip(),
                commands::rng::roll(),
                commands::rng::pick(),
                commands::latex::latex(),
                // commands::test::s(),
                // commands::plot::plot2(),
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
            event_handler: |c, e, v, u| Box::pin(async move { Ok(()) }),

            ..Default::default()
        })
        .token(dotenv::var("d_token").expect("missing d_token"))
        .intents(serenity::model::gateway::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // for c in ctx.http.get_global_application_commands().await? {
                //     println!("+ {}", c.name);
                //     ctx.http.delete_global_application_command(c.id.0).await?;
                //     println!("- {}", c.name);
                // }
                poise::builtins::register_globally(ctx, &framework.options().commands).await?; // add back later
                println!("ok!");
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
