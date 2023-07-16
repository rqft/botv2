use imagga::TagsOptions;
use tabled::settings::Style;

use crate::{
    common::{Context, Output},
    embed_preset::user,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn image_tags(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(vec!["png".to_string()], &context, image_url, false).await;
    let url = urls.get(0).ok_or("no media found")?;

    dbg!(url);

    let tags = context
        .data()
        .imagga
        .tags(
            TagsOptions {
                image_url: Some(url.clone()),
                ..Default::default()
            },
            None,
        )
        .await?
        .tags;

    let mut x = tags.iter().collect::<Vec<_>>();

    x.sort_by(|x, y| y.1.total_cmp(x.1));
    let str = tabled::Table::builder(x[0..15].iter().map(|(c, x)| (c, format!("{x:>5.2}"))))
        .set_header(vec!["Tag", "Confidence"])
        .clone()
        .build()
        .with(Style::sharp())
        .to_string();

    context
        .send(|x| {
            x.embed(|x| {
                user(context, x)
                    .title("Image Tags")
                    .description(format!("```hs\n{}\n```", str))
                    .thumbnail(url)
            })
        })
        .await?;

    Ok(())
}
