//! Mensagens em Espanhol (Espanha)

pub const HELP: &str = r#"🤖 *BanHammer*

Protege automáticamente los grupos contra:

• Pornografía
• Contenido sexual
• Prostitución
• Juegos de azar
• Spam
• Estafas
• Enlaces maliciosos

Comandos disponibles:

/help - Muestra esta ayuda.

/status - Muestra el estado del bot.

/language <pt|en|es> - Cambia el idioma del grupo (solo administradores)."#;

pub const STATUS: &str = "🟢 BanHammer está en línea y protegiendo este grupo.";


pub const VIOLATION_GENERIC: &str =
    "🚫 Se ha detectado contenido prohibido y se han aplicado las medidas correspondientes.";

pub const LANG_SET: &str = "✅ El idioma del grupo se ha cambiado correctamente a Español.";

pub const LANG_INVALID: &str = "⚠️ Idioma no válido. Utiliza: pt, en o es.";

pub const LANG_NO_PERMISSION: &str =
    "⛔ Solo los administradores pueden cambiar el idioma del grupo.";

pub fn BANNED(username: &str) -> String {
    format!("🚫 @{username} ha sido expulsado por infringir las normas.")
}
