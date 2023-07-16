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
    Self: Into<String>,
{
    fn str_match(self, content: &str) -> Vec<String> {
        Regex::new(content)
            .unwrap()
            .find_iter(self.to::<String>().as_str())
            .map(|x| x.as_str().to_owned())
            .collect::<Vec<_>>()
    }

    fn str_replace(self, content: &str, with: impl Replacer) -> String {
        Regex::new(content)
            .unwrap()
            .replace(self.to::<String>().as_str(), with)
            .to_string()
    }
}

impl<T> RegexExt for T where T: Into<String> {}
