use teloxide::{prelude::*, types::UserId};

use crate::{
    i18n::{Lang, messages},
    telegram::admin::is_chat_admin,
};

/// `/unban <user_id>` — remove o banimento de um usuário. Apenas
/// administradores. Requer o ID numérico do usuário (não é possível
/// resolver @username de alguém já banido via API do Telegram de
/// forma confiável).
pub async fn handle(bot: &Bot, msg: &Message, lang: Lang, argument: &str) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(admin) = &msg.from else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, admin.id).await {
        bot.send_message(chat_id, messages::unban_no_permission(lang))
            .await?;

        return Ok(());
    }

    let Ok(raw_id) = argument.parse::<u64>() else {
        bot.send_message(chat_id, messages::unban_invalid_id(lang))
            .await?;

        return Ok(());
    };

    let target = UserId(raw_id);

    match bot.unban_chat_member(chat_id, target).await {
        Ok(_) => {
            bot.send_message(chat_id, messages::unban_success(lang, raw_id))
                .await?;

            log::info!(
                "Usuário {} desbanido por {} no chat {}",
                raw_id,
                admin.id,
                chat_id
            );
        }

        Err(error) => {
            log::warn!("Falha ao desbanir {}: {}", raw_id, error);

            bot.send_message(chat_id, messages::unban_error(lang))
                .await?;
        }
    }

    Ok(())
}
