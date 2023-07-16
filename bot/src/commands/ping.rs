use crate::common::{Context, Output};
use std::time::Instant;

#[poise::command(prefix_command, slash_command, track_edits)]
/// pong
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
