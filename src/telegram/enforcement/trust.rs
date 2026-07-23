use crate::{core::state::AppState, moderation::rules::StrikesConfig, storage::sqlite};

/// Resolve a `StrikesConfig` efetiva para este usuário: se a whitelist
/// de confiança estiver ativa e o usuário for elegível (membro antigo
/// e sem histórico de violação, ver `TrustConfig`), os limiares
/// `mute_at`/`kick_at`/`ban_at` vêm multiplicados por
/// `trust.strikes_multiplier`. Caso contrário, retorna a config padrão.
///
/// **Precisa ser chamada antes de persistir a violação em curso** —
/// senão ela já apareceria no histórico consultado pela whitelist, e
/// um usuário com `max_violations = 0` nunca seria considerado
/// confiável.
pub async fn resolve_strikes_config(state: &AppState, chat_id: i64, user_id: i64) -> StrikesConfig {
    let (strikes_config, trust_config) = {
        let rules = state.moderation.read().await;

        (rules.strikes.clone(), rules.trust.clone())
    };

    if !trust_config.enabled {
        return strikes_config;
    }

    match sqlite::is_trusted_user(
        &state.db,
        chat_id,
        user_id,
        trust_config.min_days_in_group,
        trust_config.max_violations,
    )
    .await
    {
        Ok(true) => {
            log::info!(
                "Usuário {} é confiável (whitelist) — limiares de strikes multiplicados por {}",
                user_id,
                trust_config.strikes_multiplier
            );

            strikes_config.scaled(trust_config.strikes_multiplier)
        }

        Ok(false) => strikes_config,

        Err(error) => {
            log::warn!(
                "Falha ao checar whitelist de confiança (chat {}, user {}): {} — tratando como não confiável",
                chat_id,
                user_id,
                error
            );

            strikes_config
        }
    }
}
