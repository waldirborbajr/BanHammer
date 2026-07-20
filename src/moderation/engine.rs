use crate::{
    core::state::AppState,
    telegram::events::TelegramEvent,
};

use super::{
    csam,
    gambling,
    links,
    pornography,
    regex::normalize_text,
    spam,
};



/// Resultado da análise de moderação.
///
/// A severidade define prioridade
/// para futuras ações diferentes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {

    Csam,

    Pornography,

    Gambling,

    Spam,

    SuspiciousLink,
}



impl ViolationType {

    pub fn severity(
        &self,
    ) -> u8 {

        match self {

            ViolationType::Csam =>
                5,

            ViolationType::Pornography =>
                4,

            ViolationType::Gambling =>
                3,

            ViolationType::SuspiciousLink =>
                2,

            ViolationType::Spam =>
                1,
        }
    }

    /// Nome estável usado para persistência (coluna violation_type).
    pub fn as_str(&self) -> &'static str {

        match self {

            ViolationType::Csam =>
                "csam",

            ViolationType::Pornography =>
                "pornography",

            ViolationType::Gambling =>
                "gambling",

            ViolationType::SuspiciousLink =>
                "suspicious_link",

            ViolationType::Spam =>
                "spam",
        }
    }
}






/// Analisa uma mensagem usando:
///
/// - Detectores internos
/// - Regras externas TOML
/// - Tipo de evento Telegram
///
pub fn analyze_message(
    text: &str,
    event: &TelegramEvent,
    state: &AppState,
) -> Option<ViolationType> {


    if text.is_empty() {
        return None;
    }



    let normalized =
        normalize_text(text);



    //
    // Prioridade máxima
    //
    if csam::is_csam(
        &normalized,
    ) {

        return Some(
            ViolationType::Csam
        );
    }




    if pornography::is_pornography(
        &normalized,
        &state.moderation.pornography.keywords,
    ) {

        return Some(
            ViolationType::Pornography
        );
    }




    if gambling::is_gambling(
        &normalized,
        &state.moderation.gambling.keywords,
    ) {

        return Some(
            ViolationType::Gambling
        );
    }




    if links::is_suspicious_link(
        &normalized,
        &state.moderation.links.domains,
    ) {

        return Some(
            ViolationType::SuspiciousLink
        );
    }




    if spam::is_spam(
        &normalized,
        &state.moderation.spam.keywords,
    ) {

        return Some(
            ViolationType::Spam
        );
    }




    //
    // Regras externas TOML (apenas csam continua checado aqui)
    //
    if matches_external_rules(
        &normalized,
        state,
    ) {

        return Some(
            ViolationType::Csam
        );
    }




    //
    // Encaminhamentos longos
    //
    if event.is_forwarded()
        && normalized.len() > 20
    {

        return Some(
            ViolationType::Spam
        );
    }



    None
}






/// Verifica palavras configuradas externamente
/// em moderation.toml (csam)
fn matches_external_rules(
    text: &str,
    state: &AppState,
) -> bool {


    let rules =
        &state.moderation;


    rules
        .csam
        .keywords
        .iter()
        .any(
            |keyword|
                text.contains(
                    &keyword.to_lowercase()
                )
        )
}






/// Apenas verifica se existe violação
pub fn is_violation(
    text: &str,
    event: &TelegramEvent,
    state: &AppState,
) -> bool {


    analyze_message(
        text,
        event,
        state,
    )
    .is_some()
}