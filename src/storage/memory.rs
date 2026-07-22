use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;

/// Armazena estado temporário utilizado durante
/// a execução do bot.
///
/// Nenhuma informação aqui é persistida.
/// Todo o conteúdo é perdido ao reiniciar o processo.
#[derive(Clone, Default)]
pub struct MemoryStorage {
    /// Cache do idioma configurado por chat.
    ///
    /// A fonte oficial permanece no banco de dados.
    chat_languages: Arc<RwLock<HashMap<i64, String>>>,

    /// Updates já processados.
    ///
    /// Evita processar o mesmo Update do Telegram
    /// mais de uma vez.
    processed_updates: Arc<RwLock<HashSet<u32>>>,

    /// Contador temporário de violações por usuário.
    violation_counter: Arc<RwLock<HashMap<i64, u32>>>,
}

impl MemoryStorage {
    /// Cria uma nova instância.
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================
    // CHAT LANGUAGE CACHE
    // ============================================================

    /// Atualiza o idioma em cache.
    pub async fn set_chat_language(&self, chat_id: i64, language: impl Into<String>) {
        self.chat_languages
            .write()
            .await
            .insert(chat_id, language.into());
    }

    /// Obtém o idioma do cache.
    pub async fn get_chat_language(&self, chat_id: i64) -> Option<String> {
        self.chat_languages.read().await.get(&chat_id).cloned()
    }

    // ============================================================
    // PROCESSED UPDATES
    // ============================================================

    /// Marca um Update como processado.
    pub async fn mark_update_processed(&self, update_id: u32) {
        self.processed_updates.write().await.insert(update_id);
    }

    /// Verifica se um Update já foi processado.
    pub async fn was_update_processed(&self, update_id: u32) -> bool {
        self.processed_updates.read().await.contains(&update_id)
    }
    // ============================================================
    // VIOLATIONS
    // ============================================================

    /// Incrementa o contador de violações.
    ///
    /// Retorna o número atual de violações.
    pub async fn add_violation(&self, user_id: i64) -> u32 {
        let mut counter = self.violation_counter.write().await;

        let entry = counter.entry(user_id).or_insert(0);

        *entry += 1;

        *entry
    }

    /// Obtém o número atual de violações.
    /// Obtém o número atual de violações.
    // TODO(roadmap): expor via um futuro comando administrativo
    // (ex.: `/violations <user_id>`).
    #[allow(dead_code)]
    pub async fn get_violation_count(&self, user_id: i64) -> u32 {
        self.violation_counter
            .read()
            .await
            .get(&user_id)
            .copied()
            .unwrap_or(0)
    }

    /// Remove o histórico de violações de um usuário.
    pub async fn reset_violation_count(&self, user_id: i64) {
        self.violation_counter.write().await.remove(&user_id);
    }
}
