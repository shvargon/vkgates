use serde::{Deserialize, Serialize}; // 1.0.130

pub fn extract_photo<'de, D, T>(deserializer: D) -> Result<T, D::Error>
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
pub struct PhotoItems {
    id: u32,
    pub sizes: Vec<PhotoSizes>,
}

impl PhotoItems {
    pub fn max_proportional_image(&self) -> Option<&PhotoSizes> {
        let max = &self
            .sizes
            .iter()
            .max_by(|a, b| a.types.partial_cmp(&b.types).unwrap());

        return max.clone();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSizes {
    pub url: String,
    width: u32,
    height: u32,
    #[serde(rename = "type")]
    types: PhotoSizesType,
}

// @TODO unknown
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(untagged)]
enum PhotoSizesType {
    DisproportionateImage(DisproportionateImageItems),
    ProportionalImage(ProportionalImageItems),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
enum DisproportionateImageItems {
    #[serde(rename = "o")]
    Max130,
    #[serde(rename = "p")]
    Max200,
    #[serde(rename = "q")]
    Max320,
    #[serde(rename = "r")]
    Max510,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
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
    Max2560x2048,
}