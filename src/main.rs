mod core;
mod i18n;
mod moderation;
mod storage;
mod telegram;

use teloxide::prelude::*;

use crate::{
    core::{config::Config, state::AppState},
    i18n::Lang,
};

#[tokio::main]
async fn main() {
    core::logger::init();

    let config = Config::load();

    let default_lang = Lang::from_code(&config.default_language).unwrap_or(Lang::Pt);

    core::logger::startup(default_lang);

    let state = AppState::new(config)
        .await
        .expect("Failed to initialize application state");

    let bot = Bot::from_env();

    telegram::dispatcher::run(bot, state).await;
}
