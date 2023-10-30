use clap::{arg, command, Parser};

use image::{Rgba, RgbaImage};
use poise::serenity_prelude::AttachmentType;
use std::{borrow::Cow, io::Cursor, time::Instant};

use crate::common::{clap_parse_into, get_output, scale, Context, Output};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
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
    #[arg(long = "tm", value_name = "time_min")]
    time_min: Option<u32>,
    #[arg(long = "tx", value_name = "time_max")]
    time_max: Option<u32>,
    #[arg(long = "size", value_name = "size")]
    size: Option<u32>,
}

pub const COLOURS: [[u8; 4]; 7] = [
    [0x55, 0x55, 0x55, 0xff],
    [0xff, 0x55, 0x55, 0xff],
    [0x55, 0xff, 0x55, 0xff],
    [0x55, 0x55, 0xff, 0xff],
    [0xff, 0xff, 0x55, 0xff],
    [0xff, 0x55, 0xff, 0xff],
    [0x55, 0xff, 0xff, 0xff],
];

pub async fn graph(
    exprs: String,
    input: Args,
) -> std::result::Result<(u16, Vec<RgbaImage>), String> {
    let mut frames = vec![];
    // dbg!(&input);

    let s = input.size.unwrap_or(512);

    let splot = input.splot.unwrap_or(1);

    if splot < 1 {
        return Err("invalid splot area".to_string());
    }

    let scalar = input.scale.unwrap_or(50.0);

    let tm = input.time_min.unwrap_or(1).min(1);
    let tx = input.time_max.unwrap_or(10).max(10);

    for t in tm..=tx {
        let mut l = RgbaImage::from_pixel(s, s, image::Rgba([0xff, 0xff, 0xff, 0xff]));

        let (wid, hei) = (l.width() as i32, l.height() as i32);
        let (w, h) = (wid / 2, hei / 2);

        for i in 1..wid {
            l.put_pixel(w as u32, i as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
            l.put_pixel(i as u32, w as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
        }

        for x in (-w)..w {
            let z = exprs
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

                let y: f64 =
                    get_output(dy.unwrap(), &[x as f64 / scalar, t as f64], &["x", "t"])? * scalar;
                // dbg!(x, y);

                if y > (h as f64) || y < (-h as f64) || y.is_infinite() || y.is_nan() {
                    continue;
                }

                let dm = input
                    .domain_min
                    .clone()
                    .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

                let dx = input
                    .domain_max
                    .clone()
                    .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

                let rm = input
                    .range_min
                    .clone()
                    .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

                let rx = input
                    .range_max
                    .clone()
                    .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

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
                        let z =
                            scale(x as f64, [-w as f64, w as f64], [1.0, wid as f64]) as i32 + i;

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

        frames.push(l);
    }

    Ok((s as u16, frames))
}

#[poise::command(prefix_command, track_edits)]
/// bootleg desmos, but gif, see help for usage
pub async fn gplot(context: Context<'_>, args: Vec<String>) -> Output {
    let exprs_index = args
        .iter()
        .enumerate()
        .find(|(_, y)| y.starts_with("--"))
        .map(|x| x.0)
        .unwrap_or(args.len());
    let exprs = args[0..exprs_index].join(" ");
    let rargs = &args[exprs_index..];

    // dbg!(&exprs);
    let args = clap_parse_into::<Args>(rargs)?;

    let r = context.say("plotting..").await?;

    let (s, frames) = graph(exprs, args).await?;

    let writer = Cursor::new(Vec::new());
    let mut v = gif::Encoder::new(writer, s, s, &[]).map_err(|x| x.to_string())?;
    let mut i = 0;
    for x in &frames {
        r.edit(context, |x| x.content(format!("rendering.. (frame {i})")))
            .await
            .unwrap();
        println!("rendering {}", i);
        i += 1;
        v.write_frame(&gif::Frame::from_rgba_speed(
            x.width() as u16,
            x.height() as u16,
            &mut x.to_vec(),
            30,
        ))
        .map_err(|x| x.to_string())?;
    }

    v.set_repeat(gif::Repeat::Finite(0))
        .map_err(|x| x.to_string())?;

    context
        .send(|x| {
            x.reply(true).attachment(AttachmentType::Bytes {
                data: Cow::Owned(v.into_inner().unwrap().into_inner()),
                filename: "plot.gif".to_string(),
            })
        })
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command, rename = "gplot")]
/// bootleg desmos, but gif
pub async fn gplot_slash(
    context: Context<'_>,
    #[description = "expressions to plot"] exprs: String,
    #[description = "how big each point is"] splot: Option<i32>,
    #[description = "how large to scale the output to"] scale: Option<f64>,
    #[description = "x position minimum"] domain_min: Option<String>,
    #[description = "x position maximum"] domain_max: Option<String>,
    #[description = "y position minimum"] range_min: Option<String>,
    #[description = "y position maximum"] range_max: Option<String>,
    #[description = "time minimum"] time_min: Option<u32>,
    #[description = "time maximum"] time_max: Option<u32>,
    #[description = "image size"] size: Option<u32>,
) -> Output {
    let args = Args {
        domain_max,
        domain_min,
        range_max,
        range_min,
        scale,
        size,
        splot,
        time_max,
        time_min,
    };

    let r = context.say("plotting..").await?;

    let plot_time = Instant::now();
    let (s, frames) = graph(exprs, args).await?;
    let plot_time_elapsed = plot_time.elapsed();

    let render_time = Instant::now();
    let writer = Cursor::new(Vec::new());
    let mut v = gif::Encoder::new(writer, s, s, &[]).map_err(|x| x.to_string())?;
    for (i, x) in frames.iter().enumerate() {
        r.edit(context, |x| x.content(format!("rendering.. (frame {i})")))
            .await
            .unwrap();
        v.write_frame(&gif::Frame::from_rgba_speed(
            x.width() as u16,
            x.height() as u16,
            &mut x.to_vec(),
            30,
        ))
        .map_err(|x| x.to_string())?;
    }
    let render_time_elapsed = render_time.elapsed();

    v.set_repeat(gif::Repeat::Finite(0))
        .map_err(|x| x.to_string())?;

    context
        .send(|x| {
            x.reply(true)
                .attachment(AttachmentType::Bytes {
                    data: Cow::Owned(v.into_inner().unwrap().into_inner()),
                    filename: "plot.gif".to_string(),
                })
                .content(format!(
                    "graphed in {}ms, rendered in {}ms",
                    plot_time_elapsed.as_millis(),
                    render_time_elapsed.as_millis()
                ))
        })
        .await?;

    Ok(())
}
