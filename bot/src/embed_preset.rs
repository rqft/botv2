use std::{
    collections::VecDeque,
    io::{BufWriter, Cursor},
    time::{Duration, Instant},
};

use image::DynamicImage;
use poise::{serenity_prelude::AttachmentType, CreateReply};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::common::{human_bytes, Context};

pub fn user<'a>(context: Context<'_>, embed: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
    embed.color(0x2c2d30).author(|f| {
        f.name(context.author().name.clone()).icon_url(
            context
                .author()
                .avatar_url()
                .unwrap_or(context.author().default_avatar_url()),
        )
    })
}

pub fn image<'a, 'b>(
    context: Context<'_>,
    reply: &'a mut CreateReply<'b>,
    img: DynamicImage,
) -> &'a mut CreateReply<'b> {
    let mut v: Vec<u8> = Vec::new();
    let mut w = Cursor::new(v);
    img.write_to(&mut w, image::ImageFormat::Png);

    reply.attachment(AttachmentType::Bytes {
        data: w.into_inner().into(),
        filename: "image.png".to_string(),
    });

    let time_since: Duration = context
        .created_at()
        .signed_duration_since(chrono::Utc::now())
        .abs()
        .to_std()
        .unwrap();

    reply.embed(|x| {
        user(context, x)
            .set_footer(
                CreateEmbedFooter::default()
                    .text(format!(
                        "{}, {}x{}, took {:.2} seconds",
                        human_bytes(img.as_bytes().len() as f64, false),
                        img.width(),
                        img.height(),
                        time_since.as_secs_f64()
                    ))
                    .clone(),
            )
            .attachment("image.png")
    });

    reply
}
