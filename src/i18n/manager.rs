use teloxide::types::ChatId;

use crate::{
    core::state::AppState,
    i18n::lang::Lang,
};

/// Gerencia o idioma dos chats.
///
/// Atualmente utiliza apenas o cache em memória (`MemoryStorage`),
/// mas futuramente poderá carregar e persistir as configurações
/// no SQLite sem alterar o restante da aplicação.
pub struct LanguageManager;

impl LanguageManager {
    /// Retorna o idioma configurado para um chat.
    ///
    /// Caso o chat ainda não possua idioma definido,
    /// retorna o idioma padrão da aplicação.
    pub async fn get(
        state: &AppState,
        chat_id: ChatId,
    ) -> Lang {
        if let Some(code) = state
            .memory
            .get_chat_language(chat_id.0)
            .await
        {
            if let Some(lang) = Lang::from_code(&code) {
                return lang;
            }
        }

        Lang::default()
    }

    /// Define o idioma de um chat.
    pub async fn set(
        state: &AppState,
        chat_id: ChatId,
        lang: Lang,
    ) {
        state
            .memory
            .set_chat_language(chat_id.0, lang.code())
            .await;

        log::info!(
            "Language for chat {} changed to {}",
            chat_id,
            lang
        );
    }

    /// Remove a configuração personalizada.
    ///
    /// O chat voltará a utilizar o idioma padrão.
    pub async fn reset(
        state: &AppState,
        chat_id: ChatId,
    ) {
        state
            .memory
            .set_chat_language(
                chat_id.0,
                Lang::default().code(),
            )
            .await;

        log::info!(
            "Language for chat {} reset to {}",
            chat_id,
            Lang::default()
        );
    }

    /// Verifica se existe idioma configurado
    /// para um determinado chat.
    pub async fn exists(
        state: &AppState,
        chat_id: ChatId,
    ) -> bool {
        state
            .memory
            .get_chat_language(chat_id.0)
            .await
            .is_some()
    }
}
