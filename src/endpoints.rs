use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct VkEndpoints {
    endpoints: HashMap<Uuid, VkEndpointItems>,
    filename: String,
}

impl VkEndpoints {
    pub fn check(&self, uuid: Uuid) -> Option<&VkEndpointItems> {
        self.endpoints.get(&uuid)
    }

    pub fn add(
        &mut self,
        vk_conrifmation_token: String,
        vk_secret: Option<String>,
        telegram_chat_id: String,
        uuid: Uuid,
    ) {
        let endpoint = VkEndpointItems {
            vk_secret,
            vk_confirmation_token: vk_conrifmation_token,
            telegram_chat_id,
        };

        self.endpoints.insert(uuid, endpoint);
    }

    pub fn new(filename: String) -> Self {
        let mut endpoints: HashMap<Uuid, VkEndpointItems> = HashMap::new();

        VkEndpoints {
            endpoints,
            filename,
        }
    }
}
