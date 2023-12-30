# Использование и разработка
Разработка стандартная для [rust](https://www.rust-lang.org)

Сервер поддерживает prometheus но по умолчанию он отключен для включения необходимо включить поддержку features prometheus:
```bash
cargo run --features prometheus
```

# Примеры компиляции c docker
```bash
# Создать образ docker
make build
# Создать исполняемый файл используя docker
make binary
# Создать образ docker и выгрузить исполняемый файл
make
```

Используя переменные окружения можно поменять параметры компиляции:
```bash
# Создать образ докера с поддержкой prometheus
_BUILD_ARGS_FEATURES=prometheus make build
# Создать исполняемый файл с поддержкой prometheus и выгрузить в директорию target/docker/prometheus
_BUILD_ARGS_BINARYPATH=target/docker/prometheus _BUILD_ARGS_FEATURES=prometheus make binary
```

Все переменные смотрите в начале Makefile.

# env value
- VK_CONFIRMATION_TOKEN - [Токен подтверждения ВК](https://dev.vk.com/ru/api/callback/getting-started#%D0%9F%D0%BE%D0%B4%D0%BA%D0%BB%D1%8E%D1%87%D0%B5%D0%BD%D0%B8%D0%B5%20Callback%20API)
- VK_SECRET - [Секрет ВК для подтверждения что запрос от ВК](https://dev.vk.com/ru/api/callback/getting-started#%D0%A1%D0%B5%D0%BA%D1%80%D0%B5%D1%82%D0%BD%D1%8B%D0%B9%20%D0%BA%D0%BB%D1%8E%D1%87)
- TELOXIDE_TOKEN - [Токен Телеграм](https://core.telegram.org/bots/api#authorizing-your-bot) для [teloxide](https://github.com/teloxide/teloxide?tab=readme-ov-file#setting-up-your-environment)
- TELEGRAM_GROUP_ID - [Группа (или канал) телеграм куда бот будет пересылать записи (API телеграма chat_id)](https://core.telegram.org/bots/api#sendmessage)


# Ограничения телеграм
Неизвестно создает или нет telebot очереди. У телеграма есть ограничения на отправку сообщений в один чат после начинает возвращать 429 ошибку сейчас не учитывается надо искать инфу.

# @TODO
- Замена имени токена телеграм и приём как параметра через clap