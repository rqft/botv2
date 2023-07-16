use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct QueryOptions {
    pub input: String,
    // appid: String, // inferred!
    pub format: Option<String>,
    // output: String, // nope. json only.
    // includepodid: Option<String>,
    // excludepodid: Option<String>,
    // podtitle: Option<String>,
    // podindex: Option<String>,
    // scanner: Option<String>,
    //
    // ip: Option<String>,
    // latlong: Option<String>,
    // location: Option<String>,
    //
    // width: Option<usize>,
    // maxwidth: Option<usize>,
    // plotwidth: Option<usize>,
    // mag: Option<f64>,
    //
    // scantimeout: Option<f64>,
    // podtimeout: Option<f64>,
    // formattimeout: Option<f64>,
    // parsetimeout: Option<f64>,
    // totaltimeout: Option<f64>,
    // r#async: Option<String>,
    //
    // reinterpret: Option<bool>,
    // translation: Option<bool>,
    // ignorecase: Option<bool>,
    // sig
    // assumption: Option<String>,
    // podstate: Option<String>,
    // units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOutputHolder {
    pub queryresult: QueryOutput,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOutput {
    pub success: bool,
    pub pods: Vec<Pod>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pod {
    pub title: String,
    pub scanner: String,
    pub id: String,
    pub position: usize,
    pub subpods: Vec<Subpod>,
    // pub infos: Option<OneOrMany<Info>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subpod {
    pub title: String, // useless?
    pub img: Option<Image>,
    pub plaintext: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub text: String,
    pub img: Image,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub title: String,
}
