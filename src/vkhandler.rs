use crate::config::*;
use crate::deserialize_callback::attachments::photo::PhotoItems;
use crate::deserialize_callback::*;
use actix_web::{web::Data, web::Json, HttpResponse, Responder};
use teloxide::types::{InputFile, InputMedia};
use teloxide::{prelude::*, types::InputMediaPhoto};
use url::Url;

async fn bot_send_text(bot: &Bot, group_id: String, text: String) {
    let sendmsg = bot.send_message(group_id, text).await;

    match sendmsg {
        Ok(_) => println!("msg send"),
        Err(err) => println!("#{:?}", err),
    }
}

pub async fn index(req: Json<RequestData>, state: Data<AppState>) -> impl Responder {
    // @TODO check secret key equal

    let req = req.into_inner();

    if let RequestData::Confirmation(_) = req {
        return HttpResponse::Ok().body(state.vk_confirmation_token.clone());
    }

    if let RequestData::WallPostNew(val) = req {
        dbg!("Respond message", &val);

        let bot = &state.bot;
        let group_id: String = state.telegram_group_id.clone();
        let text = format!(
            "{} https://vk.com/wall-{}_{}",
            &val.text, state.vk_community_id, val.id
        );

        let photos = val.attachments.unwrap_or(vec![]);

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
            bot.send_media_group(group_id.clone(), photos).await;
        }
    }

    HttpResponse::Ok().body("ok")
}
