use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::{Lang, messages},
    telegram::admin::is_chat_admin,
};

/// `/blockdomain <dominio>` — bloqueia um domínio em runtime. Apenas
/// administradores. Persiste em `blocked_domains` (SQLite) e atualiza
/// a lista em memória usada pelo motor de moderação imediatamente —
/// sem precisar editar o TOML nem reiniciar o bot.
pub async fn handle(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    lang: Lang,
    argument: &str,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(admin) = &msg.from else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, admin.id).await {
        bot.send_message(chat_id, messages::blockdomain_no_permission(lang))
            .await?;

        return Ok(());
    }

    if argument.is_empty() {
        bot.send_message(chat_id, messages::blockdomain_invalid(lang))
            .await?;

        return Ok(());
    }

    match state.add_blocked_domain(argument).await {
        Ok(_) => {
            bot.send_message(chat_id, messages::blockdomain_success(lang, argument))
                .await?;

            log::info!(
                "Domínio '{}' bloqueado por {} no chat {}",
                argument,
                admin.id,
                chat_id
            );
        }

        Err(error) => {
            log::warn!("Falha ao bloquear domínio '{}': {}", argument, error);

            bot.send_message(chat_id, messages::blockdomain_error(lang))
                .await?;
        }
    }

    Ok(())
}
