use clap::{CommandFactory, FromArgMatches};
use imagga::Imagga;
use wa::Wolfram;

pub struct Data {
    pub imagga: Imagga,
    pub wolfram: Wolfram,
}
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
    let mut matches = <T as CommandFactory>::command()
        .no_binary_name(true)
        .try_get_matches_from(args)?;
    <T as FromArgMatches>::from_arg_matches_mut(&mut matches)
}

// cc: https://docs.rs/statrs/latest/src/statrs/function/gamma.rs.html#64
const GAMMA_DK: &[f64] = &[
    2.4857408913875355e-5,
    1.0514237858172197,
    -3.4568709722201625,
    4.512277094668948,
    -2.9828522532357664,
    1.056397115771267,
    -1.9542877319164587e-1,
    1.709705434044412e-2,
    -5.719261174043057e-4,
    4.633994733599057e-6,
    -2.7199490848860772e-9,
];
const TWO_SQRT_E_OVER_PI: f64 = 1.8603827342052657;
const GAMMA_R: f64 = 10.900511;

pub fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        std::f64::consts::PI
            / ((std::f64::consts::PI * x).sin()
                * s
                * TWO_SQRT_E_OVER_PI
                * ((0.5 - x + GAMMA_R) / std::f64::consts::E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / std::f64::consts::E).powf(x - 0.5)
    }
}
