use crate::{
    core::config::Config,
    moderation::rules::{self, ModerationRules},
    storage::{memory::MemoryStorage, sqlite},
};

use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,

    pub memory: MemoryStorage,

    /// Regras de moderação carregadas de `config/moderation.toml`.
    ///
    /// Fica atrás de um `Arc<RwLock<..>>` para poder ser
    /// recarregada em runtime (comando /reload) e refletir
    /// imediatamente em todos os clones do AppState.
    pub moderation: Arc<RwLock<ModerationRules>>,

    pub db: SqlitePool,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let moderation = ModerationRules::load(rules::CONFIG_PATH)?;

        let db = sqlite::init_database(&config.database_url).await?;

        Ok(Self {
            config,

            memory: MemoryStorage::new(),

            moderation: Arc::new(RwLock::new(moderation)),

            db,
        })
    }

    /// Recarrega `config/moderation.toml` do disco e substitui
    /// as regras em memória, sem reiniciar o processo.
    ///
    /// Retorna erro se o arquivo não existir, tiver TOML inválido,
    /// ou falhar na validação (alguma seção vazia) — nesse caso
    /// as regras antigas continuam valendo.
    pub async fn reload_moderation(&self) -> Result<(), Box<dyn std::error::Error>> {
        let fresh = ModerationRules::load(rules::CONFIG_PATH)?;

        let mut guard = self.moderation.write().await;

        *guard = fresh;

        Ok(())
    }
}
