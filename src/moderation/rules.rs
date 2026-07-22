use serde::Deserialize;
use std::fmt;
use std::fs;

/// Caminho padrão do arquivo de regras de moderação.
pub const CONFIG_PATH: &str = "config/moderation.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct ModerationRules {
    pub pornography: KeywordGroup,

    pub gambling: KeywordGroup,

    pub spam: KeywordGroup,

    pub links: LinkGroup,

    pub strikes: StrikesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeywordGroup {
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinkGroup {
    pub domains: Vec<String>,
}

/// Escada de punições para violações de baixa severidade
/// (gambling, spam). Ver `ViolationType::is_zero_tolerance`
/// para as categorias que ignoram essa escada e banem direto.
#[derive(Debug, Deserialize, Clone)]
pub struct StrikesConfig {
    pub window_days: i64,
    pub mute_at: u32,
    pub kick_at: u32,
    pub ban_at: u32,
    pub mute_duration_minutes: i64,
    pub kick_ban_seconds: i64,
}

/// Erro de validação de configuração de moderação.
#[derive(Debug)]
pub struct ValidationError {
    pub section: &'static str,
    pub reason: Option<&'static str>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reason {
            Some(reason) => write!(
                f,
                "moderation.toml: seção [{}] inválida — {}",
                self.section, reason
            ),

            None => write!(
                f,
                "moderation.toml: seção [{}] está vazia — o bot não pode iniciar sem regras de moderação carregadas",
                self.section
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

impl ModerationRules {
    /// `Send + Sync` no tipo de erro é necessário porque este método
    /// é chamado dentro do handler `/reload`, e o dptree (teloxide)
    /// exige que o Future de cada endpoint seja `Send`. Um
    /// `Box<dyn Error>` comum não é `Send` e quebra a injeção de
    /// dependência do dispatcher (erro `Injectable<...>` no build).
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(path)?;

        let rules = toml::from_str::<ModerationRules>(&content)?;

        rules.validate()?;

        Ok(rules)
    }

    /// Garante que nenhuma categoria de moderação
    /// foi carregada vazia por engano (config incompleta,
    /// arquivo corrompido, edição malfeita, etc), e que a
    /// escada de strikes está configurada de forma coerente.
    fn validate(&self) -> Result<(), ValidationError> {
        if self.pornography.keywords.is_empty() {
            return Err(ValidationError {
                section: "pornography",
                reason: None,
            });
        }

        if self.gambling.keywords.is_empty() {
            return Err(ValidationError {
                section: "gambling",
                reason: None,
            });
        }

        if self.spam.keywords.is_empty() {
            return Err(ValidationError {
                section: "spam",
                reason: None,
            });
        }

        if self.links.domains.is_empty() {
            return Err(ValidationError {
                section: "links",
                reason: None,
            });
        }

        self.strikes.validate()?;

        Ok(())
    }
}

impl StrikesConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        if !(1 <= self.mute_at && self.mute_at < self.kick_at && self.kick_at < self.ban_at) {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("mute_at, kick_at e ban_at precisam ser crescentes e >= 1"),
            });
        }

        if self.window_days < 1 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("window_days precisa ser >= 1"),
            });
        }

        if self.mute_duration_minutes < 1 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("mute_duration_minutes precisa ser >= 1"),
            });
        }

        // Telegram considera bans com until_date a menos de 30s no
        // futuro como permanentes — abaixo de 31 o "kick" viraria ban.
        if self.kick_ban_seconds < 31 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some(
                    "kick_ban_seconds precisa ser >= 31 (o Telegram trata valores menores como ban permanente)",
                ),
            });
        }

        Ok(())
    }
}
