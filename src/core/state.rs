use crate::{
    core::config::Config,
    moderation::rules::ModerationRules,
    storage::{memory::MemoryStorage, sqlite},
};

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,

    pub memory: MemoryStorage,

    pub moderation: ModerationRules,

    pub db: SqlitePool,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let moderation = ModerationRules::load("config/moderation.toml")?;

        let db = sqlite::init_database(&config.database_url).await?;

        Ok(Self {
            config,

            memory: MemoryStorage::new(),

            moderation,

            db,
        })
    }
}