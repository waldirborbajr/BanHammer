use std::env;

/// Idiomas suportados pelo BanHammer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    Pt,
    En,
    Es,
}

impl Lang {
    /// Converte um código ISO (ou variante) para `Lang`.
    ///
    /// Exemplos aceitos:
    ///
    /// - pt
    /// - pt-BR
    /// - pt_BR
    /// - en
    /// - en-US
    /// - es
    /// - es-ES
    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_ascii_lowercase().as_str() {
            "pt" | "pt-br" | "pt_br" => Some(Self::Pt),

            "en" | "en-us" | "en_us" => Some(Self::En),

            "es" | "es-es" | "es_es" => Some(Self::Es),

            _ => None,
        }
    }

    /// Retorna o código ISO curto do idioma.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Pt => "pt",
            Self::En => "en",
            Self::Es => "es",
        }
    }

    /// Nome legível do idioma.
    #[allow(dead_code)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Pt => "Português",
            Self::En => "English",
            Self::Es => "Español",
        }
    }

    /// Idioma padrão definido pela variável
    /// de ambiente `BOT_DEFAULT_LANG`.
    ///
    /// Caso não exista ou seja inválida,
    /// retorna Português.
    pub fn default_from_env() -> Self {
        env::var("BOT_DEFAULT_LANG")
            .ok()
            .as_deref()
            .and_then(Self::from_code)
            .unwrap_or(Self::Pt)
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self::default_from_env()
    }
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
