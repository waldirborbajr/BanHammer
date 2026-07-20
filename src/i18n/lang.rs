use std::{collections::HashMap, env, sync::RwLock};

use lazy_static::lazy_static;
use teloxide::types::ChatId;

/// Idiomas suportados pelo BanHammer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    Pt,

    En,

    Es,
}

impl Lang {
    /// Converte código de idioma para Lang
    ///
    /// Exemplos:
    /// pt
    /// pt-BR
    /// en
    /// es-ES
    ///
    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_lowercase().as_str() {
            "pt" | "pt-br" | "pt_br" => Some(Self::Pt),

            "en" | "en-us" | "en_us" => Some(Self::En),

            "es" | "es-es" | "es_es" => Some(Self::Es),

            _ => None,
        }
    }

    /// Retorna código ISO curto
    pub fn code(&self) -> &'static str {
        match self {
            Self::Pt => "pt",

            Self::En => "en",

            Self::Es => "es",
        }
    }

    /// Idioma padrão vindo do ambiente
    pub fn default_from_env() -> Self {
        let lang = env::var("BOT_DEFAULT_LANG").unwrap_or_else(|_| "pt".to_string());

        Self::from_code(&lang).unwrap_or(Self::Pt)
    }
}

lazy_static! {


    /// Idioma padrão global
    pub static ref DEFAULT_LANG: Lang =
        Lang::default_from_env();



    /// Idiomas específicos por grupo
    static ref CHAT_LANGS:
        RwLock<HashMap<ChatId, Lang>>
    =
        RwLock::new(
            HashMap::new()
        );

}

/// Retorna o idioma configurado para o grupo
///
/// Caso não exista configuração,
/// usa o idioma padrão.
pub fn lang_for_chat(chat_id: ChatId) -> Lang {
    CHAT_LANGS
        .read()
        .unwrap()
        .get(&chat_id)
        .copied()
        .unwrap_or(*DEFAULT_LANG)
}

/// Define idioma do grupo
pub fn set_lang_for_chat(chat_id: ChatId, lang: Lang) {
    CHAT_LANGS.write().unwrap().insert(chat_id, lang);

    log::info!("Idioma do chat {} alterado para {}", chat_id, lang.code());
}

/// Remove configuração personalizada
///
/// Volta a usar BOT_DEFAULT_LANG
pub fn reset_lang_for_chat(chat_id: ChatId) {
    CHAT_LANGS.write().unwrap().remove(&chat_id);

    log::info!("Idioma do chat {} voltou ao padrão", chat_id);
}
