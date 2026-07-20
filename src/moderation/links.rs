use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    /// Regex responsável por localizar URLs em mensagens.
    static ref URL_REGEX: Regex = Regex::new(
        r"(https?://[^\s]+)"
    )
    .unwrap();
}

/// Verifica se uma mensagem contém links suspeitos.
///
/// `domains` vem de `state.moderation.links.domains` (config/moderation.toml).
pub fn is_suspicious_link(text: &str, domains: &[String]) -> bool {
    if text.is_empty() {
        return false;
    }

    for capture in URL_REGEX.captures_iter(text) {
        let url = match Url::parse(&capture[1]) {
            Ok(url) => url,
            Err(_) => continue,
        };

        let host = url.host_str().unwrap_or("").to_lowercase();

        if domains
            .iter()
            .any(|domain| host.contains(&domain.to_lowercase()))
        {
            return true;
        }
    }

    false
}
