use serde::Deserialize;
use std::fmt;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ModerationRules {
    pub pornography: KeywordGroup,

    pub gambling: KeywordGroup,

    pub spam: KeywordGroup,

    pub links: LinkGroup,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeywordGroup {
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinkGroup {
    pub domains: Vec<String>,
}

/// Erro de validação de configuração de moderação.
#[derive(Debug)]
pub struct ValidationError {
    pub section: &'static str,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "moderation.toml: seção [{}] está vazia — o bot não pode iniciar sem regras de moderação carregadas",
            self.section
        )
    }
}

impl std::error::Error for ValidationError {}

impl ModerationRules {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let rules = toml::from_str::<ModerationRules>(&content)?;

        rules.validate()?;

        Ok(rules)
    }

    /// Garante que nenhuma categoria de moderação
    /// foi carregada vazia por engano (config incompleta,
    /// arquivo corrompido, edição malfeita, etc).
    fn validate(&self) -> Result<(), ValidationError> {
        if self.pornography.keywords.is_empty() {
            return Err(ValidationError {
                section: "pornography",
            });
        }

        if self.gambling.keywords.is_empty() {
            return Err(ValidationError { section: "gambling" });
        }

        if self.spam.keywords.is_empty() {
            return Err(ValidationError { section: "spam" });
        }

        if self.links.domains.is_empty() {
            return Err(ValidationError { section: "links" });
        }

        Ok(())
    }
}
