use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CSAM_PATTERNS: Vec<Regex> = vec![
        // Português
        Regex::new(
            r"(?i)(pedofilia|pedófilo|pedofilo|abuso infantil|exploração infantil)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(criança nua|crianca nua|menor nua|menor de idade|menina nua|menino nu)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(cp|conteúdo infantil|conteudo infantil|material infantil)"
        )
        .unwrap(),


        // Inglês
        Regex::new(
            r"(?i)(pedophilia|pedophile|child abuse|child exploitation)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(child porn|child pornography|underage|minor nude|nude minor)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(csam|child sexual abuse material)"
        )
        .unwrap(),


        // Espanhol
        Regex::new(
            r"(?i)(pedofilia|pedófilo|pedofilo|abuso infantil|explotación infantil)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(menor desnudo|niño desnudo|niña desnuda|contenido infantil)"
        )
        .unwrap(),
    ];
}

/// Detecta termos relacionados a exploração sexual infantil.
///
/// Este módulo possui prioridade máxima no mecanismo
/// de moderação.
///
/// Retorna `true` quando encontra padrões associados a CSAM.
pub fn is_csam(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    CSAM_PATTERNS.iter().any(|pattern| pattern.is_match(text))
}
