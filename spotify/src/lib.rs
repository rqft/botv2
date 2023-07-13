// EVERYTHING FUCKING SUCKS I HATE THIS FUCKING API THIS IS STUPID HARD TO DO IN RUST BECAUSE ITS ALL STATIC I HATE THIS I HATE YOU AAAAAAAAAAAAAAAAAA

// use reqwest::Client;
// use reqwest::Url;
// use serde::{Deserialize, Serialize};

// pub fn url() -> Url {
//     Url::parse("https://api.spotify.com/v1/").expect("constant url failed")
// }

// pub fn now() -> f64 {
//     std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as f64
// }

// #[derive(Serialize, Deserialize)]
// pub struct Err {
//     pub error: ErrError,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ErrError {
//     pub status: f64,
//     pub message: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Payload {
//     pub id: String,
//     pub href: String,
//     #[serde(rename = "type")]
//     pub kind: Keys,
//     pub uri: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Album {
//     #[serde(flatten)]
//     pub payload: Payload,
//     pub album_type: AlbumTypes,
//     pub total_tracks: f64,
//     pub available_markets: Vec<String>,
//     pub external_urls: ExternalUrls,
//     pub images: Vec<Image>,
//     pub name: String,
//     pub release_date: String,
//     pub release_date_precision: ReleaseDatePrecision,
//     pub restrictions: Option<Restrictions>,
//     pub artists: Vec<Artist>,
//     pub tracks: Vec<Paginator<Track>>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(untagged, rename_all = "snake_case")]
// pub enum AlbumTypes {
//     Album,
//     Single,
//     Compilation,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ExternalUrls {
//     pub spotify: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Image {
//     pub url: String,
//     pub height: f64,
//     pub width: f64,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(untagged, rename_all = "snake_case")]
// pub enum ReleaseDatePrecision {
//     Day,
//     Month,
//     Year,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Restrictions {
//     pub reason: RestrictionReason,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(untagged, rename_all = "snake_case")]
// pub enum RestrictionReason {
//     Market,
//     Product,
//     Explicit,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Artist {
//     #[serde(flatten)]
//     pub payload: Payload,
//     pub external_urls: ExternalUrls,
//     pub followers: Followers,
//     pub genres: Vec<String>,
//     pub images: Vec<Image>,
//     pub name: String,
//     pub popularity: f64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Followers {
//     pub href: Option<String>,
//     pub total: f64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Paginator<T: Serialize> {
//     pub href: String,
//     pub items: Vec<T>,
//     pub limit: f64,
//     pub next: Option<String>,
//     pub offset: f64,
//     pub previous: Option<String>,
//     pub total: f64,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(untagged, rename_all = "snake_case")]
// pub enum IncludeGroups {
//     Album,
//     Single,
//     AppearsOn,
//     Compilation,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Track {
//     #[serde(flatten)]
//     pub payload: Payload,
//     pub album: Album,
//     pub artists: Vec<Artist>,
//     pub available_markets: Vec<String>,
//     pub disc_number: f64,
//     pub duration_ms: f64,
//     pub explicit: bool,
//     pub external_ids: ExternalIds,
//     pub external_urls: ExternalUrls,
//     pub linked_from: Option<Box<Track>>,
//     pub restrictions: Restrictions,
//     pub name: String,
//     pub popularity: f64,
//     pub preview_url: Option<String>,
//     pub track_number: f64,
//     pub is_local: bool,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ExternalIds {
//     pub isrc: String,
//     pub ean: String,
//     pub upc: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Show {
//     #[serde(flatten)]
//     pub payload: Payload,
//     pub available_markets: Vec<String>,
//     pub copyrights: Vec<Copyright>, // die
//     pub description: String,
//     pub html_description: String,
//     pub explicit: bool,
//     pub external_urls: ExternalUrls,
//     pub images: Vec<Image>,
//     pub is_externally_hosted: bool,
//     pub languages: Vec<String>,
//     pub media_type: String,
//     pub name: String,
//     pub publisher: String,
//     pub episodes: Paginator<Episode>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Episode {
//     #[serde(flatten)]
//     pub payload: Payload,
//     pub audio_preview_url: String,
//     pub description: String,
//     pub html_description: String,
//     pub duration_ms: f64,
//     pub explicit: bool,
//     pub external_urls: ExternalUrls,
//     pub images: Vec<Image>,
//     pub is_externally_hosted: bool,
//     pub is_playable: bool,
//     pub language: String,
//     pub languages: Vec<String>,
//     pub name: String,
//     pub release_date: String,
//     pub release_date_precision: ReleaseDatePrecision,
//     pub resume_point: Option<ResumePoint>,
//     pub restrictions: Restrictions,
//     pub show: Show,
// }

// #[derive(Serialize, Deserialize)]
// pub struct ResumePoint {
//     resume_position_ms: f64,
//     fully_played: bool,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Copyright {
//     text: String,
//     #[serde(rename = "type")]
//     kind: CopyrightType,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum CopyrightType {
//     #[serde(rename = "C")]
//     Copyright,
//     #[serde(rename = "P")]
//     Performance,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioFeatures {
//     pub acousticness: f64,
//     pub analysis_url: String,
//     pub danceability: f64,
//     pub duration_ms: f64,
//     pub energy: f64,
//     pub instrumentalness: f64,
//     pub key: f64,
//     pub liveness: f64,
//     pub loudness: f64,
//     pub mode: f64,
//     pub speechiness: f64,
//     pub tempo: f64,
//     pub time_signature: f64,
//     pub track_href: String,
//     pub valence: f64,
//     #[serde(rename = "type")]
//     pub kind: String,
//     pub id: String,
//     pub uri: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysis {
//     pub meta: AudioAnalysisMeta,
//     pub track: AudioAnalysisTrack,
//     pub bars: Vec<AudioAnalysisMeasurement>,
//     pub beats: Vec<AudioAnalysisMeasurement>,
//     pub sections: Vec<AudioAnalysisSection>,
//     pub segments: Vec<AudioAnalysisSegment>,
//     pub tatums: Vec<AudioAnalysisMeasurement>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysisMeta {
//     pub analyzer_version: String,
//     pub platform: String,
//     pub detailed_status: String,
//     pub status_code: f64,
//     pub timestamp: f64,
//     pub analysis_time: f64,
//     pub input_process: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysisTrack {
//     pub num_samples: f64,
//     pub duration: f64,
//     pub sample_md5: String,
//     pub offset_seconds: f64,
//     pub window_seconds: f64,
//     pub analysis_sample_rate: f64,
//     pub analysis_channels: f64,
//     pub end_of_fade_in: f64,
//     pub start_of_fade_out: f64,
//     pub loudness: f64,
//     pub tempo: f64,
//     pub tempo_confidence: f64,
//     pub time_signature: f64,
//     pub time_signature_confidence: f64,
//     pub key: f64,
//     pub key_confidence: f64,
//     pub mode: f64,
//     pub mode_confidence: f64,
//     pub codestring: String,
//     pub code_version: f64,
//     pub echoprintstring: String,
//     pub echoprint_version: f64,
//     pub synchstring: String,
//     pub synch_version: f64,
//     pub rhythmstring: String,
//     pub rhythm_version: f64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysisMeasurement {
//     pub start: f64,
//     pub duration: f64,
//     pub confidence: f64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysisSection {
//     #[serde(flatten)]
//     pub measurement: AudioAnalysisMeasurement,
//     pub loudness: f64,
//     pub tempo: f64,
//     pub tempo_confidence: f64,
//     pub key: f64,
//     pub key_confidence: f64,
//     pub mode: f64,
//     pub mode_confidence: f64,
//     pub time_signature: f64,
//     pub time_signature_confidence: f64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AudioAnalysisSegment {
//     #[serde(flatten)]
//     pub measurement: AudioAnalysisMeasurement,
//     pub loudness_start: f64,
//     pub loudness_max_time: f64,
//     pub loudness_max: f64,
//     pub loudness_end: f64,
//     pub pitches: Vec<f64>,
//     pub timbre: Vec<f64>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum Keys {
//     Album,
//     Artist,
//     Playlist,
//     Track,
//     Show,
//     Episode,
//     User,
//     Category,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum KeysPlural {
//     Albums,
//     Artists,
//     Playlists,
//     Tracks,
//     Shows,
//     Episodes,
// }

// #[derive(Serialize, Deserialize)]
// pub struct SearchTotal {
//     artists: Paginator<Artist>,
//     albums: Paginator<Album>,
//     playlists: Paginator<Playlist>,
//     tracks: Paginator<Track>,
//     shows: Paginator<Show>,
//     episodes: Paginator<Episode>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Search {
//     artists: Option<Paginator<Artist>>,
//     albums: Option<Paginator<Album>>,
//     playlists: Option<Paginator<Playlist>>,
//     tracks: Option<Paginator<Track>>,
//     shows: Option<Paginator<Show>>,
//     episodes: Option<Paginator<Episode>>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Playlist {
//     #[serde(flatten)]
//     payload: Payload,
//     collaborative: bool,
//     description: Option<String>,
//     external_urls: ExternalUrls,
//     followers: Option<Followers>,
//     images: Vec<Image>,
//     name: String,
//     owner: User,
//     public: bool,
//     snapshot_id: String,
//     tracks: Paginator<Track>,
// }

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     #[serde(flatten)]
//     payload: Payload,
//     display_name: String,
//     external_urls: ExternalUrls,
//     followers: Followers,
// }

// #[derive(Serialize, Deserialize)]
// pub struct CategoryPlaylists {
//     albums: Paginator<Album>,
//     message: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Category {
//     #[serde(flatten)]
//     payload: Payload,
//     icons: Vec<Image>,
//     name: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Token {
//     access_token: String,
//     token_type: String,
//     expires_in: f64,
// }

// pub struct Api {
//     pub id: String,
//     pub secret: String,
//     pub token: Option<String>,
//     pub token_expiry: Option<f64>,
//     client: Client,
// }

// pub type Res<T> = Result<T, Err>;

// impl Api {
//     pub fn new(id: String, secret: String) -> Self {
//         Self {
//             id,
//             secret,
//             token: None,
//             token_expiry: None,
//             client: Client::new(),
//         }
//     }

//     pub async fn load_credentials(&mut self) -> &mut Self {
//         if self.token_expiry.is_none() || self.token_expiry.unwrap() > now() {
//             let token = self
//                 .client
//                 .post("https://accounts.spotify.com/api/token?grant_type=client_credentials")
//                 .basic_auth(&self.id, Some(&self.secret))
//                 .header("Content-Type", "application/x-www-form-urlencoded")
//                 .send()
//                 .await
//                 .unwrap()
//                 .json::<Token>()
//                 .await
//                 .unwrap();

//             return self.set_token(token);
//         }

//         self
//     }

//     pub fn set_token(&mut self, token: Token) -> &mut Self {
//         self.token = Some(token.access_token);
//         self.token_expiry = Some(now() + token.expires_in * 1000.0);
//         self
//     }

//     async fn get_json<T: for<'t> Deserialize<'t>>(&self, endpoint: String) -> T {
//         self.client
//             .get(url().join(&endpoint).unwrap())
//             .bearer_auth(self.token.as_ref().unwrap())
//             .send()
//             .await
//             .unwrap()
//             .json()
//             .await
//             .unwrap()
//     }

//     pub async fn get_album(&self, id: String, market: Option<String>) -> Res<Album> {
//         self.get_json(format!(
//             "/albums/{id}{}",
//             market
//                 .map(|x| "?market=".to_string() + &x)
//                 .unwrap_or("".to_string())
//         )).await
//     }
// }
