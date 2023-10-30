use rand::Rng;

use crate::{
    common::{Context, Output},
    emojis,
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// roll some dice
pub async fn roll(context: Context<'_>, amount: Option<u8>, sides: Option<u16>) -> Output {
    let a = amount.unwrap_or(1);
    let s = sides.unwrap_or(6);

    context
        .say(format!(
            "Rolling {a} {s}-sided di{}e...\n{}",
            if a == 1 { "" } else { "c" },
            (1..=a)
                .map(|x| format!(
                    ":game_die: `#{x:>2}: {:>5}`",
                    rand::thread_rng().gen_range(1..=s)
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// flip some coins
pub async fn flip(context: Context<'_>, amount: Option<u8>) -> Output {
    let a = amount.unwrap_or(1);

    context
        .say(format!(
            "Flipping {a} coin{}...\n{}",
            if a == 1 { "" } else { "s" },
            (1..=a)
                .map(|x| if rand::thread_rng().gen::<bool>() {
                    format!("{} `#{x:>3}: Heads`", emojis::Heads)
                } else {
                    format!("{} `#{x:>3}: Tails`", emojis::Tails)
                })
                .collect::<Vec<_>>()
                .join("\n")
        ))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, track_edits)]
/// pick one thing out of a group of things, at random
pub async fn pick(context: Context<'_>, amount: Vec<String>) -> Output {
    let x = &amount[rand::thread_rng().gen_range(0..amount.len())];
    context.say(x.to_string()).await?;
    Ok(())
}
