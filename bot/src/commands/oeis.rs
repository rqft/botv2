use std::marker::PhantomData;

use crate::{
    common::{Context, Error, Output},
    paginator::{Inter, Paginator, PaginatorOptions},
};
use serde::Deserialize;

#[poise::command(prefix_command, slash_command, track_edits)]
/// search the online encyclopedia of integer sequences
pub async fn oeis(context: Context<'_>, text: Vec<String>) -> Output {
    let seq = text.join(" ");
    let list = seq
        .split(",")
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let initial = get_ten(context, &seq, 0).await?;

    if initial.results.is_none() {
        return Err(format!(
            "Found {} results, too many to show. Please refine your search.",
            initial.count
        )
        .into());
    }

    let mut results = vec![];
    let total = initial.count;

    let mut idx = 0;
    while idx < total && idx <= 25 {
        let v = get_ten(context, &seq, idx).await?;
        if let Some(r) = v.results {
            for j in r {
                results.push(j);
                idx += 1;
            }
        } else {
            break;
        }
    }

    let pn = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Default::default(),
            expires: 300000,
            is_ephemeral: None,
            get_page: |page: usize| &results[page - 1],
            on_page: |page: usize, data: &OEISResult, mut ctx: Inter| {
                let mut txt = String::new();
                txt += &format!(
                    "**{}** [`A{:06}`](<https://oeis.org/{:06}>), page {}/{}\n",
                    data.name,
                    data.number,
                    data.number,
                    page,
                    results.len()
                );
                let fl = data
                    .data
                    .split(",")
                    .map(|x| {
                        x.trim().parse().map(|y| {
                            if list.contains(&y) {
                                format!("**{y}**")
                            } else {
                                y.to_string()
                            }
                        })
                    })
                    .collect::<Result<Vec<String>, _>>()
                    .unwrap();
                txt += &format!("```\n{}\n```\n", &data.data);

                if let Some(c) = &data.comment {
                    for i in 0..5 {
                        if let Some(v) = c.get(i) {
                            txt += &format!("*{v}*\n\n");
                        }
                    }
                }

                ctx.content(txt);
                ctx
            },
            page_limit: results.len(),
            targets: None,
            none: PhantomData,
        },
    );

    pn.start().await;

    Ok(())
}

async fn get_ten(context: Context<'_>, seq: &str, start: usize) -> Result<OEIS, Error> {
    Ok(context
        .data()
        .req
        .get("https://oeis.org/search")
        .query(&[("fmt", "json"), ("q", seq), ("start", &start.to_string())])
        .send()
        .await?
        .json()
        .await?)
}

#[derive(Deserialize, Clone, Debug)]
pub struct OEIS {
    pub greeting: String,
    pub query: String,
    pub count: usize,
    pub start: usize,
    pub results: Option<Vec<OEISResult>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OEISResult {
    pub number: usize,
    pub id: Option<String>,
    pub data: String,
    pub name: String,
    pub comment: Option<Vec<String>>,
    pub link: Option<Vec<String>>,
    pub formula: Option<Vec<String>>,
    pub mathematica: Option<Vec<String>>,
    pub program: Option<Vec<String>>,
    pub xref: Option<Vec<String>>,
    pub keyword: String,
    pub offset: String,
    pub author: String,
    pub references: usize,
    pub revision: usize,
    pub time: String,
    pub created: String,
}
