use crate::{
    common::{Context, Output},
    paginator::paginate_wa,
};

#[poise::command(prefix_command, slash_command, track_edits)]
/// wolfram alpha
pub async fn wa(context: Context<'_>, input: Vec<String>) -> Output {
    println!("hi");
    let value = context
        .data()
        .wolfram
        .query(wa::model::QueryOptions {
            input: input.join(" "),
            format: Some("plaintext,image".to_string()),
        })
        .await?;
    let p = value.queryresult.pods;
    paginate_wa(context, p).await?;
    Ok(())
}
