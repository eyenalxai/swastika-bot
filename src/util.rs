use teloxide::types::User;

use crate::swastikas::SWASTIKAS;

pub(crate) fn random_swastika(user: User) -> String {
    let today_timestamp = chrono::Utc::now()
        .date_naive()
        .and_hms_milli_opt(0, 0, 0, 0)
        .unwrap()
        .timestamp() as u64;
    let user_id = user.id.0;
    let random_index = ((user_id + today_timestamp) as usize) % SWASTIKAS.len();
    match SWASTIKAS.get(random_index) {
        Some(swastika) => swastika.to_string(),
        None => panic!("No swastika found for index {}", random_index),
    }
}
