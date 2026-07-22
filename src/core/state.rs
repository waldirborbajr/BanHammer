use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::{
    core::config::Config,
    moderation::rules::{self, ModerationRules},
    storage::{memory::MemoryStorage, sqlite},
};

/// Estado compartilhado da aplicação.
///
/// Esta estrutura é clonada pelos handlers do Telegram.
/// Recursos compartilhados ficam protegidos por `Arc` para
/// evitar cópias desnecessárias e permitir acesso concorrente.
#[derive(Clone)]
pub struct AppState {
    /// Configuração carregada na inicialização.
    pub config: Config,

    /// Estado temporário em memória.
    pub memory: Arc<MemoryStorage>,

    /// Regras de moderação carregadas de
    /// `config/moderation.toml`.
    ///
    /// Podem ser recarregadas em runtime através
    /// do comando `/reload`.
    pub moderation: Arc<RwLock<ModerationRules>>,

    /// Lista de domínios bloqueados.
    ///
    /// É carregada do SQLite durante o boot e mantida
    /// em memória para evitar consultas ao banco a
    /// cada mensagem processada.
    pub blocked_domains: Arc<RwLock<Vec<String>>>,

    /// Pool de conexões SQLite.
    pub db: SqlitePool,
}

impl AppState {
    /// Inicializa todo o estado da aplicação.
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let moderation = ModerationRules::load(rules::CONFIG_PATH)?;

        let db = sqlite::init_database(&config.database_url).await?;

        let blocked_domains = sqlite::get_blocked_domains(&db).await?;

        Ok(Self {
            config,

            memory: Arc::new(MemoryStorage::new()),

            moderation: Arc::new(RwLock::new(moderation)),

            blocked_domains: Arc::new(RwLock::new(blocked_domains)),

            db,
        })
    }

    /// Recarrega `config/moderation.toml` sem reiniciar
    /// o processo.
    ///
    /// Em caso de erro, as regras atualmente carregadas
    /// permanecem válidas.
    pub async fn reload_moderation(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let fresh = ModerationRules::load(rules::CONFIG_PATH)?;

        let mut guard = self.moderation.write().await;

        *guard = fresh;

        Ok(())
    }

    /// Adiciona um domínio à lista de bloqueio.
    ///
    /// A alteração é persistida no SQLite e refletida
    /// imediatamente na cópia mantida em memória.
    pub async fn add_blocked_domain(&self, domain: &str) -> Result<(), sqlx::Error> {
        let normalized = domain.trim().to_lowercase();

        sqlite::add_blocked_domain(&self.db, &normalized).await?;

        let mut guard = self.blocked_domains.write().await;

        if !guard.iter().any(|existing| existing == &normalized) {
            guard.push(normalized);
        }

        Ok(())
    }
}
