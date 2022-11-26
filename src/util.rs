use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::requests::{Requester, ResponseResult};
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText,
};
use teloxide::{respond, Bot};

use crate::swastikas::SWASTIKAS;

#[derive(Debug, Clone, Copy)]
pub(crate) enum PollingMode {
    Polling,
    Webhook,
}

pub(crate) fn get_random_swastika(user_id: u64) -> String {
    let today_timestamp = match chrono::Utc::now()
        .date_naive()
        .and_hms_milli_opt(0, 0, 0, 0)
    {
        Some(t) => t.timestamp() as u64,
        None => {
            log::error!("Failed to get today at midnight");
            panic!("Failed to get today at midnight");
        }
    };

    let mut rng = Pcg32::seed_from_u64(user_id + today_timestamp);

    match SWASTIKAS.choose(&mut rng) {
        Some(s) => s.to_string(),
        None => panic!("Failed to get swastika"),
    }
}

pub(crate) async fn swastika_answer(bot: Bot, q: InlineQuery) -> ResponseResult<()> {
    let swastika_text = get_random_swastika(q.from.id.0);

    let swastika_result = InlineQueryResultArticle::new(
        "0".to_string(),
        "Какая ты сегодня свастика?",
        InputMessageContent::Text(InputMessageContentText::new(swastika_text)),
    )
    .description("Из стихотворения Ярослава Могутина");

    let results = vec![InlineQueryResult::Article(swastika_result)];

    let response = bot
        .answer_inline_query(&q.id, results)
        .is_personal(true)
        .await;
    if let Err(err) = response {
        log::error!("Error in handler: {:?}", err);
    }
    respond(())
}
