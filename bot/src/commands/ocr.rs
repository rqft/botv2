use imagga::text::TextOptions;

use crate::{
    common::{Context, Output},
    embed_preset::user,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn ocr(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(vec!["png".to_string()], &context, image_url, false).await;
    let url = urls.get(0).ok_or("no media found")?;

    dbg!(url);

    let text = context
        .data()
        .imagga
        .text(TextOptions {
            image_url: url.clone(),
        })
        .await?;

    context
        .send(|x| {
            x.embed(|x| {
                let y = user(context, x).title("Image Text").thumbnail(url);

                for text in text.text {
                    y.field(
                        format!(
                            "{}, {} ({}x{})",
                            text.coordinates.xmin,
                            text.coordinates.ymin,
                            text.coordinates.width,
                            text.coordinates.height
                        ),
                        text.data,
                        true,
                    );
                }

                y
            })
        })
        .await?;

    Ok(())
}
