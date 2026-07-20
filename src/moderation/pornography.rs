/// Detecta conteúdo pornográfico ou sexual explícito.
///
/// `keywords` vem de `state.moderation.pornography.keywords`
/// (config/moderation.toml).
///
/// Retorna `true` quando encontra termos associados a:
/// - pornografia adulta
/// - conteúdo sexual explícito
/// - divulgação de material adulto
pub fn is_pornography(text: &str, keywords: &[String]) -> bool {
    if text.is_empty() {
        return false;
    }

    let normalized = text.to_lowercase();

    keywords
        .iter()
        .any(|keyword| normalized.contains(&keyword.to_lowercase()))
}
