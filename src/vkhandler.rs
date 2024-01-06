use crate::{
    attachments::photo::PhotoItems,
    deserialize_callback::{Attachments, RequestData},
    endpoints::VkEndpointItems,
};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use teloxide::{
    requests::Requester,
    types::{InputFile, InputMedia, InputMediaPhoto},
    Bot,
};
use url::Url;
use uuid::Uuid;

use crate::{
    deserialize_callback::RequestVk,
    endpoints::{self, VkEndpoints},
    WebState,
};

enum EndpointStatus {
    Waiting,
    Known,
    Unknown,
}

struct Endpoint {
    status: EndpointStatus,
    endpoint: Option<VkEndpointItems>,
}

async fn bot_send_text(bot: &Bot, group_id: String, text: String) {
    let sendmsg = bot.send_message(group_id, text).await;

    match sendmsg {
        Ok(_) => println!("msg send"),
        Err(err) => println!("#{:?}", err),
    }
}

pub async fn handle_callback(
    uuid: web::Path<Uuid>,
    state: Data<WebState>,
    req: Json<RequestVk>,
) -> impl Responder {
    let RequestVk {
        channel_id,
        secret,
        data,
    } = req.into_inner();

    let uuid = uuid.into_inner();
    let endpoint = state.endpoints.lock().unwrap().clone();

    if let RequestData::Confirmation = data {
        let waiting = state.waiting_confirmation_endpoints.lock().unwrap().clone();

        let current = if let Some(endpoint) = waiting.check(uuid) {
            Endpoint {
                status: EndpointStatus::Waiting,
                endpoint: Some(endpoint.clone()),
            }
        } else if let Some(endpoint) = endpoint.check(uuid) {
            Endpoint {
                status: EndpointStatus::Known,
                endpoint: Some(endpoint.clone()),
            }
        } else {
            Endpoint {
                status: EndpointStatus::Unknown,
                endpoint: None,
            }
        };

        if let Some(endpoint) = current.endpoint {
            if !endpoint.verify_secret(secret) {
                return HttpResponse::Forbidden().body("secret don`t match");
            }

            // @TODO
            if let EndpointStatus::Waiting = current.status {
                println!("Точка ещё не подтверждена тут функционал подтверждения")
            }

            return HttpResponse::Ok().body(endpoint.vk_confirmation_token.clone());
        }

        return HttpResponse::NotFound().body("Not found");
    } else if let RequestData::WallPostNew(post) = data {
        let current = if let Some(endpoint) = endpoint.check(uuid) {
            Endpoint {
                status: EndpointStatus::Known,
                endpoint: Some(endpoint.clone()),
            }
        } else {
            Endpoint {
                status: EndpointStatus::Unknown,
                endpoint: None,
            }
        };

        if let Some(endpoint) = current.endpoint {
            if !endpoint.verify_secret(secret) {
                return HttpResponse::Forbidden().body("secret don`t match");
            }

            let group_id = endpoint.telegram_chat_id;

            let bot = &state.bot;
            // let group_id: String = state.telegram_group_id.clone();
            let text = format!(
                "{} https://vk.com/wall-{}_{}",
                &post.text, channel_id, post.id
            );

            let photos = post.attachments.unwrap_or(vec![]);

            let photos: Vec<InputMedia> = photos
                .iter()
                .filter_map(|val| match val {
                    Attachments::Photo(attachments) => Some(attachments),
                    _ => None,
                })
                .filter_map(|val| PhotoItems::max_proportional_image(val))
                .filter_map(|val| match Url::parse(val.url.as_str()) {
                    Ok(url) => {
                        let media = InputMediaPhoto {
                            media: InputFile::url(url),
                            caption: None,
                            has_spoiler: false,
                            parse_mode: None,
                            caption_entities: None,
                        };
                        Some(media)
                    }
                    Err(_) => None,
                })
                .enumerate()
                .map(|(index, value)| {
                    let value = if index == 0 {
                        InputMediaPhoto {
                            caption: Some(text.clone()),
                            ..value
                        }
                    } else {
                        value
                    };
                    InputMedia::Photo(value)
                })
                .collect();

            if photos.len() == 0 {
                bot_send_text(bot, group_id, text).await;
            } else {
                match bot.send_media_group(group_id, photos).await {
                    Ok(_) => println!("msg send to telegram"),
                    Err(e) => println!("#{:?}", e),
                }
            }
        }
    }

    HttpResponse::Ok().body("ok")
}
