pub use self::{
    barcodes::{Barcodes, BarcodesOptions},
    categories::{Categories, CategoriesOptions, RawCategories},
    categorizers::Categorizers,
    colors::{Colors, ColorsOptions},
    croppings::{Croppings, CroppingsOptions},
    error::Error,
    faces_detections::{FacesDetections, FacesDetectionsOptions},
    faces_similarity::{FacesSimilarity, FacesSimilarityOptions},
    output::RawOutput,
    tags::{RawTags, Tags, TagsOptions},
    text::{Text, TextOptions},
    usage::{Usage, UsageOptions},
};

use reqwest::Client;

pub mod barcodes;
pub mod categories;
pub mod categorizers;
pub mod colors;
pub mod coordinates;
pub mod croppings;
pub mod error;
pub mod faces_detections;
pub mod faces_similarity;
pub mod output;
pub mod tags;
pub mod text;
pub mod usage;

pub struct Imagga {
    pub token: String,
    pub client: reqwest::Client,
}

impl Imagga {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    pub async fn tags(
        &self,
        options: TagsOptions,
        tagger_id: Option<String>,
    ) -> Result<Tags, Error> {
        let mut url = String::from("https://api.imagga.com/v2/tags");

        if let Some(id) = tagger_id {
            url += "/";
            url += &id;
        }

        // {
        //     url += "?";

        //     let mut map = HashMap::new();

        //     if_exists! { options.image_url, image_url, map.insert("image_url", image_url); };
        //     if_exists! { options.image_upload_id, image_upload_id, map.insert("image_upload_id", image_upload_id); };
        //     // if_exists! { options.language, language, map.insert("language", language); };
        //     if_exists! { options.verbose, verbose, map.insert("verbose", if verbose { "1" } else { "0" }.to_string()); };
        //     if_exists! { options.limit, limit, map.insert("limit", limit.to_string()); };
        //     if_exists! { options.threshold, threshold, map.insert("threshold", threshold.to_string()); };
        //     if_exists! { options.decrease_parents, decrease_parents, map.insert("decrease_parents", if decrease_parents { "1" } else { "0" }.to_string()); };

        //     url += &map
        //         .iter()
        //         .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        //         .collect::<Vec<String>>()
        //         .join("&")
        // };

        let o = self
            .client
            .get(url)
            .query(&options)
            .header("Authorization", self.token.clone());

        let text = o.send().await?.text().await?;

        serde_json::from_str::<RawOutput<RawTags>>(&text)
            .unwrap()
            .result()
            .map(Into::into)
    }

    pub async fn categorizers(&self) -> Result<Categorizers, Error> {
        self.client
            .get("https://api.imagga.com/v2/categorizers")
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<Categorizers>>()
            .await?
            .result()
    }

    pub async fn categories(
        &self,
        categorizer_id: String,
        options: CategoriesOptions,
    ) -> Result<Categories, Error> {
        self.client
            .get(format!(
                "https://api.imagga.com/v2/categories/{categorizer_id}"
            ))
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<RawCategories>>()
            .await?
            .result()
            .map(Into::into)
    }

    pub async fn croppings(&self, options: CroppingsOptions) -> Result<Croppings, Error> {
        self.client
            .get("https://api.imagga.com/v2/croppings")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<Croppings>>()
            .await?
            .result()
    }

    pub async fn colours(&self, options: ColorsOptions) -> Result<Colors, Error> {
        self.client
            .get("https://api.imagga.com/v2/colors")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<Colors>>()
            .await?
            .result()
    }

    pub async fn faces_detections(
        &self,
        options: FacesDetectionsOptions,
    ) -> Result<FacesDetections, Error> {
        self.client
            .get("https://api.imagga.com/v2/faces/detections")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<FacesDetections>>()
            .await?
            .result()
    }

    pub async fn faces_similarity(
        &self,
        options: FacesSimilarityOptions,
    ) -> Result<FacesSimilarity, Error> {
        self.client
            .get("https://api.imagga.com/v2/faces/similarity")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<FacesSimilarity>>()
            .await?
            .result()
    }

    // skip: faces_groupings, faces_recognition

    pub async fn text(&self, options: TextOptions) -> Result<Text, Error> {
        let text = self
            .client
            .get("https://api.imagga.com/v2/text")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .text()
            .await?;

        // dbg!(&text);
        serde_json::from_str::<RawOutput<Text>>(&text)
            .unwrap()
            .result()
    }

    // skip: similar_images_categories, similar_images_colors, similar_images_categories_short, remove_background, uploads, tickets, batches

    pub async fn usage(&self, options: UsageOptions) -> Result<Usage, Error> {
        self.client
            .get("https://api.imagga.com/v2/usage")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<Usage>>()
            .await?
            .result()
    }

    pub async fn barcodes(&self, options: BarcodesOptions) -> Result<Barcodes, Error> {
        self.client
            .get("https://api.imagga.com/v2/faces/barcodes")
            .query(&options)
            .header("Authorization", self.token.clone())
            .send()
            .await?
            .json::<RawOutput<Barcodes>>()
            .await?
            .result()
    }
}
