use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex genérica para localizar URLs em mensagens.
    ///
    /// Usada pelo módulo links.rs.
    pub static ref URL_REGEX: Regex = Regex::new(
        r"(https?://[^\s]+)"
    )
    .expect("Invalid URL regex");


    /// Regex para normalização de espaços.
    ///
    /// Ajuda na limpeza de mensagens antes da análise.
    pub static ref MULTIPLE_SPACES_REGEX: Regex = Regex::new(
        r"\s+"
    )
    .expect("Invalid whitespace regex");


    /// Regex para remover caracteres especiais
    /// mantendo letras, números e espaços.
    ///
    /// Útil para reduzir tentativas simples
    /// de burlar filtros:
    /// p.o.r.n -> porn
    pub static ref SPECIAL_CHARS_REGEX: Regex = Regex::new(
        r"[^a-zA-Z0-9À-ÿ\s]"
    )
    .expect("Invalid special chars regex");
}

/// Normaliza texto antes da análise.
///
/// Operações:
/// - converte para lowercase
/// - remove caracteres especiais
/// - reduz espaços duplicados
pub fn normalize_text(text: &str) -> String {
    let lower = text.to_lowercase();

    let cleaned = SPECIAL_CHARS_REGEX.replace_all(&lower, "");

    MULTIPLE_SPACES_REGEX
        .replace_all(&cleaned, " ")
        .trim()
        .to_string()
}
