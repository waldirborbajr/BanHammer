/// Detecta padrões comuns de spam e golpes financeiros.
///
/// `keywords` vem de `state.moderation.spam.keywords`
/// (config/moderation.toml).
///
/// Retorna `true` quando a mensagem contém termos
/// associados a spam, fraude ou publicidade abusiva.
pub fn is_spam(text: &str, keywords: &[String]) -> bool {
    if text.is_empty() {
        return false;
    }

    let normalized = text.to_lowercase();

    keywords
        .iter()
        .any(|keyword| normalized.contains(&keyword.to_lowercase()))
}
