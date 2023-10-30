use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct QueryOptions {
    pub input: String,
    // appid: String, // inferred!
    pub format: Option<String>,
    // output: String, // nope. json only.
    pub includepodid: Option<String>,
    pub excludepodid: Option<String>,
    pub podtitle: Option<String>,
    pub podindex: Option<String>,
    pub scanner: Option<String>,
    pub ip: Option<String>,
    pub latlong: Option<String>,
    pub location: Option<String>,
    pub width: Option<usize>,
    pub maxwidth: Option<usize>,
    pub plotwidth: Option<usize>,
    pub mag: Option<f64>,
    pub scantimeout: Option<f64>,
    pub podtimeout: Option<f64>,
    pub formattimeout: Option<f64>,
    pub parsetimeout: Option<f64>,
    pub totaltimeout: Option<f64>,
    pub r#async: Option<String>,
    pub reinterpret: Option<bool>,
    pub translation: Option<bool>,
    pub ignorecase: Option<bool>,
    pub assumption: Option<String>,
    pub podstate: Option<String>,
    pub units: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryOutputHolder {
    pub queryresult: QueryOutput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryOutput {
    pub success: bool,
    pub pods: Vec<Pod>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod {
    pub title: String,
    pub scanner: String,
    pub id: String,
    pub position: usize,
    pub subpods: Vec<Subpod>,
    // pub infos: Option<OneOrMany<Info>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subpod {
    pub title: String, // useless?
    pub img: Option<Image>,
    pub plaintext: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Info {
    pub text: String,
    pub img: Image,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub title: String,
}
