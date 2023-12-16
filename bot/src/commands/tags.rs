use imagga::TagsOptions;
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
/// get what the bot thinks an image is
pub async fn tags(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(vec!["png".to_string()], &context, image_url, false).await;
    let url = urls.get(0).ok_or("no media found")?;

    // dbg!(url);

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
    let str = tabled::Table::builder(x.iter().take(15).map(|(c, x)| {
        (
            c,
            if x == &&100.0 {
                "100.0".to_string()
            } else {
                format!("{x:>5.2}")
            },
        )
    }))
    .set_header(vec!["Tag", "Confidence"])
    .clone()
    .build()
    .with(Style::sharp())
    .with(Modify::new(Rows::new(1..).not(Columns::first())).with(Alignment::center()))
    .to_string();

    context
        .send(|x| x.content(format!("Image Tags ([image](<{url}>))\n```hs\n{str}\n```")))
        .await?;

    Ok(())
}
