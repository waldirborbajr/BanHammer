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

    /// Domínios bloqueados via /blockdomain (tabela `blocked_domains`).
    ///
    /// Mantido em memória (carregado do banco no boot) para não
    /// bater no SQLite a cada mensagem — só é atualizado quando
    /// um admin roda /blockdomain.
    pub blocked_domains: Arc<RwLock<Vec<String>>>,

    pub db: SqlitePool,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let moderation = ModerationRules::load(rules::CONFIG_PATH)?;

        let db = sqlite::init_database(&config.database_url).await?;

        let blocked_domains = sqlite::get_blocked_domains(&db).await?;

        Ok(Self {
            config,

            memory: MemoryStorage::new(),

            moderation: Arc::new(RwLock::new(moderation)),

            blocked_domains: Arc::new(RwLock::new(blocked_domains)),

            db,
        })
    }

    /// Recarrega `config/moderation.toml` do disco e substitui
    /// as regras em memória, sem reiniciar o processo.
    ///
    /// Retorna erro se o arquivo não existir, tiver TOML inválido,
    /// ou falhar na validação (alguma seção vazia) — nesse caso
    /// as regras antigas continuam valendo.
    pub async fn reload_moderation(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let fresh = ModerationRules::load(rules::CONFIG_PATH)?;

        let mut guard = self.moderation.write().await;

        *guard = fresh;

        Ok(())
    }

    /// Adiciona um domínio à lista de bloqueio: persiste no SQLite
    /// e atualiza a cópia em memória usada pelo motor de moderação.
    ///
    /// Idempotente — bloquear o mesmo domínio duas vezes não gera erro
    /// nem duplicata (a query usa `INSERT OR IGNORE`).
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
