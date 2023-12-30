# Docker build image
```bash
docker build --target=server -t shvargon/vkgates .
```

# Docker copy binary to bin directory
```bash
docker build --target=binaries --output=bin .
```

# env value
- VK_CONFIRMATION_TOKEN - [Токен подтверждения ВК](https://dev.vk.com/ru/api/callback/getting-started#%D0%9F%D0%BE%D0%B4%D0%BA%D0%BB%D1%8E%D1%87%D0%B5%D0%BD%D0%B8%D0%B5%20Callback%20API)
- VK_SECRET - [Секрет ВК для подтверждения что запрос от ВК](https://dev.vk.com/ru/api/callback/getting-started#%D0%A1%D0%B5%D0%BA%D1%80%D0%B5%D1%82%D0%BD%D1%8B%D0%B9%20%D0%BA%D0%BB%D1%8E%D1%87)
- TELOXIDE_TOKEN - [Токен Телеграм](https://core.telegram.org/bots/api#authorizing-your-bot) для [teloxide](https://github.com/teloxide/teloxide?tab=readme-ov-file#setting-up-your-environment)
- TELEGRAM_GROUP_ID - [Группа (или канал) телеграм куда бот будет пересылать записи (API телеграма chat_id)](https://core.telegram.org/bots/api#sendmessage)


# Ограничения телеграм
Неизвестно создает или нет telebot очереди. У телеграма есть ограничения на отправку сообщений в один чат после начинает возвращать 429 ошибку сейчас не учитывается надо искать инфу
