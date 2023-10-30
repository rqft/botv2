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
    let value = context
        .data()
        .wolfram
        .query(wa::model::QueryOptions {
            input: input.join(" "),
            format: Some("plaintext,image".to_string()),
            ..Default::default()
        })
        .await
        .map_err(|_| ":warning: Failed to get result")?;
    let p = &value.queryresult.pods;
    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| p.clone()[page - 1].clone(),
            on_page: |page: usize, data: Pod, mut ctx: Inter| {
                ctx.embed(|b| {
                    let x = user(context, b)
                        .title(data.title.clone())
                        .footer(|x| x.text(format!("Page {}/{}", page, p.len())));

                    if let Some(v) = data.subpods.get(0) {
                        if let Some(i) = &v.img {
                            x.image(i.src.clone());
                        }
                    }

                    x.fields(
                        data.subpods
                            .iter()
                            .filter(|x| x.plaintext.is_some())
                            .map(|x| (x.title.clone(), x.plaintext.clone().unwrap(), true)),
                    );

                    x
                });

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

#[poise::command(prefix_command, slash_command, track_edits)]
/// wolfram alpha (steps)
pub async fn steps(context: Context<'_>, input: Vec<String>) -> Output {
    let value = context
        .data()
        .wolfram
        .query(wa::model::QueryOptions {
            input: input.join(" "),
            format: Some("plaintext,image".to_string()),
            podstate: Some("Result__Step-by-step solution".to_string()),
            ..Default::default()
        })
        .await
        .map_err(|_| ":warning: Failed to get result")?;
    let p = &value.queryresult.pods;
    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| p.clone()[page - 1].clone(),
            on_page: |page: usize, data: Pod, mut ctx: Inter| {
                ctx.embed(|b| {
                    let x = user(context, b)
                        .title(data.title.clone())
                        .footer(|x| x.text(format!("Page {}/{}", page, p.len())));

                    if let Some(v) = data.subpods.get(0) {
                        if let Some(i) = &v.img {
                            x.image(i.src.clone());
                        }
                    }

                    x.fields(
                        data.subpods
                            .iter()
                            .filter(|x| x.plaintext.is_some())
                            .map(|x| (x.title.clone(), x.plaintext.clone().unwrap(), true)),
                    );

                    x
                });

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
