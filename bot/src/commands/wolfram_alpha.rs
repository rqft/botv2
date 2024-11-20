use std::marker::PhantomData;

use regex::Captures;
use serenity::futures::TryFutureExt;
use wa::model::{Pod, Subpod};

use crate::{
    common::{Context, Output},
    embed_preset::user,
    ext::{Do, RegexExt},
    paginator::{Inter, Paginator, PaginatorOptions},
};

#[derive(Clone, Debug)]
struct Page<'a> {
    pod: &'a Pod,
    subpod: &'a Subpod,
}

fn lose_if_empty(x: Option<String>) -> Option<String> {
    if let Some(ref s) = x
        && s.is_empty()
    {
        None
    } else {
        x
    }
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// wolfram alpha
pub async fn wa(context: Context<'_>, input: Vec<String>) -> Output {
    let join = input.join(" ");
    let mut i = join.trim();
    let steps = if let Some(x) = i.strip_suffix("--steps") {
        i = x;
        true
    } else {
        false
    };

    dbg!(steps);
    let mut value = context
        .data()
        .wolfram
        .query(wa::model::QueryOptions {
            input: i.to_string(),
            format: Some("plaintext,image,minput".to_string()),
            mag: Some(1.5),
            width: Some(1500),
            podstate: if steps {
                Some("Result__Step-by-step solution".to_string())
            } else {
                None
            },
            ..Default::default()
        })
        .await
        .map_err(|x| format!(":warning: Failed to get result: ```\n{}\n```", x))?;
    if value.queryresult.pods.is_none() {
        context.reply(format!("No results found.")).await?;

        return Ok(());
    }

    let p = &mut value.queryresult.pods.unwrap();
    dbg!(&p);
    let pages = p
        .iter()
        .map(|pod| pod.subpods.iter().map(|subpod| Page { pod, subpod }))
        .flatten()
        .collect::<Vec<_>>();

    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| pages.clone()[page - 1].clone(),
            on_page: |page: usize, data: Page, mut ctx: Inter| {
                let mut txt = String::new();

                txt += &*format!(
                    "__{}__ ({}), page {}/{}",
                    data.pod.title,
                    data.pod.scanner,
                    page,
                    pages.len()
                );

                txt += "\n";

                txt += &*format!(
                    "{}{}{}{}",
                    if data.subpod.title.trim() == "" {
                        String::new()
                    } else {
                        format!("**{}**", data.subpod.title)
                    },
                    data.subpod
                        .img
                        .as_ref()
                        .map(|x| format!(" ([image]({}))", x.src))
                        .unwrap_or(String::new()),
                    data.subpod.minput.as_ref().map(|x| format!("\n`{x}`")).unwrap_or(String::new()),
                    data.subpod
                        .plaintext
                        .as_ref()
                        .map(|x| format!(
                            "\n\n{}",
                            x.str_replace("[*_~\\-`]", |x: &Captures| format!(
                                "\\{}",
                                x.get(0).unwrap().as_str()
                            ))
                        ))
                        .unwrap_or(String::new())
                )
                .trim();

                ctx.content(txt);

                ctx
            },
            page_limit: pages.len(),
            targets: None,
            none: PhantomData,
        },
    );

    pn.start().await;
    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits, aliases("q"))]
/// wolfram alpha (short answer)
pub async fn answer(context: Context<'_>, input: Vec<String>) -> Output {
    let value = context
        .data()
        .wolfram
        .short_answer(input.join(" "), "metric")
        .await?;

    if let Some(v) = value {
        context.say(v).await?;
    } else {
        context
            .say(":warning: Was either unable to interpret the input, or no answer was found")
            .await?;
    }
    Ok(())
}
