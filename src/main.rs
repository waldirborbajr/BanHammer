mod core;
mod i18n;
mod moderation;
mod storage;
mod telegram;

use teloxide::prelude::*;

use crate::i18n::lang::DEFAULT_LANG;

#[tokio::main]
async fn main() {
    core::logger::init();

    core::logger::startup(*DEFAULT_LANG);

    let bot = Bot::from_env();

    telegram::dispatcher::run(bot).await;
}