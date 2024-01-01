use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VkEndpointItems {
    pub vk_conrifmation_token: String,
    pub vk_secret: Option<String>,
    pub telegram_chat_id: String,
}

impl VkEndpointItems {
    pub fn verify_secret(&self, secret: String) -> bool {
        if let Some(vk_secret) = &self.vk_secret {
            if vk_secret != &secret {
                return true;
            } else {
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug)]
pub struct VkEndpoints {
    endpoints: HashMap<Uuid, VkEndpointItems>,
}

impl VkEndpoints {
    pub fn check(&self, uuid: Uuid) -> Option<&VkEndpointItems> {
        self.endpoints.get(&uuid)
    }

    pub fn new(
        vk_conrifmation_token: String,
        vk_secret: Option<String>,
        telegram_chat_id: String,
        uuid: Uuid,
    ) -> Self {
        let endpoint = VkEndpointItems {
            vk_secret,
            vk_conrifmation_token,
            telegram_chat_id,
        };
        let mut endpoints: HashMap<Uuid, VkEndpointItems> = HashMap::new();
        endpoints.insert(uuid, endpoint);

        VkEndpoints { endpoints }
    }
}
