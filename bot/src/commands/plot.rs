use clap::{arg, command, Parser};
use image::{ImageBuffer, ImageFormat, Rgb, Rgba, RgbaImage};
use plotters::{
    backend::BitMapBackend,
    style::{RGBAColor, RGBColor},
};
use poise::serenity_prelude::AttachmentType;
use std::{borrow::Cow, io::Cursor};

use crate::common::{clap_parse_into, get_output, scale, Context, Output};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    #[arg(short = 's', long = "splot")]
    splot: Option<u32>,
    #[arg(long = "scale")]
    scale: Option<f64>,
    #[arg(long = "dm", value_name = "domain_min")]
    domain_min: Option<f64>,
    #[arg(long = "dx", value_name = "domain_max")]
    domain_max: Option<f64>,
    #[arg(long = "rm", value_name = "range_min")]
    range_min: Option<f64>,
    #[arg(long = "rx", value_name = "range_max")]
    range_max: Option<f64>,
    #[arg(long = "size", value_name = "size")]
    size: Option<u32>,
}

pub const COLOURS: [RGBColor; 7] = [
    RGBColor(0x55, 0x55, 0x55),
    RGBColor(0xff, 0x55, 0x55),
    RGBColor(0x55, 0xff, 0x55),
    RGBColor(0x55, 0x55, 0xff),
    RGBColor(0xff, 0xff, 0x55),
    RGBColor(0xff, 0x55, 0xff),
    RGBColor(0x55, 0xff, 0xff),
];

// pub fn graph(exprs: String, input: Args) -> std::result::Result<RgbaImage, String> {
// // dbg!(&input);

// let s = input.size.unwrap_or(1024);

// let splot = input.splot.unwrap_or(1);

// if splot < 1 {
//     return Err("invalid splot area".to_string());
// }

// let scalar = input.scale.unwrap_or(50.0);

// let mut l = RgbaImage::from_pixel(s, s, image::Rgba([0xff, 0xff, 0xff, 0xff]));

// let (wid, hei) = (l.width() as i32, l.height() as i32);
// let (w, h) = (wid / 2, hei / 2);

// for i in 1..wid {
//     l.put_pixel(w as u32, i as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
//     l.put_pixel(i as u32, w as u32, image::Rgba([0x88, 0x88, 0x88, 0xff]));
// }

// for x in (-w)..w {
//     let z = exprs
//         .split(';')
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>();

//     if z.len() > COLOURS.len() {
//         return Err(format!("too many expressions (max {})", COLOURS.len()));
//     }

//     for dx in 0..z.len() {
//         let dy = z.get(dx);
//         let c = COLOURS.get(dx);

//         if dy.is_none() || c.is_none() {
//             continue;
//         }

//         let y: f64 = get_output(dy.unwrap(), &[x as f64 / scalar], &["x"])? * scalar;
//         // dbg!(x, y);

//         if y > (h as f64) || y < (-h as f64) || y.is_infinite() || y.is_nan() {
//             continue;
//         }

//         let dm = input
//             .domain_min
//             .clone()
//             .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

//         let dx = input
//             .domain_max
//             .clone()
//             .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

//         let rm = input
//             .range_min
//             .clone()
//             .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

//         let rx = input
//             .range_max
//             .clone()
//             .and_then(|v| get_output(&v, &[x as f64, y], &["x", "y"]).ok());

//         if (dm.is_some() && dm > Some(x as f64))
//             || (dx.is_some() && dx < Some(x as f64))
//             || (rm.is_some() && rm > Some(y))
//             || (rx.is_some() && rx < Some(y))
//         {
//             // println!("failed d/r, {x}, {y} not in [{dm:?}..{dx:?}] [{rm:?}..{rx:?}]");
//             continue;
//         }

//         // dbg!(c);
//         let mut i = -splot;
//         while i <= splot {
//             // dbg!(i);
//             let mut j = -splot;
//             'b: while j <= splot {
//                 // dbg!(j);
//                 let z = scale(x as f64, [-w as f64, w as f64], [1.0, wid as f64]) as i32 + i;

//                 // dbg!(z);
//                 let d = wid - scale(y, [-h as f64, h as f64], [1.0, hei as f64]) as i32 + j;
//                 // dbg!(d);

//                 // println!("check");
//                 if z >= wid || z < 1 || d >= hei || d < 1 {
//                     j += 1;
//                     // println!("{z} {d} skipped anyways");
//                     continue 'b;
//                 }

//                 // println!("pixel");
//                 l.put_pixel(z as u32, d as u32, Rgba(*c.unwrap()));
//                 // println!("incr j");
//                 j += 1;
//             }

//             // println!("incr i");
//             i += 1;
//         }
//     }
// }

// Ok(l)
// }

// #[poise::command(prefix_command, track_edits)]
/// bootleg desmos, see help for usage
// pub async fn plot(context: Context<'_>, args: Vec<String>) -> Output {
// let exprs_index = args
//     .iter()
//     .enumerate()
//     .find(|(_, y)| y.starts_with("--"))
//     .map(|x| x.0)
//     .unwrap_or(args.len());
// let exprs = args[0..exprs_index].join(" ");
// let rargs = &args[exprs_index..];

// // dbg!(&exprs);
// let args = clap_parse_into::<Args>(rargs)?;

// let image = graph(exprs, args)?;

// let mut writer = Cursor::new(Vec::new());
// image.write_to(&mut writer, ImageFormat::Png)?;

// context
//     .send(|x| {
//         x.reply(true).attachment(AttachmentType::Bytes {
//             data: Cow::Owned(writer.into_inner()),
//             filename: "plot.png".to_string(),
//         })
//     })
//     .await?;

// Ok(())
// }

#[poise::command(prefix_command, track_edits)]
/// bootleg desmos, see help for usage
pub async fn plot(context: Context<'_>, args: Vec<String>) -> Output {
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

    let image = graph2(exprs, args)?;

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

fn graph2(exprs: String, input: Args) -> Output<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    use plotters::prelude::*;
    let size = input.size.unwrap_or(1024);
    let scalar = input.scale.unwrap_or(50.0);
    let splot = input.splot.unwrap_or(2);
    let dm = input.domain_min.unwrap_or(-10.0);
    let dr = input.domain_max.unwrap_or(10.0);
    let rm = input.range_min.unwrap_or(-10.0);
    let rr = input.range_max.unwrap_or(10.0);
    let margin = 30;
    let padding = 1.0;
    let mut buf = image::ImageBuffer::<image::Rgb<u8>, _>::new(size, size);
    let root = BitMapBackend::with_buffer(&mut buf, (size, size)).into_drawing_area();

    root.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root)
        .margin(margin)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(dm..dr, rm..rr)?;

    chart.configure_mesh().max_light_lines(3).draw()?;

    let z = exprs
        .split(';')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    if z.len() > COLOURS.len() {
        todo!();
    }

    for dx in 0..z.len() {
        let dy = z.get(dx);
        let c = COLOURS.get(dx);

        if dy.is_none() || c.is_none() {
            continue;
        }

        let cc = *c.unwrap();

        chart
            .draw_series(LineSeries::new(
                (((dm * scalar) as i32)..((dr * scalar) as i32))
                    .map(f64::from)
                    .map(|x| x / scalar)
                    .map(|x| (x, get_output(dy.unwrap(), &[x], &["x"]).unwrap()))
                    .filter(|(x, y)| y >= &(rm - padding) && y <= &(rr + padding) && y.is_finite()),
                ShapeStyle {
                    color: cc.into(),
                    filled: true,
                    stroke_width: splot,
                },
            ))?
            .label(dy.unwrap())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], cc.clone()));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    drop(chart);
    drop(root);

    Ok(buf)
}

#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command, rename = "plot")]
/// bootleg desmos
pub async fn plot_slash(
    context: Context<'_>,
    #[description = "expressions to plot"] exprs: String,
    #[description = "how big each point is"] splot: Option<u32>,
    #[description = "how large to scale the output to"] scale: Option<f64>,
    #[description = "x position minimum"] domain_min: Option<f64>,
    #[description = "x position maximum"] domain_max: Option<f64>,
    #[description = "y position minimum"] range_min: Option<f64>,
    #[description = "y position maximum"] range_max: Option<f64>,
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
    };

    let image = graph2(exprs, args)?;

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

#[poise::command(prefix_command, slash_command)]
/// run math expression
pub async fn math(
    context: Context<'_>,
    #[description = "the expression"] expr: Vec<String>,
) -> Output {
    let value = get_output(&expr.join(" "), &[], &[])?;
    context.say(value.to_string()).await?;
    Ok(())
}
