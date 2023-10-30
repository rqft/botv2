use imagga::{colors::Color, ColorsOptions, TagsOptions};
use tabled::settings::{
    object::{Columns, Object, Rows},
    Alignment, Modify, Style,
};

use crate::{
    common::{Context, Output},
    embed_preset::user,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// get colours of an image
pub async fn colors(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(vec!["png".to_string()], &context, image_url, false).await;
    let url = urls.get(0).ok_or("no media found")?;

    // dbg!(url);

    let colors = context
        .data()
        .imagga
        .colours(ColorsOptions {
            extract_overall_colors: 1,
            extract_object_colors: 1,
            overall_count: 10,
            separated_count: 3,
            deterministic: 0,
            image_url: Some(url.clone()),
            image_upload_id: None,
        })
        .await?
        .colors;

    context
        .send(|x| {
            x.embed(|y| {
                let t = user(context, y);

                t.title("Image Colours");
                t.thumbnail(url);

                let m = |mut x: Vec<Color>, h: bool| -> String {
                    x.sort_by(|x, y| y.percent.total_cmp(&x.percent));
                    let mut y = tabled::Table::builder(x.iter().map(|c| {
                        (
                            format!("{} ({})", &c.closest_palette_color, &c.html_code),
                            if c.percent == 100.0 {
                                "100.0".to_string()
                            } else {
                                format!("{:>5.2}", c.percent)
                            },
                        )
                    }));

                    if h {
                        y.set_header(vec!["Colour", "Percentage"]);
                    } else {
                        y.remove_header();
                    }

                    y.clone()
                        .build()
                        .with(Style::sharp())
                        .with(
                            Modify::new(Rows::new(1..).not(Columns::first()))
                                .with(Alignment::center()),
                        )
                        .to_string()
                };

                if let Some(fg) = colors.image_colors.clone() {
                    t.description(format!("```hs\n{}\n```", m(fg, true)));
                }

                // if let Some(fg) = colors.background_colors.clone() {
                //     t.field(
                //         "Background Colours",
                //         format!("```hs\n{}\n```", m(fg, false)),
                //         false,
                //     );
                // }

                // if let Some(fg) = colors.foreground_colors.clone() {
                //     t.field(
                //         "Foreground Colours",
                //         format!("```hs\n{}\n```", m(fg, false)),
                //         false,
                //     );
                // }

                t.field(
                    "Colour Percent Threshold",
                    format!("{:.3}%", colors.color_percent_threshold),
                    true,
                );
                t.field("Colour Variance", colors.color_variance, true);
                t.field(
                    "Object Percentage",
                    format!("{:.3}%", colors.object_percentage),
                    true,
                );

                t
            });

            x
        })
        .await?;

    Ok(())
}
