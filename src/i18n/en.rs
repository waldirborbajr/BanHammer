pub const HELP: &str = r#"🤖 *BotHammer* v2.1

Automatically detects:
• Pornography
• Sales / spam
• Gambling
• Child exploitation / CSAM
• Suspicious links

Commands:
/help — this message
/status — bot status
/language <pt|en|es> — sets the bot's language for this group (admins only)"#;


pub const STATUS: &str =
    "✅ BotHammer is online and protecting the group!";


pub const VIOLATION_GENERIC: &str =
    "🚫 Prohibited content detected and removed.";


pub fn BANNED(username: &str) -> String {
    format!("🚫 @{username} has been banned for violating the rules.")
}


pub const LANG_SET: &str =
    "✅ Bot language set to English.";


pub const LANG_INVALID: &str =
    "⚠️ Invalid language. Use `/language pt`, `/language en` or `/language es`.";


pub const LANG_NO_PERMISSION: &str =
    "⚠️ Only administrators can change the bot's language.";