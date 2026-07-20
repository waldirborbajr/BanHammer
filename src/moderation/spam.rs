use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPAM_PATTERNS: Vec<Regex> = vec![
        // Português
        Regex::new(
            r"(?i)(dinheiro fácil|renda extra|ganhe em casa|ganhe dinheiro|trabalho online|lucro garantido)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(investimento seguro|retorno garantido|multiplique seu dinheiro|renda passiva)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(pirâmide|esquema|mlm|marketing multinível|afiliado fácil)"
        )
        .unwrap(),

        // Inglês
        Regex::new(
            r"(?i)(make money|easy money|get rich|work from home|guaranteed profit)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(investment opportunity|passive income|double your money|risk free)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(pyramid scheme|mlm|affiliate program|earn fast)"
        )
        .unwrap(),
    ];
}

/// Detecta padrões comuns de spam e golpes financeiros.
///
/// Retorna `true` quando a mensagem contém termos
/// associados a spam, fraude ou publicidade abusiva.
pub fn is_spam(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    SPAM_PATTERNS.iter().any(|pattern| pattern.is_match(text))
}
