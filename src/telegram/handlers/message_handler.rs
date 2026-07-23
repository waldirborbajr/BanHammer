use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::LanguageManager,
    moderation::engine::analyze_message,
    telegram::{enforcement::violation, events::TelegramEvent},
};

/// Handler de mensagens normais — detecta violações e delega a
/// decisão de punição pra `enforcement::violation`.
pub async fn message_handler(bot: Bot, msg: Message, state: AppState) -> ResponseResult<()> {
    let Some(user) = &msg.from else {
        return Ok(());
    };

    let event = TelegramEvent::from_message(&msg);

    let Some(content) = event.content() else {
        return Ok(());
    };

    if let Some(detected) = analyze_message(content, &event, &state).await {
        let lang = LanguageManager::get(&state, msg.chat.id).await;

        violation::handle(&bot, &msg, user, lang, &state, detected).await?;
    }

    Ok(())
}
