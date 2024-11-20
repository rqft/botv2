use std::str::FromStr;
use std::time::{Duration, Instant};

use anyhow::bail;

use crate::common::{Context, Output};
use crate::sf::ren::{pathfind, State};
use crate::sf::{are_grids_equal, parse_grid, render_grid, resp, to_grid, Piece};
#[poise::command(prefix_command, slash_command, track_edits)]
/// render a tetris play field
pub async fn grid(
    context: Context<'_>,
    grid: String,
    delay: Option<f64>,
    lcs: Option<bool>,
    lp: Option<bool>,
) -> Output {
    // context.reply("ok");
    // let gif = render_grid(
    //     &grid,
    //     lp.unwrap_or(false),
    //     true,
    //     lcs.unwrap_or(true),
    //     Duration::from_secs_f64(delay.unwrap_or(0.25)),
    // );

    context
        .send(|v| {
            // v.attachment(poise::serenity_prelude::AttachmentType::Bytes {
            // data: gif.into(),
            // filename: "render.gif".to_string(),
            // })
            let is_multi_frame = grid.contains(';');
            v.content(format!(
                "\u{200b}https://qv.rqft.workers.dev/render.{}?grid={grid}&delay={}&clear={}&loop={}",
                if is_multi_frame { "gif" } else { "png" },
                delay.unwrap_or(500.0),
                lcs.unwrap_or(true),
                lp.unwrap_or(true)
            ))
        })
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// translate between fumen format and grid format
pub async fn fumen(context: Context<'_>, data: String) -> Output {
    let is_fumen = data.starts_with("v1");
    let text = context
        .data()
        .req
        .get(format!("https://qv.rqft.workers.dev/convert?data={data}"))
        .send()
        .await?
        .text()
        .await?;

    context
        .send(|v| {
            if is_fumen {
                v.content(format!(
                    "{text}\n[image](https://qv.rqft.workers.dev/render.gif?grid={text})",
                ))
            } else {
                v.content(format!("{text}\n[link](https://harddrop.com/fumen/?{text}) | [image](https://qv.rqft.workers.dev/fumen.gif?data={text})"))
            }
        })
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// find combos
pub async fn sf(context: Context<'_>, pattern: String, queue: Option<String>) -> Output {
    // let q = queue
    //     .chars()
    //     .map(|x| Piece::from_str(&x.to_string()))
    //     .collect::<Vec<_>>();

    let rs = resp();
    // println!("bbb");
    let pat = if pattern.contains('#') {
        let mut p = pattern.split('#');
        let res = p.next().unwrap().parse().unwrap();
        let id = p.next().unwrap();

        rs.get(&res)
            .unwrap()
            .iter()
            .find(|x| x.id == id)
            .map(|x| (res, x))
    } else {
        let pg = parse_grid(pattern);
        let cnt = pg
            .iter()
            .map(|x| x.iter().filter(|y| y == &&Piece::G).count())
            .reduce(|a, b| a + b)
            .unwrap();
        let pat = rs
            .get(&(cnt as u8))
            .unwrap()
            .iter()
            .find(|x| are_grids_equal(x.grid.clone(), pg.clone()));

        if let Some(pat) = pat {
            Some((cnt as u8, pat))
        } else {
            None
        }
    };

    if pat.is_none() {
        return Err("unknown pattern".into());
    }

    // println!("aaa");

    let (res, bo) = pat.unwrap();

    // dbg!(resp().get(&3).unwrap());

    if let Some(q) = queue {
        let mut parts = q.split('@');
        let (q, h) = (
            parts
                .next()
                .unwrap()
                .chars()
                .map(|x| Piece::from_str(&x.to_string()).unwrap())
                .collect::<Vec<_>>(),
            parts.next().map(|x| Piece::from_str(x).unwrap()),
        );

        let st = Instant::now();
        let list = pathfind(State {
            board: bo.clone(),
            patterns: resp().get(&res).unwrap().to_vec(),
            queue: q.clone(),
            hold: h,
        });
        let es = st.elapsed();

        let grid = sequence_grids(list.iter().map(|x| x.2.clone()).collect());

        context
            .send(|v| {
                v.content(format!(
                    "[res={}, {}/{}, {:.3}s] {}\n[link]({}) | [image]({})",
                    res,
                    list.len(),
                    q.len(),
                    es.as_secs_f64(),
                    list.iter()
                        .map(|x| format!("{}{}", x.0.id.clone(), x.1))
                        .collect::<Vec<_>>()
                        .join(", "),
                    format!("<https://qv.rqft.workers.dev/list/ren/{res}/{}>", bo.id),
                    format!("https://qv.rqft.workers.dev/render.png?grid={grid}&clear=true",)
                ))
            })
            .await?;
    } else {
        let gif = render_grid(
            &to_grid(&bo.grid),
            false,
            true,
            false,
            Duration::from_secs_f64(0.25),
        );

        context
            .send(|v| {
                v.attachment(poise::serenity_prelude::AttachmentType::Bytes {
                    data: gif.into(),
                    filename: "render.gif".to_string(),
                })
                .content(format!("{}#{}", res, bo.id))
            })
            .await?;
    }
    Ok(())
}

fn sequence_grids(mut grids: Vec<Vec<Vec<Piece>>>) -> String {
    let mut s = String::new();
    let max = grids.iter().map(|x| x.len()).max().unwrap_or(0);
    for grid in &mut grids {
        let cmax = grid.iter().map(|x| x.len()).max().unwrap_or(0);
        while grid.len() < max {
            *grid = vec![vec![vec![Piece::E; cmax]], grid.clone()].concat()
        }
    }

    for i in 0..max {
        for (j, grid) in grids.iter().enumerate() {
            let row = &grid[i]; // should be ok now
            for col in row {
                s += &col.to_string();
            }

            s += "E";
        }

        s += "|"
    }
    s
}
