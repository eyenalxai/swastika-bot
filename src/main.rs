use std::env;

use teloxide::dispatching::update_listeners::webhooks;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::types::Update;
use teloxide::{dptree, Bot};
use url::Url;

use crate::util::{swastika_answer, PollingMode};

mod swastikas;
mod util;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();

    let bot = Bot::from_env();

    let polling_mode = match env::var("POLLING_MODE") {
        Ok(mode) => match mode.as_str() {
            "POLLING" => PollingMode::Polling,
            "WEBHOOK" => PollingMode::Webhook,
            _ => panic!("Unknown polling mode: {}", mode),
        },
        Err(_) => panic!("POLLING_MODE env var is not set, probably..."),
    };

    let swastika_handler = Update::filter_inline_query().branch(dptree::endpoint(swastika_answer));

    match polling_mode {
        PollingMode::Polling => {
            log::info!("Polling!");

            Dispatcher::builder(bot, swastika_handler)
                .enable_ctrlc_handler()
                .build()
                .dispatch()
                .await
        }

        PollingMode::Webhook => {
            let port: u16 = env::var("PORT")
                .expect("PORT env variable is not set")
                .parse()
                .expect("PORT env variable value is not an integer");

            let domain = env::var("DOMAIN").expect("DOMAIN env variable is not set");
            let url: Url = match format!("https://{domain}/webhook/main").parse() {
                Ok(url) => url,
                Err(err) => panic!("Failed to parse URL: {}", err),
            };

            log::info!("Webhook!");
            log::info!("Port: {}", port.clone().to_string());
            log::info!("URL: {}", url.clone().to_string());

            let addr = ([0, 0, 0, 0], port).into();
            let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
                .await
                .expect("Couldn't setup webhook");

            Dispatcher::builder(bot, swastika_handler)
                .enable_ctrlc_handler()
                .build()
                .dispatch_with_listener(listener, LoggingErrorHandler::new())
                .await
        }
    }
}
