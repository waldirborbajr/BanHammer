use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::RwLock;


/// Estado temporário em memória.
///
/// Todos os dados aqui são perdidos
/// quando o processo é reiniciado.
#[derive(Clone, Default)]
pub struct MemoryStorage {
    /// Idioma configurado por grupo.
    chat_languages: Arc<RwLock<HashMap<i64, String>>>,

    /// Usuários que já foram processados
    /// durante o ciclo atual.
    processed_users: Arc<RwLock<HashSet<i64>>>,

    /// Contador de violações por usuário.
    violation_counter: Arc<RwLock<HashMap<i64, u32>>>,
}


impl MemoryStorage {


    /// Cria uma nova instância vazia.
    pub fn new() -> Self {
        Self::default()
    }


    // =========================
    // CHAT LANGUAGE
    // =========================


    pub async fn set_chat_language(
        &self,
        chat_id: i64,
        language: String,
    ) {
        self.chat_languages
            .write()
            .await
            .insert(chat_id, language);
    }



    pub async fn get_chat_language(
        &self,
        chat_id: i64,
    ) -> Option<String> {

        self.chat_languages
            .read()
            .await
            .get(&chat_id)
            .cloned()
    }



    // =========================
    // PROCESSED USERS
    // =========================


    pub async fn mark_user_processed(
        &self,
        user_id: i64,
    ) {
        self.processed_users
            .write()
            .await
            .insert(user_id);
    }



    pub async fn was_user_processed(
        &self,
        user_id: i64,
    ) -> bool {

        self.processed_users
            .read()
            .await
            .contains(&user_id)
    }



    // =========================
    // VIOLATIONS
    // =========================


    pub async fn add_violation(
        &self,
        user_id: i64,
    ) -> u32 {

        let mut counter =
            self.violation_counter
                .write()
                .await;


        let value =
            counter
                .entry(user_id)
                .or_insert(0);


        *value += 1;

        *value
    }



    pub async fn get_violation_count(
        &self,
        user_id: i64,
    ) -> u32 {

        self.violation_counter
            .read()
            .await
            .get(&user_id)
            .copied()
            .unwrap_or(0)
    }



    /// Limpa todo estado temporário.
    pub async fn clear(&self) {

        self.chat_languages
            .write()
            .await
            .clear();


        self.processed_users
            .write()
            .await
            .clear();


        self.violation_counter
            .write()
            .await
            .clear();
    }
}