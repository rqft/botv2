use crate::{
    common::{Context, Output},
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn image_url(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(
        vec!["png".to_string()],
        &context,
        image_url,
        false,
    )
    .await;

    context
        .say(urls.get(0).unwrap_or(&"no media found".to_string()))
        .await?;
    Ok(())
}
