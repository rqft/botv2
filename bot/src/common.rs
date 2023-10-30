use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Div, Rem},
};

use clap::{CommandFactory, FromArgMatches};
use exmex::{FloatOpsFactory, MakeOperators, Operator};
use imagga::Imagga;
use ucd::Ucd;
use wa::Wolfram;

pub struct Data {
    pub imagga: Imagga,
    pub wolfram: Wolfram,
    pub req: reqwest::Client,
    pub ucd: Ucd,
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

pub fn get_output(expr: &str, x: &[f64], names: &[&str]) -> std::result::Result<f64, String> {
    use exmex::prelude::*;
    #[derive(Clone, Debug)]
    struct ExtendedOpsFactory;
    impl MakeOperators<f64> for ExtendedOpsFactory {
        fn make<'a>() -> Vec<exmex::Operator<'a, f64>> {
            let mut ops = FloatOpsFactory::<f64>::make();

            ops.push(Operator::make_unary("factorial", |x| gamma(x + 1.0)));
            ops.push(Operator::make_unary("sign", |x| x.signum()));

            ops.push(Operator::make_bin(
                "%",
                exmex::BinOp {
                    apply: |x, y| x.rem_euclid(y),
                    prio: 1,
                    is_commutative: true,
                },
            ));

            ops
        }
    }
    let e = FlatEx::<f64, ExtendedOpsFactory>::parse(&format!("(0*({}))+{expr}", names.join("+")))
        .map_err(|x| x.to_string())?;
    e.eval_relaxed(x).map_err(|x| x.msg().to_string())
}

const Y: &str = "Yes";
const N: &str = "No";

pub const fn yn(t: bool) -> &'static str {
    if t {
        Y
    } else {
        N
    }
}

pub struct Desc(String);

impl Desc {
    pub const fn new() -> Self {
        Desc(String::new())
    }

    pub fn nl(&mut self) -> &mut Self {
        self.0 += "\n";
        self
    }

    pub fn field(&mut self, field: impl Display, value: impl Display) -> &mut Self {
        self.0 += &format!("{}: {}\n", field, value);
        self
    }

    pub fn field_quote(&mut self, field: impl Display, value: impl Display) -> &mut Self {
        self.0 += &format!(
            "\u{200b}\u{2001}\u{200b}\u{2001}\u{200b}{}: {}\n",
            field, value
        );
        self
    }

    pub fn emoji(
        &mut self,
        emoji: impl Display,
        field: impl Display,
        value: impl Display,
    ) -> &mut Self {
        self.0 += &format!("{} {}: {}\n", emoji, field, value);
        self
    }

    pub fn finish(self) -> String {
        self.0
    }
}

pub struct FrequencyTable<T>(HashMap<T, usize>)
where
    T: Hash + Eq + PartialEq;

impl<T> FrequencyTable<T>
where
    T: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, k: T) -> &mut Self {
        if let Some(t) = self.0.get(&k) {
            self.0.insert(k, t + 1);
        } else {
            self.0.insert(k, 1);
        }

        self
    }
}

impl<T> IntoIterator for FrequencyTable<T>
where
    T: Hash + Eq + PartialEq,
{
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub trait AsVec<T> {
    fn as_vec<F>(self, f: F) -> std::vec::IntoIter<T>
    where
        Self: Sized,
        F: FnOnce(Vec<T>) -> Vec<T>;
}

impl<T, U> AsVec<T> for U
where
    U: Iterator<Item = T>,
{
    fn as_vec<F>(self, f: F) -> std::vec::IntoIter<T>
    where
        F: FnOnce(Vec<T>) -> Vec<T>,
    {
        f(self.collect::<Vec<_>>()).into_iter()
    }
}

pub fn format_radix<T>(mut x: T, radix: T) -> String
where
    T: TryInto<u32> + TryFrom<u32> + Rem<T, Output = T> + Div<T, Output = T> + PartialEq + Copy,
    <T as TryInto<u32>>::Error: Debug,
    <T as TryFrom<u32>>::Error: Debug,
{
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result
            .push(std::char::from_digit(m.try_into().unwrap(), radix.try_into().unwrap()).unwrap());
        if x == T::try_from(0u32).unwrap() {
            break;
        }
    }
    result.into_iter().rev().collect()
}

pub const TAIL: &str = "\u{276f}";
pub const TAB: &str = "\u{2003}\u{200b}";
pub const DELVE: &str = "├─";
pub const DERIVE: &str = "└─";
pub const BAR: &str = " │";
