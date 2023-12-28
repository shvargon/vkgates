mod config;
mod deserialize_callback;

use actix_web::{
    error, get,
    web::Json,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use config::*;
use deserialize_callback::attachments::photo::PhotoItems;
use deserialize_callback::*;
use teloxide::types::{InputFile, InputMedia};
use teloxide::{prelude::*, types::InputMediaPhoto};
use url::Url;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index(req: Json<RequestData>, state: Data<AppState>) -> impl Responder {
    // @TODO check secret key equal
    async fn bot_send_text(bot: &Bot, group_id: String, text: String) {
        let sendmsg = bot.send_message(group_id, text).await;

        match sendmsg {
            Ok(_) => println!("msg send"),
            Err(err) => println!("#{:?}", err),
        }
    }

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
                    Some(InputMedia::Photo(media))
                }
                Err(_) => None,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // @TODO thread spawn?

    let bot = Bot::from_env();
    let state = Data::new(config::read_config(bot));
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        dbg!(&err);
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    });

    HttpServer::new(move || {
        App::new().service(hello).service(
            web::resource("/")
                .app_data(json_config.clone())
                .app_data(state.clone())
                .route(web::post().to(index)),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
