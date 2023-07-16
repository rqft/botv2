use crate::{
    common::{Context, Output},
    paginator::{Buttons, Inter, Paginator, PaginatorOptions},
};

#[poise::command(prefix_command, track_edits)]
/// pong
pub async fn test(context: Context<'_>) -> Output {
    let pa = Paginator::new(
        context,
        PaginatorOptions {
            buttons: Some(Buttons::default()),
            expires: 1000 * 60 * 5,
            is_ephemeral: Some(false),
            on_page: &|v: usize, mut p: Inter| {
                match v {
                    1 => p.content("ok, 1"),
                    2 => p.content("not ok, 2"),
                    3 => p.embed(|b| b.title("ok, 3")),
                    _ => unreachable!(),
                };

                p
            },
            on_stop: Some(&|| {}),
            page_limit: 3,
            targets: None,
        },
    );

    pa.start().await;
    Ok(())
}
