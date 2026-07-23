use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::{Lang, messages},
    telegram::admin::is_chat_admin,
};

/// `/reload` — recarrega `config/moderation.toml` em runtime,
/// sem reiniciar o processo. Apenas administradores.
pub async fn handle(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    lang: Lang,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(user) = &msg.from else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, user.id).await {
        bot.send_message(chat_id, messages::reload_no_permission(lang))
            .await?;

        return Ok(());
    }

    match state.reload_moderation().await {
        Ok(_) => {
            bot.send_message(chat_id, messages::reload_success(lang))
                .await?;

            log::info!(
                "moderation.toml recarregado por {} no chat {}",
                user.id,
                chat_id
            );
        }

        Err(error) => {
            log::warn!("Falha ao recarregar moderation.toml: {}", error);

            bot.send_message(chat_id, messages::reload_error(lang))
                .await?;
        }
    }

    Ok(())
}
