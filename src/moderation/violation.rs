use std::fmt;

/// Tipos de violação detectados pelo motor de moderação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    Csam,
    Pornography,
    Gambling,
    SuspiciousLink,
    Spam,
}

impl ViolationType {
    /// Representação em `&str`, usada para persistência (SQLite)
    /// e para exibição em `/stats`.
    pub fn as_str(&self) -> &'static str {
        match self {
            ViolationType::Csam => "csam",
            ViolationType::Pornography => "pornography",
            ViolationType::Gambling => "gambling",
            ViolationType::SuspiciousLink => "suspicious_link",
            ViolationType::Spam => "spam",
        }
    }

    /// Violações de zero tolerância banem direto, sem passar
    /// pela escada de strikes (aviso → mute → kick → ban).
    ///
    /// `SuspiciousLink` entra aqui porque a lista de domínios em
    /// `moderation.toml` mistura sites adultos com apostas — não dá
    /// pra inferir a severidade só pela URL, então tratamos como
    /// zero tolerância por segurança. Ajuste aqui se quiser que
    /// links suspeitos passem pela escada gradual como gambling/spam.
    pub fn is_zero_tolerance(&self) -> bool {
        matches!(
            self,
            ViolationType::Csam | ViolationType::Pornography | ViolationType::SuspiciousLink
        )
    }
}

impl fmt::Display for ViolationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
