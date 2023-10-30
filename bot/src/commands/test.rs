use std::marker::PhantomData;

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
            get_page: std::convert::identity,
            on_page: |_: usize, v: usize, mut p: Inter| {
                match v {
                    1 => p.content("ok, 1"),
                    2 => p.content("not ok, 2"),
                    3 => p.embed(|b| b.title("ok, 3")),
                    _ => unreachable!(),
                };

                p
            },
            page_limit: 3,
            targets: None,
            none: PhantomData,
        },
    );

    pa.start().await;
    Ok(())
}

use std::fmt::Write;
pub const H: &str = "\x1b[";
pub fn cramers_rule([[a, b, e], [c, d, f]]: [[f64; 3]; 2]) -> String {
    let mut output = String::new();
    writeln!(output, "{H}33mSystem:{H}0m");
    writeln!(output, "{H}31m{a}{H}0mx + {H}31m{b}{H}0my = {H}31m{e}{H}0m\n{H}31m{c}{H}0mx + {H}31m{d}{H}0my = {H}31m{f}{H}0m\n");

    writeln!(output, "{H}33mMatrix:{H}0m");
    writeln!(output, "[{H}31m{a} {b}{H}0m][x] = [{H}31m{e}{H}0m]");
    writeln!(output, "[{H}31m{c} {d}{H}0m][y] = [{H}31m{f}{H}0m]\n");
    writeln!(output, "{H}33mDt{H}0m = det | {H}31m{a} {b}{H}0m");
    writeln!(output, "         | {H}31m{c} {d}{H}0m");
    writeln!(
        output,
        "{H}33mDt{H}0m = {H}31m{a}{H}0m*{H}31m{d}{H}0m - {H}31m{b}{H}0m*{H}31m{c}{H}0m"
    );
    writeln!(output, "   = {} - {}", a * d, b * c);
    writeln!(output, "   = {}\n", a.mul_add(d, -b * c));
    writeln!(output, "{H}33mDx{H}0m = det | {H}31m{e} {b}{H}0m");
    writeln!(output, "         | {H}31m{f} {d}{H}0m");
    writeln!(
        output,
        "{H}33mDx{H}0m = {H}31m{e}{H}0m*{H}31m{d}{H}0m - {H}31m{b}{H}0m*{H}31m{f}{H}0m"
    );
    writeln!(output, "   = {} - {}", e * d, b * f);
    writeln!(output, "   = {}\n", e.mul_add(d, -b * f));
    writeln!(output, "{H}33mDy{H}0m = det | {H}31m{a} {e}{H}0m");
    writeln!(output, "         | {H}31m{c} {f}{H}0m");
    writeln!(
        output,
        "{H}33mDy{H}0m = {H}31m{a}{H}0m*{H}31m{f}{H}0m - {H}31m{c}{H}0m*{H}31m{f}{H}0m"
    );
    writeln!(output, "   = {} - {}", a * f, c * e);
    writeln!(output, "   = {}\n", a.mul_add(f, -c * e));
    writeln!(output, "{H}35mx{H}0m = {H}33mD{H}35mx{H}0m/{H}33mD{H}0m = {H}35m{}{H}0m / {H}33m{}{H}0m = {H}32m{}{H}0m", e.mul_add(d, -b * f), a.mul_add(d, -b * c), e.mul_add(d, -b * f) / a.mul_add(d, -b * c));
    writeln!(output, "{H}35my{H}0m = {H}33mD{H}35my{H}0m/{H}33mD{H}0m = {H}35m{}{H}0m / {H}33m{}{H}0m = {H}32m{}{H}0m", a.mul_add(f, -c * e), a.mul_add(d, -b * c), a.mul_add(f, -c * e) / a.mul_add(d, -b * c));

    output
}

#[poise::command(prefix_command, track_edits)]
pub async fn crame(context: Context<'_>, a: f64, b: f64, e: f64, c: f64, d: f64, f: f64) -> Output {
    let o = cramers_rule([[a, b, e], [c, d, f]]);
    context.say(format!("```ansi\n{o}\n```")).await?;
    Ok(())
}
