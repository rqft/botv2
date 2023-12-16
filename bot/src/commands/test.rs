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

fn plot() -> Output<Cursor<Vec<u8>>> {
    let mut buf = image::ImageBuffer::<image::Rgb<u8>, _>::new(512, 512);
    let root = BitMapBackend::with_buffer(&mut buf, (512, 512)).into_drawing_area();
    root.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root)
        .caption("f(x, z) = x^2 + z^2", ("serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_3d((-3.0..3.0).step(0.1), -3.0..3.0, (-3.0..3.0).step(0.1))?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw();

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-30..30).map(|f| f as f64 / 10.0),
                (-30..30).map(|f| f as f64 / 10.0),
                |x, z| (x * x + z * z).cos(),
            )
            .style(BLUE.mix(0.2).filled()),
        )?
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    root.present()?;

    drop(chart);
    drop(root);

    let mut writer = Cursor::new(Vec::new());

    buf.write_to(&mut writer, ImageFormat::Png)?;

    Ok(writer)
}
