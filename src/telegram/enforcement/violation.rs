use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::Lang,
    moderation::{
        engine::ViolationType,
        rules::StrikesConfig,
        strikes::{StrikeAction, resolve_action},
    },
    storage::sqlite,
    telegram::enforcement::{punishment, trust},
};

/// Ponto de entrada do fluxo pós-detecção: chamado por
/// `handlers::message_handler` assim que `analyze_message` encontra
/// uma violação. Decide entre ban direto (zero tolerância) e a
/// escada de strikes (aviso → mute → kick → ban), persistindo a
/// violação em qualquer um dos dois casos.
pub async fn handle(
    bot: &Bot,
    msg: &Message,
    user: &User,
    lang: Lang,
    state: &AppState,
    violation: ViolationType,
) -> ResponseResult<()> {
    if violation.is_zero_tolerance() {
        record_violation(state, msg, user, violation).await;

        punishment::ban(bot, msg, user, lang).await
    } else {
        // Resolve a config de strikes (com o multiplicador de
        // confiança, se aplicável) ANTES de registrar esta violação.
        // Se checássemos depois, a violação em curso já apareceria
        // no histórico consultado pela whitelist, e um usuário com
        // `max_violations = 0` nunca seria considerado confiável.
        let strikes_config =
            trust::resolve_strikes_config(state, msg.chat.id.0, user.id.0 as i64).await;

        record_violation(state, msg, user, violation).await;

        handle_graduated_violation(bot, msg, user, lang, state, strikes_config).await
    }
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
    strikes_config: StrikesConfig,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

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
        StrikeAction::Warn => punishment::warn(bot, msg, user, lang, count).await,
        StrikeAction::Mute => punishment::mute(bot, msg, user, lang, &strikes_config).await,
        StrikeAction::Kick => punishment::kick(bot, msg, user, lang, &strikes_config).await,
        StrikeAction::Ban => punishment::ban(bot, msg, user, lang).await,
    }
}
