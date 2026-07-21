use serde::Deserialize;
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

impl ModerationRules {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let rules = toml::from_str::<ModerationRules>(&content)?;

        Ok(rules)
    }
}
