use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::{Lang, LanguageManager, messages},
    telegram::admin::is_chat_admin,
};

/// `/language <pt|en|es>` — altera o idioma do grupo. Apenas administradores.
pub async fn handle(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    lang: Lang,
    argument: &str,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(user) = &msg.from else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, user.id).await {
        bot.send_message(chat_id, messages::lang_no_permission(lang))
            .await?;

        return Ok(());
    }

    match Lang::from_code(argument) {
        Some(new_lang) => {
            LanguageManager::set(state, chat_id, new_lang).await;

            bot.send_message(chat_id, messages::lang_set(new_lang))
                .await?;
        }

        None => {
            bot.send_message(chat_id, messages::lang_invalid(lang))
                .await?;
        }
    }

    Ok(())
}
