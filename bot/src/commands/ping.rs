use std::time::Instant;
use crate::common::{Context, Output};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn ping(context: Context<'_>) -> Output {
    let start = Instant::now();

    let response = context.say("pong").await?;

    response
        .edit(context, |x| {
            x.content(format!("pong in {}ms", start.elapsed().as_millis()))
        })
        .await?;
    Ok(())
}
