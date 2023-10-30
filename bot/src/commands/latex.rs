use imagga::text::TextOptions;
use url::UrlQuery;

use crate::{
    common::{Context, Output},
    embed_preset::user,
    ext::RegexExt,
    get_image::find_media_urls,
};

#[poise::command(prefix_command, slash_command, track_edits, aliases("tex"))]
/// render LaTeX
pub async fn latex(context: Context<'_>, tex: Vec<String>) -> Output {
    dbg!(1);
    let l = tex.join(" ");

    /*
        formula=\begin{align*}
    x^2 + y^2 %26= 1 \\
    y %26= \sqrt{1 - x^2}
    \end{align*}&fsize=99px&fcolor=FFFFFF&mode=0&out=1&remhost=quicklatex.com&preamble=\usepackage{amsmath}
    \usepackage{amsfonts}
    \usepackage{amssymb}&rnd=42.39749766193235 */

    // dbg!(&l);

    let v = format!("formula={}&fsize=99px&fcolor=FFFFFF&mode=0&out=1&remhost=quicklatex.com&preamble\\usepackage{{amsmath}}\n\\usepackage{{amsfonts}}\n\\usepackage{{amssymb}}", l.bytes().map(|x| {
        match x {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')' | b' ' => (x as char).to_string(),
            c => format!("%{:0>2x}", c)
        }
    }).collect::<Vec<_>>().join(""));

    let q = context
        .data()
        .req
        .post("https://www.quicklatex.com/latex3.f")
        .body(v)
        .send()
        .await?
        .text()
        .await?
        .str_replace("\\d+\\r?\\n(.+?)( \\d+)+", "$1");

    // dbg!(&q);

    context.say(q).await?;

    Ok(())
}

fn drop<T>(i: T) {
    i;
}
