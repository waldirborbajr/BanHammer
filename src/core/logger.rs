use crate::i18n::lang::Lang;

/// Inicializa o sistema de logs
pub fn init() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

/// Registra informações de inicialização do bot
pub fn startup(default_lang: Lang) {
    log::info!(
        "🚀 BanHammer Multi-Idioma iniciado! Idioma padrão: {}",
        lang_name(default_lang)
    );
}

/// Retorna o nome curto do idioma
fn lang_name(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => "pt-BR",
        Lang::En => "en-US",
        Lang::Es => "es-ES",
    }
}