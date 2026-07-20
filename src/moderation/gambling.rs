use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GAMBLING_PATTERNS: Vec<Regex> = vec![
        // Português
        Regex::new(
            r"(?i)(aposta|aposte|apostar|cassino|casino|bet|bet365|slot|caça[- ]níquel|roleta)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(ganhe dinheiro|ganhe fácil|prêmio garantido|dinheiro rápido|lucro fácil)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(odd|odds|palpite|bolão|jogo do bicho|banca esportiva)"
        )
        .unwrap(),


        // Inglês
        Regex::new(
            r"(?i)(gambling|casino|betting|bet|slot|roulette|jackpot)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(sportsbet|sportbet|odds|place a bet|free bet|bonus bet)"
        )
        .unwrap(),


        // Espanhol
        Regex::new(
            r"(?i)(apuesta|apuestas|casino|tragamonedas|ruleta|premio)"
        )
        .unwrap(),

        Regex::new(
            r"(?i)(ganar dinero|dinero fácil|bono gratis|apuesta deportiva)"
        )
        .unwrap(),
    ];
}


/// Detecta conteúdo relacionado a apostas e jogos de azar.
///
/// Retorna `true` quando encontra termos associados a:
/// - cassinos
/// - apostas esportivas
/// - plataformas betting
/// - promoções de jogos de azar
pub fn is_gambling(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    GAMBLING_PATTERNS
        .iter()
        .any(|pattern| pattern.is_match(text))
}