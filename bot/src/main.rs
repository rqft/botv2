#![feature(trait_alias, let_chains)]
#![allow(warnings)]
use std::time::Duration;

use common::{Data, Error};
use poise::{serenity_prelude::Message, EditTracker, Event, PrefixFrameworkOptions};
use regex::Regex;
use serde_json::Map;
use serenity::{
    builder::CreateAllowedMentions,
    client::{Context, EventHandler},
};

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
pub mod sf;
// mod wrap;
#[tokio::main]
pub async fn main() {
    struct Handle;
    #[serenity::async_trait]
    impl EventHandler for Handle {
        async fn message(&self, ctx: Context, msg: Message) {
            if !msg.is_own(&ctx) && (msg.mentions_me(&ctx).await.unwrap() || msg.author.id == 504698587221852172)
            {
                let fumreg = Regex::new("v1\\d+@.+?\\b").unwrap();
                let found = fumreg
                    .find_iter(&msg.content)
                    .enumerate()
                    .map(|(i, x)| {
                        format!(
                            "[fumen {i}](https://qv.rqft.workers.dev/fumen.gif?data={}",
                            x.as_str()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" | ");
                msg.reply(ctx, found).await.unwrap();
            }
        }
    }
    let framework = poise::Framework::<Data, Error>::builder()
        .client_settings(|f| f.event_handler(Handle))
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
                commands::test::test(),
                commands::combine::combine(),
                // commands::gplot::gplot(),
                // commands::gplot::gplot_slash(),
                commands::info::info(),
                commands::colours::colors(),
                commands::rng::coin(),
                commands::rng::roll(),
                commands::rng::pick(),
                commands::latex::latex(),
                // commands::test::s(),
                // commands::plot::plot2(),
                commands::image::invert(),
                commands::image::flip(),
                commands::image::flop(),
                commands::oeis::oeis(),
                commands::tetr::grid(),
                commands::tetr::sf(),
                commands::tetr::fumen(),
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
            on_error: |t| Box::pin(async move {}),
            ..<_>::default()
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
