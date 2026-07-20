/// Detecta conteúdo relacionado a apostas e jogos de azar.
///
/// `keywords` vem de `state.moderation.gambling.keywords`
/// (config/moderation.toml).
///
/// Retorna `true` quando encontra termos associados a:
/// - cassinos
/// - apostas esportivas
/// - plataformas betting
/// - promoções de jogos de azar
pub fn is_gambling(text: &str, keywords: &[String]) -> bool {
    if text.is_empty() {
        return false;
    }

    let normalized = text.to_lowercase();

    keywords
        .iter()
        .any(|keyword| normalized.contains(&keyword.to_lowercase()))
}
