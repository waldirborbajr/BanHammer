use super::rules::StrikesConfig;

/// Ação a aplicar para uma violação de baixa severidade
/// (gambling, spam), de acordo com o número de violações
/// recentes do usuário no chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrikeAction {
    Warn,
    Mute,
    Kick,
    Ban,
}

/// Resolve a ação a partir da contagem de violações recentes
/// (`count`, já incluindo a violação atual) e dos limiares
/// configurados em `[strikes]` no `moderation.toml`.
///
/// `StrikesConfig::validate` garante `mute_at < kick_at < ban_at`
/// no carregamento da config, então a ordem de checagem abaixo
/// (do limiar mais alto para o mais baixo) é segura.
pub fn resolve_action(count: i64, config: &StrikesConfig) -> StrikeAction {
    if count >= config.ban_at as i64 {
        StrikeAction::Ban
    } else if count >= config.kick_at as i64 {
        StrikeAction::Kick
    } else if count >= config.mute_at as i64 {
        StrikeAction::Mute
    } else {
        StrikeAction::Warn
    }
}
