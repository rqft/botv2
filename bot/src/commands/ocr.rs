use imagga::text::TextOptions;

use crate::{
    common::{Context, Output},
    embed_preset::user,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// read text in an image
pub async fn ocr(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(vec!["png".to_string()], &context, image_url, false).await;
    let url = urls.get(0).ok_or("no media found")?;

    // dbg!(url);

    let text = context
        .data()
        .imagga
        .text(TextOptions {
            image_url: url.clone(),
        })
        .await?;

    context
        .send(|x| {
            x.content(format!(
                "Image Text ([image](<{}>))\n{}",
                url,
                text.text
                    .into_iter()
                    .map(|x| format!(
                        "{}, {} ({}x{})\n{}",
                        x.coordinates.xmin,
                        x.coordinates.ymin,
                        x.coordinates.width,
                        x.coordinates.height,
                        x.data
                            .lines()
                            .map(|x| "> ".to_string() + x)
                            .collect::<Vec<_>>()
                            .join("\n")
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        })
        .await?;

    Ok(())
}
