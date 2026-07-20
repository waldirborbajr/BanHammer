mod core;
mod i18n;
mod moderation;
mod storage;
mod telegram;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    core::logger::init();

    log::info!("🚀 Iniciando BanHammer...");

    let bot = Bot::from_env();

    telegram::dispatcher::run(bot).await;
}