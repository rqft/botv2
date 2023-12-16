use std::marker::PhantomData;

use serenity::futures::TryFutureExt;
use wa::model::Pod;

use crate::{
    common::{Context, Output},
    embed_preset::user,
    paginator::{Inter, Paginator, PaginatorOptions},
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// wolfram alpha
pub async fn wa(context: Context<'_>, input: Vec<String>) -> Output {
    let mut value = context
        .data()
        .wolfram
        .query(wa::model::QueryOptions {
            input: input.join(" "),
            format: Some("plaintext,image,minput".to_string()),
            ..Default::default()
        })
        .await
        .map_err(|_| ":warning: Failed to get result")?;
    let p = &mut value.queryresult.pods;

    p.sort_by_key(|v| v.scanner == "Identity");

    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| p.clone()[page - 1].clone(),
            on_page: |page: usize, data: Pod, mut ctx: Inter| {
                let mut txt = String::new();
                txt += &*format!(
                    "__{}__ ({}), page {}/{}",
                    data.title,
                    data.scanner,
                    page,
                    p.len()
                );

                for pod in data.subpods {
                    if txt.len() > 1998 {
                        break;
                    }

                    txt += "\n";

                    txt += &*format!(
                        "{}{}{}",
                        if pod.title.trim() == "" {
                            String::new()
                        } else {
                            format!("**{}**", pod.title)
                        },
                        pod.img
                            .map(|x| format!(" ([image]({}))", x.src))
                            .unwrap_or(String::new()),
                        pod.plaintext
                            .map(|x| format!("\n\n{x}"))
                            .unwrap_or(String::new())
                    )
                    .trim();
                }

                ctx.content(txt);

                ctx
            },
            page_limit: p.len(),
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
