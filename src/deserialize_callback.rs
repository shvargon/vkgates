use crate::attachments::photo::{extract_photo, PhotoItems};
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
    pub id: i32,
    pub text: String,
    pub attachments: Option<Vec<Attachments>>,
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
