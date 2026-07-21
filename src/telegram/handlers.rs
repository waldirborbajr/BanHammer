use teloxide::{
    prelude::*,
    types::{ParseMode, User, UserId},
};

use crate::{
    core::state::AppState,
    i18n::{
        lang::{Lang, lang_for_chat, set_lang_for_chat},
        messages,
    },
    moderation::engine::{ViolationType, analyze_message},
    storage::sqlite,
    telegram::{admin::is_chat_admin, events::TelegramEvent},
};

use super::commands::Command;

/// Handler principal dos comandos Telegram
pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: AppState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let lang = lang_for_chat(chat_id);

    match cmd {
        Command::Help => {
            bot.send_message(chat_id, messages::help(lang)).await?;
        }

        Command::Status => {
            bot.send_message(chat_id, messages::status(lang)).await?;
        }

        Command::Stats => {
            handle_stats_command(&bot, &msg, &state, lang).await?;
        }

        Command::Language(language) => {
            handle_language_command(&bot, &msg, lang, language.trim()).await?;
        }

        Command::Reload => {
            handle_reload_command(&bot, &msg, &state, lang).await?;
        }

        Command::Unban(argument) => {
            handle_unban_command(&bot, &msg, lang, argument.trim()).await?;
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

/// Recarrega config/moderation.toml em runtime,
/// sem reiniciar o processo. Apenas administradores.
async fn handle_reload_command(
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

/// Remove o banimento de um usuário via /unban <user_id>.
/// Apenas administradores. Requer o ID numérico do usuário
/// (não é possível resolver @username de alguém já banido
/// via API do Telegram de forma confiável).
async fn handle_unban_command(
    bot: &Bot,
    msg: &Message,
    lang: Lang,
    argument: &str,
) -> ResponseResult<()> {
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

/// Monta e envia a mensagem de estatísticas do grupo
async fn handle_stats_command(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    lang: Lang,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let labels = messages::stats_labels(lang);

    let stats = match sqlite::get_chat_stats(&state.db, chat_id.0).await {
        Ok(stats) => stats,

        Err(error) => {
            log::warn!(
                "Falha ao buscar estatísticas do chat {}: {}",
                chat_id,
                error
            );

            bot.send_message(chat_id, messages::violation_generic(lang))
                .await
                .ok();

            return Ok(());
        }
    };

    if stats.total == 0 {
        bot.send_message(chat_id, labels.empty).await?;

        return Ok(());
    }

    let mut text = format!(
        "{}\n\n{}: {}\n{}: {}\n\n*{}:*\n",
        labels.title, labels.total, stats.total, labels.last_24h, stats.last_24h, labels.by_type,
    );

    for (violation_type, count) in &stats.by_type {
        text.push_str(&format!("• {violation_type}: {count}\n"));
    }

    if !stats.top_offenders.is_empty() {
        text.push_str(&format!("\n*{}:*\n", labels.top));

        for (user_id, count) in &stats.top_offenders {
            text.push_str(&format!("• `{user_id}` — {count}\n"));
        }
    }

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// Handler de mensagens normais
pub async fn message_handler(bot: Bot, msg: Message, state: AppState) -> ResponseResult<()> {
    let Some(user) = &msg.from else {
        return Ok(());
    };

    let event = TelegramEvent::from_message(&msg);

    let Some(content) = event.content() else {
        return Ok(());
    };

    if let Some(violation) = analyze_message(content, &event, &state).await {
        let lang = lang_for_chat(msg.chat.id);

        record_violation(&state, &msg, user, violation).await;

        handle_violation(&bot, &msg, user, lang).await?;
    }

    Ok(())
}

/// Persiste a violação detectada no banco,
/// usada depois pelo /stats
async fn record_violation(state: &AppState, msg: &Message, user: &User, violation: ViolationType) {
    let chat_id = msg.chat.id.0;

    let user_id = user.id.0 as i64;

    let message_text = msg.text().or_else(|| msg.caption());

    if let Err(error) = sqlite::insert_violation(
        &state.db,
        chat_id,
        user_id,
        violation.as_str(),
        message_text,
    )
    .await
    {
        log::warn!(
            "Falha ao registrar violação no banco (chat {}, user {}): {}",
            chat_id,
            user_id,
            error
        );
    }
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
