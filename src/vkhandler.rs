use crate::attachments::photo::PhotoItems;
use crate::config::*;
use crate::deserialize_callback::*;
use actix_web::{post, web::Data, web::Json, HttpResponse, Responder};
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

#[post("/{group_uuid}")]
pub async fn handle_callback(req: Json<RequestVk>, state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// pub async fn index(req: Json<RequestVk>, state: Data<AppState>) -> impl Responder {
//     let req = req.into_inner();
//     let RequestVk {
//         channel_id,
//         secret: req_secret,
//         data,
//     } = req;

//     if let Some(secret) = &state.vk_secret {
//         if secret != &req_secret {
//             return HttpResponse::Forbidden().body("secret don`t match");
//         }
//     }

//     dbg!(&data);

//     if let RequestData::Confirmation = data {
//         return HttpResponse::Ok().body(state.vk_confirmation_token.clone());
//     }

//     if let RequestData::WallPostNew(post) = data {
//         dbg!("Respond message",);

//         let bot = &state.bot;
//         // let group_id: String = state.telegram_group_id.clone();
//         let text = format!(
//             "{} https://vk.com/wall-{}_{}",
//             &post.text, channel_id, post.id
//         );

//         let photos = post.attachments.unwrap_or(vec![]);

//         let photos: Vec<InputMedia> = photos
//             .iter()
//             .filter_map(|val| match val {
//                 Attachments::Photo(attachments) => Some(attachments),
//                 _ => None,
//             })
//             .filter_map(|val| PhotoItems::max_proportional_image(val))
//             .filter_map(|val| match Url::parse(val.url.as_str()) {
//                 Ok(url) => {
//                     let media = InputMediaPhoto {
//                         media: InputFile::url(url),
//                         caption: None,
//                         has_spoiler: false,
//                         parse_mode: None,
//                         caption_entities: None,
//                     };
//                     Some(media)
//                 }
//                 Err(_) => None,
//             })
//             .enumerate()
//             .map(|(index, value)| {
//                 let value = if index == 0 {
//                     InputMediaPhoto {
//                         caption: Some(text.clone()),
//                         ..value
//                     }
//                 } else {
//                     value
//                 };
//                 InputMedia::Photo(value)
//             })
//             .collect();

//         let group_id = state.telegram_group_id.clone();

//         if photos.len() == 0 {
//             bot_send_text(bot, group_id, text).await;
//         } else {
//             match bot.send_media_group(group_id, photos).await {
//                 Ok(_) => println!("msg send to telegram"),
//                 Err(e) => println!("#{:?}", e),
//             }
//         }
//     }

//     HttpResponse::Ok().body("ok")
// }
