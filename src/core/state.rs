use crate::{
    core::config::Config, moderation::rules::ModerationRules, storage::memory::MemoryStorage,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,

    pub memory: MemoryStorage,

    pub moderation: ModerationRules,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let moderation = ModerationRules::load("config/moderation.toml")?;

        Ok(Self {
            config,

            memory: MemoryStorage::new(),

            moderation,
        })
    }
}
