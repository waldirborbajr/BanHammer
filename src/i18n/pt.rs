pub fn help() -> String {
    format!(
        r#"🤖 *BotHammer v{}*

Detecta automaticamente:
• Pornografia
• Vendas / spam
• Apostas
• Pedofilia / CSAM
• Links suspeitos

Comandos:
/help — esta mensagem
/status — status do bot
/language <pt|en|es> — define o idioma do bot neste grupo (apenas administradores)"#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "✅ BotHammer está online e protegendo o grupo!";

pub const VIOLATION_GENERIC: &str = "🚫 Conteúdo proibido detectado e removido.";

pub fn BANNED(username: &str) -> String {
    format!("🚫 @{username} foi banido por violação das regras.")
}

pub const LANG_SET: &str = "✅ Idioma do bot definido para Português.";

pub const LANG_INVALID: &str =
    "⚠️ Idioma inválido. Use `/language pt`, `/language en` ou `/language es`.";

pub const LANG_NO_PERMISSION: &str = "⚠️ Apenas administradores podem alterar o idioma do bot.";
