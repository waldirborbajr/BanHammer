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
}

impl fmt::Display for ViolationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
