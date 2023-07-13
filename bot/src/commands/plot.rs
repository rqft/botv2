use clap::{arg, command, Parser};
use image::{ImageFormat, Rgba, RgbaImage};
use poise::serenity_prelude::AttachmentType;
use std::{borrow::Cow, io::Cursor};

use crate::common::{clap_parse_into, scale, Context, Output};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    exprs: String,
    #[arg(short = 's', long = "splot")]
    splot: Option<i32>,
    #[arg(long = "scale")]
    scale: Option<f64>,
    #[arg(long = "dm", value_name = "domain_min")]
    domain_min: Option<String>,
    #[arg(long = "dx", value_name = "domain_max")]
    domain_max: Option<String>,
    #[arg(long = "rm", value_name = "range_min")]
    range_min: Option<String>,
    #[arg(long = "rx", value_name = "range_max")]
    range_max: Option<String>,
    #[arg(long = "size", value_name = "size")]
    size: Option<u32>,
}

pub const COLOURS: [[u8; 4]; 7] = [
    [0x88, 0x88, 0x88, 0x88],
    [0xff, 0x55, 0x55, 0xff],
    [0x55, 0xff, 0x55, 0xff],
    [0x55, 0x55, 0xff, 0xff],
    [0xff, 0xff, 0x55, 0xff],
    [0xff, 0x55, 0xff, 0xff],
    [0x55, 0xff, 0xff, 0xff],
];

fn get_output(expr: &str, x: &[f64]) -> std::result::Result<f64, String> {
    use exmex::prelude::*;
    let e = FlatEx::<f64>::parse(expr).unwrap();
    e.eval_relaxed(x).map_err(|x| x.msg().to_string())
}

pub async fn graph(input: Args) -> std::result::Result<RgbaImage, String> {
    dbg!(&input);

    let s = input.size.unwrap_or(1024);

    let splot = input.splot.unwrap_or(1);

    if splot < 1 {
        return Err("invalid splot area".to_string());
    }

    let scalar = input.scale.unwrap_or(1.0);

    let mut l = RgbaImage::from_pixel(s, s, image::Rgba([0xff, 0xff, 0xff, 0xff]));

    let (wid, hei) = (l.width() as i32, l.height() as i32);
    let (w, h) = (wid / 2, hei / 2);

    for i in 1..wid {
        l.put_pixel(w as u32, i as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
        l.put_pixel(i as u32, w as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
    }

    for x in (-w)..w {
        let z = input
            .exprs
            .split(';')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if z.len() > COLOURS.len() {
            return Err(format!("too many expressions (max {})", COLOURS.len()));
        }

        for dx in 0..z.len() {
            let dy = z.get(dx);
            let c = COLOURS.get(dx);

            if dy.is_none() || c.is_none() {
                continue;
            }

            let y: f64 = get_output(dy.unwrap(), &[x as f64 / scalar])? * scalar;
            // dbg!(x, y);

            if y > (h as f64) || y < (-h as f64) || y.is_infinite() || y.is_nan() {
                continue;
            }

            let dm = input
                .domain_min
                .clone()
                .and_then(|v| get_output(&v, &[x as f64, y]).ok());

            let dx = input
                .domain_max
                .clone()
                .and_then(|v| get_output(&v, &[x as f64, y]).ok());

            let rm = input
                .range_min
                .clone()
                .and_then(|v| get_output(&v, &[x as f64, y]).ok());

            let rx = input
                .range_max
                .clone()
                .and_then(|v| get_output(&v, &[x as f64, y]).ok());

            if (dm.is_some() && dm > Some(x as f64))
                || (dx.is_some() && dx < Some(x as f64))
                || (rm.is_some() && rm > Some(y))
                || (rx.is_some() && rx < Some(y))
            {
                // println!("failed d/r, {x}, {y} not in [{dm:?}..{dx:?}] [{rm:?}..{rx:?}]");
                continue;
            }

            // dbg!(c);
            let mut i = -splot;
            while i <= splot {
                // dbg!(i);
                let mut j = -splot;
                'b: while j <= splot {
                    // dbg!(j);
                    let z = scale(x as f64, [-w as f64, w as f64], [1.0, wid as f64]) as i32 + i;

                    // dbg!(z);
                    let d = wid - scale(y, [-h as f64, h as f64], [1.0, hei as f64]) as i32 + j;
                    // dbg!(d);

                    // println!("check");
                    if z >= wid || z < 1 || d >= hei || d < 1 {
                        j += 1;
                        // println!("{z} {d} skipped anyways");
                        continue 'b;
                    }

                    // println!("pixel");
                    l.put_pixel(z as u32, d as u32, Rgba(*c.unwrap()));
                    // println!("incr j");
                    j += 1;
                }

                // println!("incr i");
                i += 1;
            }
        }
    }

    Ok(l)
}

#[poise::command(prefix_command, track_edits)]
pub async fn plot(context: Context<'_>, args: Vec<String>) -> Output {
    let args = clap_parse_into::<Args>(&args)?;

    let image = graph(args).await?;

    let mut writer = Cursor::new(Vec::new());
    image.write_to(&mut writer, ImageFormat::Png)?;

    context
        .send(|x| {
            x.reply(true).attachment(AttachmentType::Bytes {
                data: Cow::Owned(writer.into_inner()),
                filename: "plot.png".to_string(),
            })
        })
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command, rename = "plot")]
pub async fn plot_slash(
    context: Context<'_>,
    #[description = "expressions to plot"] exprs: String,
    #[description = "how big each point is"] splot: Option<i32>,
    #[description = "how large to scale the output to"] scale: Option<f64>,
    #[description = "x position minimum"] domain_min: Option<String>,
    #[description = "x position maximum"] domain_max: Option<String>,
    #[description = "y position minimum"] range_min: Option<String>,
    #[description = "y position maximum"] range_max: Option<String>,
    #[description = "image size"] size: Option<u32>,
) -> Output {
    let args = Args {
        domain_max,
        domain_min,
        range_max,
        range_min,
        exprs,
        scale,
        size,
        splot,
    };

    let image = graph(args).await?;

    let mut writer = Cursor::new(Vec::new());
    image.write_to(&mut writer, ImageFormat::Png)?;

    context
        .send(|x| {
            x.reply(true).attachment(AttachmentType::Bytes {
                data: Cow::Owned(writer.into_inner()),
                filename: "plot.png".to_string(),
            })
        })
        .await?;

    Ok(())
}
