use std::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

use regex::{Regex, Replacer};

pub trait To {
    fn to<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    fn from<T>(value: T) -> Self
    where
        T: Into<Self>,
        Self: Sized,
    {
        value.into()
    }
}

impl<T> To for T {}

pub trait RegexExt
where
    Self: Into<String> + Clone,
{
    fn str_match(&self, content: &str) -> Vec<String> {
        Regex::new(content)
            .unwrap()
            .find_iter(self.clone().to::<String>().as_str())
            .map(|x| x.as_str().to_owned())
            .collect::<Vec<_>>()
    }

    fn str_replace(&self, content: &str, with: impl Replacer) -> String {
        Regex::new(content)
            .unwrap()
            .replace_all(self.clone().to::<String>().as_str(), with)
            .to_string()
    }

    fn test(&self, content: &str) -> bool {
        Regex::new(content)
            .unwrap()
            .is_match(self.clone().to::<String>().as_str())
    }
}

impl<T> RegexExt for T where T: Into<String> + Clone {}

pub trait Do {
    fn do_mut<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
        Self: Sized,
    {
        f(&mut self);
        self
    }

    fn do_ref<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
        Self: Sized,
    {
        f(&self);
        self
    }
}

impl<T> Do for T {}

pub fn test() {
    struct A;
    struct B;

    let x: Option<&(A, B)> = Some(&(A, B));
    let y = x.map(|x| &x.1);
}

pub trait FloatExt {
    fn is_close_by(self, rhs: Self, epsilon: Self) -> bool;
}

impl FloatExt for f64 {
    fn is_close_by(self, rhs: Self, epsilon: Self) -> bool {
        (self - rhs) < epsilon
    }
}

pub trait Fmt
where
    Self: Sized,
{
    fn display(self) -> String
    where
        Self: Display,
    {
        format!("{self}")
    }

    fn debug(self) -> String
    where
        Self: Debug,
    {
        format!("{self:?}")
    }

    fn binary(self) -> String
    where
        Self: Binary,
    {
        format!("{self:b}")
    }

    fn octal(self) -> String
    where
        Self: Octal,
    {
        format!("{self:o}")
    }

    fn lower_hex(self) -> String
    where
        Self: LowerHex,
    {
        format!("{self:x}")
    }

    fn upper_hex(self) -> String
    where
        Self: UpperHex,
    {
        format!("{self:X}")
    }

    fn lower_exp(self) -> String
    where
        Self: LowerExp,
    {
        format!("{self:e}")
    }

    fn upper_exp(self) -> String
    where
        Self: UpperExp,
    {
        format!("{self:E}")
    }

    fn pointer(self) -> String
    where
        Self: Pointer,
    {
        format!("{self:p}")
    }
}

impl<T> Fmt for T {}
