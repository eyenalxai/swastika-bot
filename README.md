## What Swastika Am I Bot

Телеграм инлайн-бот для определения какая вы сегодня свастика из
[стихотворения Ярослава Могутина](http://kolonna.mitin.com/people/mogutin/ss/swast.shtml).

### Как пользоваться

Как любым другим инлайн-ботом, просто напишите юзернейм [@what_swastika_am_i_bot ](https://t.me/what_swastika_am_i_bot )
бота в любом чате.

### Как запустить бота самостоятельно

1. Задать переменную окружения `TELOXIDE_TOKEN` с токеном бота полученным от [@BotFather](https://t.me/BotFather).
2. Задать переменную окружения `POLLING_MODE` со значением `POLLING` или `WEBHOOK`.

- Если `POLLING_MODE` == `POLLING`, то больше ничего не нужно.
- Если `POLLING_MODE` == `WEBHOOK`, то нужно задать еще переменные окружения:
-
    - `PORT` - порт на котором будет работать бот.
-
    - `DOMAIN` - домен на котором будет работать бот, например *example.com*.

3. Запустить бота командой `cargo run --release`.