use std::env;
use std::fmt::Debug;

use teloxide::dispatching::update_listeners::webhooks;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::Bot;
use url::Url;

mod swastikas;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PollingMode {
    Polling,
    Webhook,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env();

    let polling_mode = match env::var("POLLING_MODE") {
        Ok(mode) => match mode.as_str() {
            "POLLING" => PollingMode::Polling,
            "WEBHOOK" => PollingMode::Webhook,
            _ => panic!("Unknown polling mode: {}", mode),
        },
        Err(_) => panic!("POLLING_MODE env var is not set, probably..."),
    };

    match polling_mode {
        PollingMode::Polling => {
            log::info!("Starting polling");
            teloxide::repl(bot, |bot: Bot, msg: Message| async move {
                bot.send_dice(msg.chat.id).await?;
                Ok(())
            })
            .await;
        }
        PollingMode::Webhook => {
            let port: u16 = env::var("PORT")
                .expect("PORT env variable is not set")
                .parse()
                .expect("PORT env variable value is not an integer");

            let addr = ([0, 0, 0, 0], port).into();

            let host = env::var("DOMAIN").expect("DOMAIN env variable is not set");
            let url: Url = match format!("https://{host}/webhook/main").parse() {
                Ok(url) => url,
                Err(err) => panic!("Failed to parse URL: {}", err),
            };

            log::info!("Starting webhook");
            log::info!("Port: {}", port.clone().to_string());
            log::info!("URL: {}", url.clone().to_string());

            let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
                .await
                .expect("Couldn't setup webhook");

            teloxide::repl_with_listener(
                bot,
                |bot: Bot, msg: Message| async move {
                    bot.send_dice(msg.chat.id).await?;
                    Ok(())
                },
                listener,
            )
            .await;
        }
    }
}
