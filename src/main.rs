mod deserialize_callback;
use actix_web::{
    error, get,
    web::Json,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use deserialize_callback::*;
use dotenv::dotenv;
use std::env;
use teloxide::types::{InputFile, InputMedia};
use teloxide::{prelude::*, types::InputMediaPhoto};
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, env)]
    vk_confirmation_token: String,
    vk_group_id: String, // #[clap(long, env)]
                         // vkcommunityid: u32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index(req: Json<RequestData>, state: Data<AppState>) -> impl Responder {
    // @TODO check secret key equal

    match req.into_inner() {
        RequestData::Confirmation(val) => {
            dbg!("Respond confirmation", val);
            HttpResponse::Ok().body(state.vk_confirmation_token.clone())
        }
        RequestData::WallPostNew(val) => {
            dbg!("Respond message", &val);
            // @todo handle error

            let text = &val.text;
            let text = format!(
                "{} https://vk.com/wall-{}_{}",
                text, state.vk_group_id, val.id
            );

            if let Some(atachments) = &val.attachments {
                // let mut

                let mut photos: Vec<InputMedia> = vec![];
                for current in atachments {
                    match current {
                        Attachments::Photo(values) => {
                            if let Some(max) = PhotoItems::max_proportional_image(values) {
                                let url = Url::parse(&max.url);
                                if let Ok(url) = url {
                                    let media = InputFile::url(url);
                                    let caption: Option<String> = if photos.len() == 0 {
                                        Some(text.clone())
                                    } else {
                                        None
                                    };

                                    let media = InputMediaPhoto {
                                        media: media,
                                        has_spoiler: false,
                                        caption: caption,
                                        parse_mode: None,
                                        caption_entities: None,
                                    };
                                    let media: InputMedia = InputMedia::Photo(media);
                                    photos.push(media);
                                }
                                // let url = InputFile::url(max.url)
                            }
                        }
                        _ => {}
                    }
                }

                let bot = &state.bot;

                let _ = if photos.len() == 0 {
                    bot.send_message(state.telegram_group_id.clone(), &text)
                        .await;
                } else {
                    bot.send_media_group(state.telegram_group_id.clone(), photos)
                        .await;
                };
            } else {
                let sendmsg = state
                    .bot
                    .send_message(state.telegram_group_id.clone(), &text)
                    .await;

                match sendmsg {
                    Ok(_) => {
                        println!("msg send")
                    }
                    Err(err) => {
                        println!("#{:?}", err)
                    }
                }
            }

            HttpResponse::Ok().body("ok")
        }
        _ => HttpResponse::Ok().body("ok"),
    }
}

#[derive(Debug, Clone)]
struct AppState {
    vk_confirmation_token: String,
    vk_group_id: String,
    // vkcommunityid: u32,
    bot: Bot,
    telegram_group_id: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    // @TODO thread spawn?
    let bot = Bot::from_env();
    let groupid = env::var("TELEGRAM_GROUP_ID").unwrap();

    let state = Data::new(AppState {
        vk_confirmation_token: cli.vk_confirmation_token,
        vk_group_id: cli.vk_group_id,
        // vkcommunityid: cli.vkcommunityid,
        bot: bot.clone(),
        telegram_group_id: groupid,
    });

    // bot.send_message(groupid, "hello world").await;

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            // .limit(904096)
            .error_handler(|err, _req| {
                dbg!(&err);
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(hello).service(
            web::resource("/")
                // change json extractor configuration
                .app_data(json_config)
                .app_data(state.clone())
                .route(web::post().to(index)),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
