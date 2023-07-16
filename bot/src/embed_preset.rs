use serenity::builder::CreateEmbed;

use crate::common::Context;

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
