
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


pub fn create() {
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


   

    let endpoint = VkEndpointItems {
        vk_secret: state.vk_secret.clone(),
        vk_conrifmation_token: state.vk_confirmation_token.clone(),
        telegram_chat_id: state.telegram_group_id.clone(),
    };


    let uuid = uuid!("44663e93-c1c2-4ea4-95b6-d957632c408f");
    
    let mut endpoints: HashMap<Uuid, VkEndpointItems> = HashMap::new();
    endpoints.insert(uuid, endpoint).unwrap();
    
    let endpoints = VkEndpoints {
        endpoints
    };

    let state = Data::new(
        WebState {
            bot,
            endpoints
        }
    );
}