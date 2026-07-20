use super::{en, es, lang::Lang, pt};

/// Mensagem de ajuda
pub fn help(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::HELP,
        Lang::En => en::HELP,
        Lang::Es => es::HELP,
    }
}

/// Status do bot
pub fn status(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::STATUS,
        Lang::En => en::STATUS,
        Lang::Es => es::STATUS,
    }
}

/// Usuário banido
pub fn banned(lang: Lang, username: &str) -> String {
    match lang {
        Lang::Pt => pt::BANNED(username),
        Lang::En => en::BANNED(username),
        Lang::Es => es::BANNED(username),
    }
}

/// Violação genérica
pub fn violation_generic(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::VIOLATION_GENERIC,

        Lang::En => en::VIOLATION_GENERIC,

        Lang::Es => es::VIOLATION_GENERIC,
    }
}

/// Idioma alterado
pub fn lang_set(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_SET,

        Lang::En => en::LANG_SET,

        Lang::Es => es::LANG_SET,
    }
}

/// Idioma inválido
pub fn lang_invalid(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_INVALID,

        Lang::En => en::LANG_INVALID,

        Lang::Es => es::LANG_INVALID,
    }
}

/// Sem permissão para alterar idioma
pub fn lang_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_NO_PERMISSION,

        Lang::En => en::LANG_NO_PERMISSION,

        Lang::Es => es::LANG_NO_PERMISSION,
    }
}
