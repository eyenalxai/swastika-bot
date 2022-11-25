use std::env;

use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::requests::{Request, Requester};
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText, Update, User,
};
use teloxide::{dptree, respond, Bot};

use crate::swastikas::SWASTIKAS;
use crate::util::random_swastika;

mod swastikas;
mod util;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, q: InlineQuery| async move {
            let swastika_text = random_swastika(q.from);
            let swastika_result = InlineQueryResultArticle::new(
                "01".to_string(),
                "Swastika",
                InputMessageContent::Text(InputMessageContentText::new(swastika_text)),
            );
            let results = vec![InlineQueryResult::Article(swastika_result)];

            let response = bot.answer_inline_query(&q.id, results).send().await;
            if let Err(err) = response {
                log::error!("Error in handler: {:?}", err);
            }
            respond(())
        },
    ));

    // let port: u16 = env::var("PORT")
    //     .expect("PORT env variable is not set")
    //     .parse()
    //     .expect("PORT env variable value is not an integer");
    //
    // let addr = ([0, 0, 0, 0], port).into();
    //
    // let host = env::var("DOMAIN").expect("DOMAIN env variable is not set");
    // let url = format!("https://{host}/webhook").parse().unwrap();
    //
    // let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
    //     .await
    //     .expect("Couldn't setup webhook");
    //
    // teloxide::repl_with_listener(bot, handler, listener).await;

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
