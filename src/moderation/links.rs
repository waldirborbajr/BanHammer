use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    /// Regex responsável por localizar URLs em mensagens.
    static ref URL_REGEX: Regex = Regex::new(
        r"(https?://[^\s]+)"
    )
    .unwrap();


    /// Domínios considerados suspeitos.
    ///
    /// A lista pode futuramente ser carregada de banco
    /// ou arquivo de configuração.
    static ref SUSPICIOUS_DOMAINS: Vec<&'static str> = vec![
        // Conteúdo adulto
        "onlyfans.com",
        "pornhub.com",
        "xvideos.com",
        "xnxx.com",

        // Encurtadores frequentemente usados em spam
        "bit.ly",
        "tinyurl.com",

        // Convites suspeitos Telegram
        "t.me/joinchat",
        "t.me/+",

        // Apostas
        "bet365",
        "sportbet",
        "cassino",

        // Categorias suspeitas genéricas
        "adult",
        "porn",
    ];
}

/// Verifica se uma mensagem contém links suspeitos.
///
/// Analisa:
/// - domínio da URL
/// - padrões conhecidos de abuso
///
/// Retorna `true` quando encontra uma URL bloqueada.
pub fn is_suspicious_link(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }

    for capture in URL_REGEX.captures_iter(text) {
        let url = match Url::parse(&capture[1]) {
            Ok(url) => url,
            Err(_) => continue,
        };

        let host = url.host_str().unwrap_or("").to_lowercase();

        if SUSPICIOUS_DOMAINS
            .iter()
            .any(|domain| host.contains(domain))
        {
            return true;
        }
    }

    false
}
