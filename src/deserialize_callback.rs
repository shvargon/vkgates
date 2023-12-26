use serde::{Deserialize, Serialize}; // 1.0.130

fn extract_post<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Container<T> {
        object: T,
    }
    Container::deserialize(deserializer).map(|a| a.object)
}

fn extract_photo<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Container<T> {
        photo: T,
    }
    Container::deserialize(deserializer).map(|a| a.photo)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestData {
    Confirmation(Confirmation),
    #[serde(deserialize_with = "extract_post")]
    WallPostNew(Post),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Confirmation {
    pub group_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: u32,
    pub text: String,
    pub attachments: Option<Vec<Attachments>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoItems {
    id: u32,
    width: u32,
    height: u32,
    sizes: Vec<PhotoSizes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSizes {
    src: String,
    width: u32,
    height: u32,
    #[serde(rename = "type")]
    types: PhotoSizesType,
}

// @TODO unknown
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PhotoSizesType {
    DisproportionateImage(DisproportionateImageItems),
    ProportionalImage(ProportionalImageItems),
}

#[derive(Debug, Deserialize, Serialize)]
enum DisproportionateImageItems {
    #[serde(rename = "o")]
    Max130,
    #[serde(rename = "p")]
    Max200,
    #[serde(rename = "q")]
    Max320,
    #[serde(rename = "r")]
    Max510
}

#[derive(Debug, Deserialize, Serialize)]
enum ProportionalImageItems {
    #[serde(rename = "s")]
    Max75,
    #[serde(rename = "m")]
    Max130,
    #[serde(rename = "x")]
    Max604,
    #[serde(rename = "y")]
    Max807,
    #[serde(rename = "z")]
    Max1080x1024,
    #[serde(rename = "w")]
    Max2560x2048
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Attachments {
    #[serde(deserialize_with = "extract_photo")]
    Photo(PhotoItems),
    Audio,
    None,
    #[serde(other)]
    Unknown,
}
