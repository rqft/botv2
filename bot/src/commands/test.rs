use std::{borrow::Cow, io::Cursor, marker::PhantomData};

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use plotters::prelude::*;
use poise::serenity_prelude::AttachmentType;

use crate::common::{Context, Output};

#[poise::command(prefix_command, track_edits)]
/// pong
pub async fn cli(context: Context<'_>, expr: Vec<String>) -> Output {
    if context.author().id != 504698587221852172u64 {
        return Ok(());
    }

    let r_expr = expr
        .iter()
        .cloned()
        .filter(|y| !y.starts_with("-f="))
        .collect::<Vec<_>>()
        .join(" ");
    let r_fmt = expr.iter().find_map(|y| y.strip_prefix("-f="));

    let mut cmd = std::process::Command::new(
        // gl!
        r"C:\Program Files\Wolfram Research\WolframScript\wolframscript.exe",
    );
    cmd.arg("-cloud").args(["-code", &r_expr]);

    if let Some(fmt) = r_fmt {
        println!("-format {fmt}");
        cmd.args(["-format", fmt]);
    }

    println!("{cmd:?}");

    let o = cmd.output()?;

    if let Some(fmt) = r_fmt {
        context
            .send(|x| {
                x.reply(true).attachment(AttachmentType::Bytes {
                    data: Cow::Borrowed(&o.stdout),
                    filename: "output.png".to_string(),
                })
            })
            .await?;
    } else {
        let mut bytes = String::from_utf8(cmd.output()?.stdout)?;

        context.send(|x| x.reply(true).content(bytes)).await?;
    }

    Ok(())
}

#[poise::command(prefix_command, track_edits)]
pub async fn test(context: Context<'_>, expr: Vec<String>) -> Output {
    panic!("?");
    Ok(())
}