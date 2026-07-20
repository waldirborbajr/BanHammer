use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub default_language: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://banhammer.db".to_string()),

            default_language: env::var("BOT_DEFAULT_LANG").unwrap_or_else(|_| "pt".to_string()),
        }
    }
}
