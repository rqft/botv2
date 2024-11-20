use imagga::CategoriesOptions;
use tabled::settings::{
    object::{Columns, Object, Rows},
    Alignment, Modify, Style,
};

use crate::{
    common::{Context, Output},
    embed_preset::user,
    ext::RegexExt,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// get the categories that the bot thinks an image belongs to
pub async fn categories(context: Context<'_>, image_url: Option<String>) -> Output {
    let urls = find_media_urls(&["png"], &context, image_url, None).await;
    let url = urls.get(0).ok_or("no media found")?;

    // dbg!(url);

    let categories = context
        .data()
        .imagga
        .categories(
            "general_v3".into(),
            CategoriesOptions {
                image_url: Some(url.clone()),
                ..Default::default()
            },
        )
        .await?
        .categories;

    let mut x = categories.iter().collect::<Vec<_>>();

    x.sort_by(|x, y| y.1.total_cmp(x.1));
    let str = tabled::Table::builder(x.iter().take(15).map(|(c, x)| {
        (
            c.str_replace(".n.\\d+$", ""),
            if x == &&100.0 {
                "100.0".to_string()
            } else {
                format!("{x:>5.2}")
            },
        )
    }))
    .set_header(vec!["Category", "Confidence"])
    .clone()
    .build()
    .with(Style::sharp())
    .with(Modify::new(Rows::new(1..).not(Columns::first())).with(Alignment::center()))
    .to_string();

    context
        .send(|x| {
            x.content(format!(
                "Image Categories ([image](<{url}>))\n```hs\n{str}\n```"
            ))
        })
        .await?;

    Ok(())
}
