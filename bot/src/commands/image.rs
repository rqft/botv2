use crate::{
    common::{Context, Output}, embed_preset, get_image::{find_media_urls, get_media_data}
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// inverts an image
pub async fn invert(context: Context<'_>, image_url: Option<String>) -> Output {
    let byt = get_media_data(&["png"], &context, image_url, None).await?;
    let mut im = image::load_from_memory(&byt)?;

    im.invert();

    context.send(|x| embed_preset::image(context, x, im)).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// flip
pub async fn flip(context: Context<'_>, image_url: Option<String>) -> Output {
    let byt = get_media_data(&["png"], &context, image_url, None).await.unwrap();
    let mut im = image::load_from_memory(&byt)?;

    im.fliph();

    context.send(|x| embed_preset::image(context, x, im)).await?;

    Ok(())
}
#[poise::command(prefix_command, slash_command, track_edits)]
/// flop
pub async fn flop(context: Context<'_>, image_url: Option<String>) -> Output {
    let byt = get_media_data(&["png"], &context, image_url, None).await.unwrap();
    let mut im = image::load_from_memory(&byt)?;

    im.flipv();

    context.send(|x| embed_preset::image(context, x, im)).await?;

    Ok(())
}