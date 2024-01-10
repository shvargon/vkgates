use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VkEndpointItems {
    pub vk_confirmation_token: String,
    pub vk_secret: Option<String>,
    pub telegram_chat_id: String,
}

impl VkEndpointItems {
    pub fn verify_secret(&self, secret: String) -> bool {
        if let Some(vk_secret) = &self.vk_secret {
            if vk_secret == &secret {
                return true;
            } else {
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VkEndpoints {
    endpoints: HashMap<Uuid, VkEndpointItems>,
    filename: String,
}

impl VkEndpoints {
    pub async fn read(filename: String) -> Result<VkEndpoints, Box<dyn Error>> {
        let file = File::open(filename.as_str())?;
        let buff = BufReader::new(file);
        let endpoints: HashMap<Uuid, VkEndpointItems> = serde_yaml::from_reader(buff)?;

        Ok(VkEndpoints {
            filename,
            endpoints,
        })
    }

    pub fn check(&self, uuid: Uuid) -> Option<&VkEndpointItems> {
        self.endpoints.get(&uuid)
    }

    pub async fn add(
        &mut self,
        vk_conrifmation_token: String,
        vk_secret: Option<String>,
        telegram_chat_id: String,
        uuid: Uuid,
    ) -> std::io::Result<()> {
        let endpoint = VkEndpointItems {
            vk_secret,
            vk_confirmation_token: vk_conrifmation_token,
            telegram_chat_id,
        };

        self.endpoints.insert(uuid, endpoint);

        let file = File::create(self.filename.as_str())?;
        let mut writer = BufWriter::new(file);
        serde_yaml::to_writer(&mut writer, &self.endpoints).expect("config not write");
        writer.flush()?;
        Ok(())
    }
}
