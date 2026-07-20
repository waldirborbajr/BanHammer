use teloxide::types::Message;

use super::{
    csam,
    gambling,
    links,
    pornography,
    spam,
    regex::normalize_text,
};


/// Resultado da análise de moderação.
///
/// Permite futuramente diferenciar
/// ações conforme o tipo de violação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    Csam,
    Pornography,
    Gambling,
    Spam,
    SuspiciousLink,
}


impl ViolationType {
    pub fn severity(&self) -> u8 {
        match self {
            ViolationType::Csam => 5,
            ViolationType::Pornography => 4,
            ViolationType::Gambling => 3,
            ViolationType::SuspiciousLink => 2,
            ViolationType::Spam => 1,
        }
    }
}


/// Analisa uma mensagem e retorna o tipo de violação encontrado.
///
/// A ordem importa:
/// CSAM possui prioridade máxima.
pub fn analyze_message(
    text: &str,
    msg: &Message,
) -> Option<ViolationType> {
    if text.is_empty() {
        return None;
    }


    let normalized = normalize_text(text);


    // Prioridade máxima
    if csam::is_csam(&normalized) {
        return Some(ViolationType::Csam);
    }


    if pornography::is_pornography(&normalized) {
        return Some(ViolationType::Pornography);
    }


    if gambling::is_gambling(&normalized) {
        return Some(ViolationType::Gambling);
    }


    if links::is_suspicious_link(&normalized) {
        return Some(ViolationType::SuspiciousLink);
    }


    if spam::is_spam(&normalized) {
        return Some(ViolationType::Spam);
    }


    // Encaminhamentos longos são comuns em spam
    if msg.forward_origin().is_some()
        && normalized.len() > 20
    {
        return Some(ViolationType::Spam);
    }


    None
}


/// Atalho simples quando apenas precisamos saber
/// se uma mensagem é proibida.
pub fn is_violation(
    text: &str,
    msg: &Message,
) -> bool {
    analyze_message(text, msg).is_some()
}