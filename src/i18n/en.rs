pub fn help() -> String {
    format!(
        r#"🤖 *BotHammer v{}*

Automatically detects:
- Pornography
- Sales / spam
- Gambling
- Child exploitation / CSAM
- Suspicious links

Commands:
/help — this message
/status — bot status
/stats — group moderation stats
/language <pt|en|es> — sets the bot's language for this group (admins only)"#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "✅ BotHammer is online and protecting the group!";

pub const VIOLATION_GENERIC: &str = "🚫 Prohibited content detected and removed.";

pub fn BANNED(username: &str) -> String {
    format!("🚫 @{username} has been banned for violating the rules.")
}

pub const LANG_SET: &str = "✅ Bot language set to English.";

pub const LANG_INVALID: &str =
    "⚠️ Invalid language. Use `/language pt`, `/language en` or `/language es`.";

pub const LANG_NO_PERMISSION: &str = "⚠️ Only administrators can change the bot's language.";

pub const STATS_TITLE: &str = "📊 *Group statistics*";
pub const STATS_TOTAL: &str = "Total violations";
pub const STATS_24H: &str = "Last 24h";
pub const STATS_BY_TYPE: &str = "By category";
pub const STATS_TOP: &str = "Top offenders";
pub const STATS_EMPTY: &str = "✅ No violations recorded in this group yet.";