mod core;
mod i18n;
mod moderation;
mod storage;
mod telegram;

use teloxide::prelude::*;

use crate::{
    core::{
        config::Config,
        state::AppState,
    },
    i18n::lang::DEFAULT_LANG,
};


#[tokio::main]
async fn main() {
    core::logger::init();

    core::logger::startup(*DEFAULT_LANG);


    let config = Config::load();


    let state = AppState::new(config)
        .await
        .expect("Failed to initialize application state");


    let bot = Bot::from_env();


    telegram::dispatcher::run(
        bot,
        state,
    )
    .await;
}