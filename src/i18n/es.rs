//! Mensagens em Espanhol (España)

pub fn help() -> String {
    format!(
        r#"🤖 *BanHammer v{}*

Protege automáticamente los grupos contra:

- Pornografía
- Contenido sexual
- Prostitución
- Juegos de azar
- Spam
- Estafas
- Enlaces maliciosos

Comandos disponibles:

/help - Muestra esta ayuda.

/status - Muestra el estado del bot.

/stats - Muestra estadísticas de moderación del grupo.

/language <pt|en|es> - Cambia el idioma del grupo (solo administradores)."#,
        env!("CARGO_PKG_VERSION")
    )
}


pub const STATUS: &str =
    "🟢 BanHammer está en línea y protegiendo este grupo.";


pub const VIOLATION_GENERIC: &str =
    "🚫 Se ha detectado contenido prohibido y se han aplicado las medidas correspondientes.";


pub fn BANNED(username: &str) -> String {
    format!(
        "🚫 @{username} ha sido expulsado por infringir las normas."
    )
}


pub const LANG_SET: &str =
    "✅ El idioma del grupo se ha cambiado correctamente a Español.";


pub const LANG_INVALID: &str =
    "⚠️ Idioma no válido. Utiliza: pt, en o es.";


pub const LANG_NO_PERMISSION: &str =
    "⛔ Solo los administradores pueden cambiar el idioma del grupo.";

pub const STATS_TITLE: &str = "📊 *Estadísticas del grupo*";
pub const STATS_TOTAL: &str = "Violaciones totales";
pub const STATS_24H: &str = "Últimas 24h";
pub const STATS_BY_TYPE: &str = "Por categoría";
pub const STATS_TOP: &str = "Principales infractores";
pub const STATS_EMPTY: &str = "✅ Todavía no hay violaciones registradas en este grupo.";