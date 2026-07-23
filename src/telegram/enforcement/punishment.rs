use chrono::{Duration as ChronoDuration, Utc};
use teloxide::{prelude::*, types::ChatPermissions, types::User};

use crate::{
    i18n::{Lang, messages},
    moderation::rules::StrikesConfig,
};

/// 1ª violação de baixa severidade: remove a mensagem e avisa o
/// usuário, sem restringir sua participação no grupo.
pub async fn warn(
    bot: &Bot,
    msg: &Message,
    user: &User,
    lang: Lang,
    count: i64,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    bot.delete_message(chat_id, msg.id).await.ok();

    let username = user.username.as_deref().unwrap_or("user");

    bot.send_message(chat_id, messages::warned(lang, username, count))
        .await
        .ok();

    log::info!(
        "Usuário {} avisado (violação {} na janela configurada)",
        user.id,
        count
    );

    Ok(())
}

/// Violação recorrente: remove a mensagem e silencia o usuário
/// por `mute_duration_minutes` (definido em moderation.toml).
pub async fn mute(
    bot: &Bot,
    msg: &Message,
    user: &User,
    lang: Lang,
    strikes_config: &StrikesConfig,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    bot.delete_message(chat_id, msg.id).await.ok();

    let until = Utc::now() + ChronoDuration::minutes(strikes_config.mute_duration_minutes);

    match bot
        .restrict_chat_member(chat_id, user.id, ChatPermissions::empty())
        .until_date(until)
        .await
    {
        Ok(_) => {
            let username = user.username.as_deref().unwrap_or("user");

            bot.send_message(
                chat_id,
                messages::muted(lang, username, strikes_config.mute_duration_minutes),
            )
            .await
            .ok();

            log::info!(
                "Usuário {} silenciado por {} minuto(s) por violações repetidas",
                user.id,
                strikes_config.mute_duration_minutes
            );
        }

        Err(error) => {
            log::warn!("Falha ao silenciar usuário {}: {}", user.id, error);

            bot.send_message(chat_id, messages::violation_generic(lang))
                .await
                .ok();
        }
    }

    Ok(())
}

/// Violação recorrente: remove a mensagem e remove o usuário do
/// grupo sem banimento permanente (o Telegram o deixa voltar
/// automaticamente após `kick_ban_seconds`).
pub async fn kick(
    bot: &Bot,
    msg: &Message,
    user: &User,
    lang: Lang,
    strikes_config: &StrikesConfig,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    bot.delete_message(chat_id, msg.id).await.ok();

    let until = Utc::now() + ChronoDuration::seconds(strikes_config.kick_ban_seconds);

    match bot.ban_chat_member(chat_id, user.id).until_date(until).await {
        Ok(_) => {
            let username = user.username.as_deref().unwrap_or("user");

            bot.send_message(chat_id, messages::kicked(lang, username))
                .await
                .ok();

            log::info!(
                "Usuário {} removido do grupo (kick) por violações repetidas",
                user.id
            );
        }

        Err(error) => {
            log::warn!("Falha ao remover (kick) usuário {}: {}", user.id, error);

            bot.send_message(chat_id, messages::violation_generic(lang))
                .await
                .ok();
        }
    }

    Ok(())
}

/// Remove conteúdo proibido e bane o usuário permanentemente.
/// Usado tanto para violações de zero tolerância (csam, pornografia,
/// link suspeito) quanto para o topo da escada de strikes.
pub async fn ban(bot: &Bot, msg: &Message, user: &User, lang: Lang) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    // Remove mensagem
    bot.delete_message(chat_id, msg.id).await.ok();

    // Banimento
    match bot.ban_chat_member(chat_id, user.id).await {
        Ok(_) => {
            let username = user.username.as_deref().unwrap_or("user");

            bot.send_message(chat_id, messages::banned(lang, username))
                .await
                .ok();

            log::info!("Usuário {} banido por conteúdo proibido", user.id);
        }

        Err(error) => {
            log::warn!("Falha ao banir usuário {}: {}", user.id, error);

            bot.send_message(chat_id, messages::violation_generic(lang))
                .await
                .ok();
        }
    }

    Ok(())
}
