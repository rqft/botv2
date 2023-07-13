use clap::{CommandFactory, FromArgMatches};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = ::poise::Context<'a, Data, Error>;
pub type Output = Result<(), Error>;

pub fn scale<T>(v: T, [xn, xm]: [T; 2], [yn, ym]: [T; 2]) -> T
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + std::ops::Div<T, Output = T>
        + Copy,
{
    ((v - xn) / (xm - xn)) * (ym - yn) + yn
}

pub fn to_code_point(surrogates: &str, separator: &str) -> String {
    let mut r = Vec::new();
    let mut c = 0u32;
    let mut p = 0u32;

    // while i < surrogates.len() {
    //     c = surrogates.chars().collect::<Vec<char>>()[i] as u8 as u32;
    //     i += 1;

    //     if p != 0 {
    //         r.push(format!(
    //             "{:x}",
    //             (0x10000 + ((p - 0xd800) << 10) + (c - 0xdc00))
    //         ));
    //         p = 0;
    //     } else if (0xd800..=0xdbff).contains(&c) {
    //         p = c;
    //     } else {
    //         r.push(format!("{c:x}"))
    //     }
    // }

    surrogates.encode_utf16().for_each(|x| {
        c = x as u32;

        if p != 0 {
            r.push(format!(
                "{:x}",
                (0x10000 + ((p - 0xd800) << 10) + (c - 0xdc00))
            ));
            p = 0;
        } else if (0xd800..=0xdbff).contains(&c) {
            p = c;
        } else {
            r.push(format!("{c:x}"))
        }
    });

    r.join(separator)
}

pub fn to_code_point_for_twemoji(surrogates: &str) -> String {
    if !surrogates.contains('\u{200d}') {
        to_code_point(&surrogates.replace('\u{fe0f}', ""), "-")
    } else {
        to_code_point(surrogates, "-")
    }
}

pub fn clap_parse_into<T>(args: &[String]) -> Result<T, clap::error::Error>
where
    T: clap::CommandFactory + clap::FromArgMatches,
{
    let mut matches = <T as CommandFactory>::command().no_binary_name(true).get_matches_from(args);
    <T as FromArgMatches>::from_arg_matches_mut(&mut matches)
}
