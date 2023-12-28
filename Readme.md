# Docker build image
```bash
docker build --target=server -t shvargon/vkapi .
```

# Docker copy binary to bin directory
```bash
docker build --target=binaries --output=bin .
```

# env value
- VK_CONFIRMATION_TOKEN - Токен подтверждения ВК
- VK_COMMUNITY_ID - Номер групы в вк
- TELOXIDE_TOKEN - Токен Телеграм
- TELEGRAM_GROUP_ID - Группа к которой привязан бот

# Ограничения телеграм
Неизвестно создает или нет telebot очереди. У телеграма есть ограничения на отправку сообщений в один чат после начинает возвращать 429 ошибку сейчас не учитывается надо искать инфу