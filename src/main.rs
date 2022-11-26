use std::env;
use std::fmt::Debug;

use teloxide::dispatching::update_listeners::webhooks;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::requests::{Request, Requester, ResponseResult};
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText, Update,
};
use teloxide::{dptree, respond, Bot};
use url::Url;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PollingMode {
    Polling,
    Webhook,
}

async fn answer(bot: Bot, q: InlineQuery) -> ResponseResult<()> {
    let result = InlineQueryResultArticle::new(
        "01".to_string(),
        "oof?",
        InputMessageContent::Text(InputMessageContentText::new("boof")),
    );
    let results = vec![InlineQueryResult::Article(result)];

    let response = bot.answer_inline_query(&q.id, results).send().await;
    if let Err(err) = response {
        log::error!("Error in handler: {:?}", err);
    }
    respond(())
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

    let handler = Update::filter_inline_query().branch(dptree::endpoint(answer));

    match polling_mode {
        PollingMode::Polling => {
            Dispatcher::builder(bot, handler)
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

            teloxide::repl_with_listener(bot, answer, listener).await

            // Dispatcher::builder(bot, handler)
            //     .enable_ctrlc_handler()
            //     .build()
            //     .dispatch_with_listener(listener, LoggingErrorHandler::new())
            //     .await
        }
    }
}
