use std::env;
use std::fmt::Debug;

use teloxide::dispatching::update_listeners::webhooks;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::requests::{Request, Requester};
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText, Update,
};
use teloxide::{dptree, respond, Bot};

use crate::swastikas::SWASTIKAS;

mod swastikas;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PollingMode {
    Polling,
    Webhook,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::from_env();

    let swastika_handler = |bot: Bot, q: InlineQuery| async move {
        let today_at_midnight = match chrono::Utc::now()
            .date_naive()
            .and_hms_milli_opt(0, 0, 0, 0)
        {
            Some(t) => t,
            None => {
                log::error!("Failed to get today at midnight");
                panic!("Failed to get today at midnight");
            }
        };
        let timestamp = today_at_midnight.timestamp() as u64;
        let user_id = q.from.id.0;
        let random_index = ((user_id + timestamp) as usize) % SWASTIKAS.len();
        let swastika_text = match SWASTIKAS.get(random_index) {
            Some(swastika) => swastika.to_string(),
            None => panic!("No swastika found for index {}", random_index),
        };

        let swastika_result = InlineQueryResultArticle::new(
            "01".to_string(),
            "Какая ты сегодня свастика?",
            InputMessageContent::Text(InputMessageContentText::new(swastika_text)),
        );
        let results = vec![InlineQueryResult::Article(swastika_result)];

        let response = bot.answer_inline_query(&q.id, results).send().await;
        if let Err(err) = response {
            log::error!("Error in handler: {:?}", err);
        }
        respond(())
    };

    let handler = Update::filter_inline_query().branch(dptree::endpoint(swastika_handler));

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
            log::info!("Starting polling...");
            Dispatcher::builder(bot, handler)
                .enable_ctrlc_handler()
                .build()
                .dispatch()
                .await;
        }
        PollingMode::Webhook => {
            let port: u16 = env::var("PORT")
                .expect("PORT env variable is not set")
                .parse()
                .expect("PORT env variable value is not an integer");

            let addr = ([0, 0, 0, 0], port).into();

            let host = env::var("DOMAIN").expect("DOMAIN env variable is not set");
            let url = match format!("https://{host}/webhook").parse() {
                Ok(url) => url,
                Err(err) => panic!("Failed to parse URL: {}", err),
            };

            let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
                .await
                .expect("Couldn't setup webhook");

            log::info!("Starting webhook at:");
            log::info!("Port: {}", port);
            log::info!("URL: {}", url);
            teloxide::repl_with_listener(bot, swastika_handler, listener).await;
        }
    }
}
