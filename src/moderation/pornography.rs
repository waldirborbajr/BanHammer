use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PORNOGRAPHY_PATTERNS: Vec<Regex> = vec![
        // Português
        Regex::new(
            r"(?i)(porn|porno|pornografia|sexo|sexual|nudes|nude|nsfw|xxx)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(putaria|puta|piranha|boquete|anal|gozar|tesão|tesao|foder|foda)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(pelada|pelado|safada|safado|conteúdo adulto|conteudo adulto)"
        )
        .unwrap(),


        // Inglês
        Regex::new(
            r"(?i)(porn|pornography|sex|sexual|nudes|nude|nsfw|xxx)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(horny|fuck|fucking|slut|bitch|dick|pussy|cum|blowjob)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(adult content|explicit content|private video)"
        )
        .unwrap(),


        // Espanhol
        Regex::new(
            r"(?i)(porno|pornografía|pornografia|sexo|sexual|desnudo|desnuda)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(contenido adulto|contenido explícito|contenido explicito)"
        )
        .unwrap(),
    ];
}


/// Detecta conteúdo pornográfico ou sexual explícito.
///
/// Retorna `true` quando encontra padrões associados a:
/// - pornografia adulta
/// - conteúdo sexual explícito
/// - divulgação de material adulto
pub fn is_pornography(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    PORNOGRAPHY_PATTERNS
        .iter()
        .any(|pattern| pattern.is_match(text))
}