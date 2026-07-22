use chrono::{Duration as ChronoDuration, Utc};
use teloxide::{
    prelude::*,
    types::{ChatPermissions, ParseMode, User, UserId},
};

use crate::{
    core::state::AppState,
    i18n::{Lang, LanguageManager, messages},
    moderation::{
        engine::{ViolationType, analyze_message},
        rules::StrikesConfig,
        strikes::{StrikeAction, resolve_action},
    },
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

    let lang = LanguageManager::get(&state, chat_id).await;
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
            handle_language_command(&bot, &msg, &state, lang, language.trim()).await?;
        }

        Command::Reload => {
            handle_reload_command(&bot, &msg, &state, lang).await?;
        }

        Command::Unban(argument) => {
            handle_unban_command(&bot, &msg, lang, argument.trim()).await?;
        }

        Command::BlockDomain(argument) => {
            handle_blockdomain_command(&bot, &msg, &state, lang, argument.trim()).await?;
        }
    }

    Ok(())
}

/// Processa alteração de idioma do grupo
async fn handle_language_command(
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

/// Bloqueia um domínio em runtime via /blockdomain <dominio>.
/// Apenas administradores. Persiste em `blocked_domains` (SQLite)
/// e atualiza a lista em memória usada pelo motor de moderação
/// imediatamente — sem precisar editar o TOML nem reiniciar o bot.
async fn handle_blockdomain_command(
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
        text.push_str(&format!(
            "• {}: {count}\n",
            escape_markdown_v2(violation_type)
        ));
    }

    if !stats.top_offenders.is_empty() {
        text.push_str(&format!("\n*{}:*\n", labels.top));

        for (user_id, username, count) in &stats.top_offenders {
            match username {
                Some(name) => {
                    text.push_str(&format!("• @{} — {count}\n", escape_markdown_v2(name)));
                }

                None => {
                    text.push_str(&format!("• `{user_id}` — {count}\n"));
                }
            }
        }
    }

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// Escapa caracteres reservados do Telegram MarkdownV2 (username,
/// violation_type e qualquer outro texto dinâmico) para não quebrar
/// a formatação — ou o envio — da mensagem.
fn escape_markdown_v2(text: &str) -> String {
    const RESERVED: &[char] = &[
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    let mut escaped = String::with_capacity(text.len());

    for ch in text.chars() {
        if RESERVED.contains(&ch) {
            escaped.push('\\');
        }

        escaped.push(ch);
    }

    escaped
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
        let lang = LanguageManager::get(&state, msg.chat.id).await;

        record_violation(&state, &msg, user, violation).await;

        if violation.is_zero_tolerance() {
            handle_ban(&bot, &msg, user, lang).await?;
        } else {
            handle_graduated_violation(&bot, &msg, user, lang, &state).await?;
        }
    }

    Ok(())
}

/// Persiste a violação detectada no banco (e o usuário que a cometeu,
/// para permitir exibir @username em /stats), usada depois pelo /stats
/// e pela contagem de strikes.
async fn record_violation(state: &AppState, msg: &Message, user: &User, violation: ViolationType) {
    let chat_id = msg.chat.id.0;

    let user_id = user.id.0 as i64;

    let message_text = msg.text().or_else(|| msg.caption());

    if let Err(error) = sqlite::upsert_user(&state.db, user_id, user.username.as_deref()).await {
        log::warn!(
            "Falha ao registrar usuário no banco (user {}): {}",
            user_id,
            error
        );
    }

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

/// Decide e aplica a punição para violações de baixa severidade
/// (gambling, spam), com base no número de violações recentes do
/// usuário nesse chat (aviso → mute → kick → ban).
///
/// A contagem vem do SQLite (tabela `violations`, já persistida
/// por `record_violation`), então sobrevive a reinícios do bot.
async fn handle_graduated_violation(
    bot: &Bot,
    msg: &Message,
    user: &User,
    lang: Lang,
    state: &AppState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let strikes_config = state.moderation.read().await.strikes.clone();

    let count = match sqlite::count_recent_violations(
        &state.db,
        chat_id.0,
        user.id.0 as i64,
        strikes_config.window_days,
    )
    .await
    {
        Ok(count) => count,

        Err(error) => {
            log::warn!(
                "Falha ao contar violações recentes (chat {}, user {}): {} — tratando como 1ª violação",
                chat_id,
                user.id,
                error
            );

            // Falha ao consultar o histórico não deve escalar a punição
            // por engano: na dúvida, trata como primeira violação (aviso).
            1
        }
    };

    match resolve_action(count, &strikes_config) {
        StrikeAction::Warn => handle_warn(bot, msg, user, lang, count).await,
        StrikeAction::Mute => handle_mute(bot, msg, user, lang, &strikes_config).await,
        StrikeAction::Kick => handle_kick(bot, msg, user, lang, &strikes_config).await,
        StrikeAction::Ban => handle_ban(bot, msg, user, lang).await,
    }
}

/// 1ª violação de baixa severidade: remove a mensagem e avisa o
/// usuário, sem restringir sua participação no grupo.
async fn handle_warn(
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
async fn handle_mute(
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
async fn handle_kick(
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

            log::info!("Usuário {} removido do grupo (kick) por violações repetidas", user.id);
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
async fn handle_ban(bot: &Bot, msg: &Message, user: &User, lang: Lang) -> ResponseResult<()> {
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
