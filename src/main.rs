pub mod attachments;
pub mod config;
pub mod deserialize_callback;
use std::fs::Permissions;
use std::sync::{Mutex, Arc};

use teloxide::dispatching::dialogue;
use teloxide::types::{ChatMember, ChatPermissions, User};
use teloxide::RequestError;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use teloxide::{
    dispatching::Dispatcher,
    requests::Requester,
    types::{Message, Update},
    Bot,
};
use uuid::Uuid;

mod vkhandler;

use actix_web::{
    error, get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

#[cfg(feature = "prometheus")]
use actix_web_prom::PrometheusMetricsBuilder;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
struct BotState {
    bot: Bot,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveTelegramGroupID,
    ReceiveVkConfiramationToken {
        telegram_group_id: UserId,
    },
    ReceiveVkSecrets {
        telegram_group_id: UserId,
        vk_confirmation_token: String

    },
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn start(bot: Bot, dialogue: MyDialogue, state: Arc<Mutex<Option<VkState>>>, msg: Message) -> HandlerResult {
    // @TODO Выводить сообщение когда в стейте уже что то есть
    if let Some(state) = state.lock().unwrap().clone() {
        dbg!(state);
    } 
        bot.send_message(msg.chat.id, "Введите номер группы телеграм")
        .await?;
        dialogue.update(State::ReceiveTelegramGroupID).await?;
    
    
    Ok(())
}

async fn receive_telegram_group_id(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        let chat_id: String = text.into();
        let user_id = UserId(6858958681);
        let member = bot.get_chat_member(chat_id, user_id).await;
        if let Ok(_) = member {
            bot.send_message(msg.chat.id, "Подтверждаю я являюсь членом этой группы.")
                .await?;
            bot.send_message(
                msg.chat.id,
                "Теперь пожалуйста введите Confirmation Token сообщества ВК.",
            )
            .await?;
            dialogue
                .update(State::ReceiveVkConfiramationToken {
                    telegram_group_id: user_id,
                })
                .await?;
        } else {
            bot.send_message(
                msg.chat.id,
                "Бот не является членом группы проверьте вводимый id и отправьте заново.",
            )
            .await?;
        }
    } else {
        bot.send_message(
            msg.chat.id,
            "Я не могу понять что вы мне прислали. Пожалуйста введите номер группы текстом",
        )
        .await?;
    }

    Ok(())
}

async fn receive_vk_confirmation_token(
    bot: Bot,
    dialogue: MyDialogue,
    telegram_group_id: UserId,
    msg: Message,
) -> HandlerResult {
    if let Some(vk_confirmation_token) = msg.text() {
        let vk_confirmation_token = vk_confirmation_token.to_string();
        bot.send_message(msg.chat.id, format!("Надеюсь вы прислали верные данные {}", vk_confirmation_token)).await?;
        bot.send_message(msg.chat.id, "Теперь пожалуйста сообщите мне нужно ли использовать Секретный токен ВК. Отправьте No или Токен в ответ").await?;
        dialogue.update(State::ReceiveVkSecrets { 
            telegram_group_id, 
            vk_confirmation_token
        }).await?;
    } else {
        bot.send_message(
            msg.chat.id, 
            "Я не могу понять что вы мне прислали. Пожалуйста введите Confirmation Token сообщества ВК текстом."
        )
            .await?;
    }
    Ok(())
}

async fn receive_vk_secret(
    bot: Bot,
    dialogue: MyDialogue,
    (telegram_group_id, vk_confirmation_token): (UserId, String)
    , msg: Message
) -> HandlerResult {
    if let Some(vk_secret) = msg.text() {
        let vk_secret = match vk_secret  {
            "No" => None,
            secret => Some(secret)
        };

        let id = Uuid::new_v4();
       
        if let Some(secret) = vk_secret {
            bot.send_message(
                msg.chat.id,
                 format!("Для доступа будет использовать Секретный токен {}", secret)
                ).await?;
        } else {
            
        }
        dialogue.exit().await?;
    } else {
        bot.send_message(
            msg.chat.id, 
            "Я не могу понять что вы мне прислали. Пожалуйста отправьте No если вы не будете использовать секретный токен или если Секретный токен ВК будет использоваться отправьте секретный токен."
        )
            .await?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct VkState {
    value: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // @TODO thread spawn?
   
    let (host, port, state) = config::read_config();
    let host = host.unwrap_or("0.0.0.0".to_string());
    let port = port.unwrap_or(3000);
    let state = Data::new(state);

    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        dbg!(&err);
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    });

    let bot = Bot::from_env();
    
    let botstate: Mutex<Option<VkState>> = Mutex::new(Some(VkState { value: "Hello".to_string() }));
    let botstate = Arc::new(botstate);

    Dispatcher::builder(
        bot.clone(),
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(
                dptree::case![State::ReceiveTelegramGroupID].endpoint(receive_telegram_group_id),
            )
            .branch(
                dptree::case![State::ReceiveVkConfiramationToken { telegram_group_id }]
                    .endpoint(receive_vk_confirmation_token),
            ).branch(
                dptree::case![State::ReceiveVkSecrets { telegram_group_id, vk_confirmation_token }]
                    .endpoint(receive_vk_secret),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new(), botstate])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

   

    #[cfg(feature = "prometheus")]
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        #[cfg(not(feature = "prometheus"))]
        let app = App::new();

        #[cfg(feature = "prometheus")]
        let app = App::new().wrap(prometheus.clone());

        app.service(hello).service(
            web::scope("/")
                .app_data(json_config.clone())
                .app_data(state.clone())
                .app_data(web::Data::new(BotState { bot: bot.clone() }))
                // .route(web::post().to(vkhandler::handle_callback)),
                .service(vkhandler::handle_callback),
        )
    })
    .bind((host, port))?
    .run()
    .await
}
