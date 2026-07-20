use teloxide::{prelude::*, types::User};

use crate::{
    i18n::{
        lang::{Lang, lang_for_chat, set_lang_for_chat},
        messages,
    },
    moderation::engine::is_violation,
    telegram::admin::is_chat_admin,
};

use super::commands::Command;

/// Handler principal dos comandos Telegram
pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let lang = lang_for_chat(chat_id);

    match cmd {
        Command::Help => {
            bot.send_message(chat_id, messages::help(lang)).await?;
        }

        Command::Status => {
            bot.send_message(chat_id, messages::status(lang)).await?;
        }

        Command::Language(language) => {
            handle_language_command(&bot, &msg, lang, language.trim()).await?;
        }
    }

    Ok(())
}

/// Processa alteração de idioma do grupo
async fn handle_language_command(
    bot: &Bot,
    msg: &Message,
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
            set_lang_for_chat(chat_id, new_lang);

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

/// Handler de mensagens normais
pub async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let user = match &msg.from {
        Some(user) => user,

        None => {
            return Ok(());
        }
    };

    let text = msg
        .text()
        .or_else(|| msg.caption())
        .map(|text| text.to_lowercase())
        .unwrap_or_default();

    if text.is_empty() {
        return Ok(());
    }

    if is_violation(&text, &msg) {
        let lang = lang_for_chat(chat_id);

        handle_violation(&bot, &msg, user, lang).await?;
    }

    Ok(())
}

/// Remove conteúdo proibido e pune usuário
async fn handle_violation(bot: &Bot, msg: &Message, user: &User, lang: Lang) -> ResponseResult<()> {
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
