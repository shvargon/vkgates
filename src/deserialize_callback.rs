use serde::{Deserialize, Serialize}; // 1.0.130

fn extract_message<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct Container<T> {
        message: T,
    }
    Container::deserialize(deserializer).map(|a| a.message)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Data {
    Confirmation(Confirmation),
    #[serde(deserialize_with = "extract_message")]
    MessageNew(Message),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Confirmation {
    pub group_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: u32,
    pub text: String,
    pub attachments: Option<Vec<Attachments>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Attachments {
    Photo,
    Audio,
    None,
    #[serde(other)]
    Unknown,
}
