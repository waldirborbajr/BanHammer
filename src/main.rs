use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use teloxide::{
    prelude::*,
    types::{
        ChatId,
        ChatMemberKind,
        ParseMode,
        User,
    },
    utils::command::BotCommands,
};

use regex::Regex;
use lazy_static::lazy_static;
use url::Url;

// ==================== i18n ====================

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    Pt,
    En,
    Es,
}
impl Lang {
fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "pt" | "pt-br" | "ptbr" => Some(Lang::Pt),
            "en" | "en-us" | "enus" => Some(Lang::En),
            "es" | "es-es" | "eses" => Some(Lang::Es),
            _ => None,
        }
    }


    /// Read the process-wide default language from `BOT_DEFAULT_LANG`.
    /// Falls back to Portuguese if unset or invalid, preserving prior behavior.
    fn default_from_env() -> Self {
        env::var("BOT_DEFAULT_LANG")
            .ok()
            .and_then(|v| Lang::from_code(&v))
            .unwrap_or(Lang::Pt)
    }
}

lazy_static! {
    /// Bot-wide default language, resolved once at startup.
    static ref DEFAULT_LANG: Lang = Lang::default_from_env();

    /// Per-chat language override. In-memory only — resets on restart.
    /// See "Suggested improvements" for persisting this.
    static ref CHAT_LANG: Mutex<HashMap<ChatId, Lang>> = Mutex::new(HashMap::new());
}

fn lang_for_chat(chat_id: ChatId) -> Lang {
    CHAT_LANG
        .lock()
        .unwrap()
        .get(&chat_id)
        .copied()
        .unwrap_or(*DEFAULT_LANG)
}

fn set_lang_for_chat(chat_id: ChatId, lang: Lang) {
    CHAT_LANG.lock().unwrap().insert(chat_id, lang);
}

mod messages {
    use super::Lang;

    pub fn help(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => {
                "🤖 *BanHammer* v2.1\n\n\
                 Detecta automaticamente:\n\
                 • Pornografia\n\
                 • Vendas / spam\n\
                 • Apostas\n\
                 • Pedofilia / CSAM\n\
                 • Links suspeitos\n\n\
                 Comandos:\n\
                 /help — esta mensagem\n\
                 /status — status do bot\n\
                 /language <pt|en> — define o idioma do bot neste grupo (apenas administradores)"
            }
            Lang::En => {
                "🤖 *BanHammer* v2.1\n\n\
                 Automatically detects:\n\
                 • Pornography\n\
                 • Sales / spam\n\
                 • Gambling\n\
                 • Child exploitation / CSAM\n\
                 • Suspicious links\n\n\
                 Commands:\n\
                 /help — this message\n\
                 /status — bot status\n\
                 /language <pt|en> — sets the bot's language for this group (admins only)"
            }
            Lang::Es => {
            "🤖 *BanHammer* v2.1\n\n\
             Detecta automáticamente:\n\
             • Pornografía\n\
             • Ventas / spam\n\
             • Apuestas\n\
             • Explotación infantil / CSAM\n\
             • Enlaces sospechosos\n\n\
             Comandos:\n\
             /help — esta ayuda\n\
             /status — estado del bot\n\
             /language <pt|en|es> — cambia el idioma del bot para este grupo (solo administradores)"
        }
        }
    }

    pub fn status(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => "✅ BanHammer está online e protegendo o grupo!",
            Lang::En => "✅ BanHammer is online and protecting the group!",
                    Lang::Es => "✅ ¡BanHammer está en línea y protegiendo el grupo!",

        }
    }

    pub fn violation_generic(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => "🚫 Conteúdo proibido detectado e removido.",
            Lang::En => "🚫 Prohibited content detected and removed.",
                    Lang::Es => "🚫 Contenido prohibido detectado y eliminado.",

        }
    }

    pub fn banned(lang: Lang, username: &str) -> String {
        match lang {
            Lang::Pt => format!("🚫 @{username} foi banido por violação das regras."),
            Lang::En => format!("🚫 @{username} has been banned for violating the rules."),
                    Lang::Es => format!("🚫 @{username} ha sido expulsado por infringir las normas."),

        }
    }

    pub fn lang_set(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => "✅ Idioma do bot definido para Português.",
            Lang::En => "✅ Bot language set to English.",
                    Lang::Es => "✅ Idioma del bot establecido en Español.",

        }
    }

    pub fn lang_invalid(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => "⚠️ Idioma inválido. Use `/language pt` ou `/language en`.",
            Lang::En => "⚠️ Invalid language. Use `/language pt` or `/language en`.",
                    Lang::Es => "⚠️ Idioma no válido. Usa `/language pt`, `/language en` o `/language es`.",
        }
    }

    pub fn lang_no_permission(lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => "⚠️ Apenas administradores podem alterar o idioma do bot.",
            Lang::En => "⚠️ Only administrators can change the bot's language.",
                    Lang::Es => "⚠️ Solo los administradores pueden cambiar el idioma del bot.",


        }
    }
}

// ==================== COMMANDS ====================

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "BanHammer Commands")]
enum Command {
    #[command(description = "Mostrar ajuda / Show help")]
    Help,
    #[command(description = "Status do bot / Bot status")]
    Status,
    #[command(description = "Definir idioma / Set language / Establecer idioma: pt|en|es")]
    Language(String),
}

// ==================== PALAVRAS PROIBIDAS MULTI-IDIOMA ====================
// Detection stays multi-language regardless of the UI language above:
// prohibited content can be posted in Portuguese or English either way.
lazy_static! {
    static ref BAD_PATTERNS: Vec<Regex> = vec![
        // === PORTUGUÊS ===
        Regex::new(r"(?i)(porn|sexo|putaria|nudes|onlyfans|nsfw|xxx|porno|puta|piranha|boquete|anal|gozar|tesão|foder|foda|cu|bunda|pelada|safada)").unwrap(),
        Regex::new(r"(?i)(compre|promo|desconto|oferta|barato|revenda|fornecedor|entrega|frete grátis|link na bio)").unwrap(),
        Regex::new(r"(?i)(bet|aposta|cassino|slot|roleta|fortuna|odds|aposte|ganhe dinheiro)").unwrap(),
        Regex::new(r"(?i)(loli|shota|pedofilia|cp|menor nua|criança nua)").unwrap(),

        // === ENGLISH ===
        Regex::new(r"(?i)(porn|sex|nudes|onlyfans|nsfw|xxx|horny|fuck|slut|bitch|ass|dick|pussy|cum)").unwrap(),
        Regex::new(r"(?i)(buy now|promo|discount|offer|cheap|resell|supplier|free shipping)").unwrap(),
        Regex::new(r"(?i)(bet|gambling|casino|slot|roulette|odds|betting|sportbet)").unwrap(),
        Regex::new(r"(?i)(loli|shota|pedophilia|cp|underage|child porn)").unwrap(),

        // === GOLPES / SPAM COMUM ===
        Regex::new(r"(?i)(dinheiro fácil|renda extra|ganhe em casa|bitcoin|cripto|investment|make money|mlm|pyramid)").unwrap(),
    ];

    static ref SUSPICIOUS_DOMAINS: Vec<&'static str> = vec![
        "onlyfans.com", "pornhub.com", "xvideos.com", "xnxx.com", "bit.ly", "tinyurl.com",
        "t.me/joinchat", "t.me/+", "bet365", "sportbet", "cassino", "adult", "porn"
    ];

    // Fix: this regex was being recompiled on every message before (`Regex::new`
    // inside `contains_suspicious_link`). Compiling a regex is expensive relative
    // to matching it, so under load this was a real, measurable cost per message.
    static ref URL_REGEX: Regex = Regex::new(r"(https?://[^\s]+)").unwrap();
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!(
        "🚀 BanHammer Multi-Idioma Iniciado! Default language: {}",
        match *DEFAULT_LANG {
            Lang::Pt => "pt",
            Lang::En => "en",
                Lang::Es => "es",

        }
    );

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(command_handler))
        .branch(dptree::entry().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let lang = lang_for_chat(chat_id);

    match cmd {
        Command::Help => {
            bot.send_message(chat_id, messages::help(lang))
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        Command::Status => {
            bot.send_message(chat_id, messages::status(lang)).await?;
        }
        Command::Language(arg) => {
            handle_language_command(&bot, &msg, lang, arg.trim()).await?;
        }
    }
    Ok(())
}

async fn handle_language_command(
    bot: &Bot,
    msg: &Message,
    lang: Lang,
    arg: &str,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(user) = msg.from() else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, user.id).await {
        bot.send_message(chat_id, messages::lang_no_permission(lang)).await?;
        return Ok(());
    }

    match Lang::from_code(arg) {
        Some(new_lang) => {
            set_lang_for_chat(chat_id, new_lang);
            bot.send_message(chat_id, messages::lang_set(new_lang)).await?;
        }
        None => {
            bot.send_message(chat_id, messages::lang_invalid(lang)).await?;
        }
    }

    Ok(())
}

async fn is_chat_admin(bot: &Bot, chat_id: ChatId, user_id: UserId) -> bool {
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(member) => matches!(
            member.kind,
            ChatMemberKind::Owner(_) | ChatMemberKind::Administrator(_)
        ),
        Err(e) => {
            log::warn!("Failed to fetch chat member {user_id} in {chat_id}: {e}");
            false
        }
    }
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user = match msg.from() {
        Some(u) => u,
        None => return Ok(()),
    };

    let text = msg.text().or_else(|| msg.caption())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    if is_violation(&text, &msg) {
        let lang = lang_for_chat(chat_id);
        handle_violation(&bot, &msg, user, lang).await?;
    }

    Ok(())
}

fn is_violation(text: &str, msg: &Message) -> bool {
    if text.is_empty() {
        return false;
    }

    // Verifica padrões de texto (PT + EN)
    if BAD_PATTERNS.iter().any(|re| re.is_match(text)) {
        return true;
    }

    // Verifica links suspeitos
    if contains_suspicious_link(text) {
        return true;
    }

    // Mensagens encaminhadas longas (comum em spam)
    if msg.forward_origin().is_some() && text.len() > 20 {
        return true;
    }

    false
}

fn contains_suspicious_link(text: &str) -> bool {
    for cap in URL_REGEX.captures_iter(text) {
        if let Ok(url) = Url::parse(&cap[1]) {
            let host = url.host_str().unwrap_or("").to_lowercase();
            if SUSPICIOUS_DOMAINS.iter().any(|&d| host.contains(d)) {
                return true;
            }
        }
    }
    false
}

async fn handle_violation(bot: &Bot, msg: &Message, user: &User, lang: Lang) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    bot.delete_message(chat_id, msg.id).await.ok();

    if let Err(e) = bot.ban_chat_member(chat_id, user.id).await {
        log::warn!("Falha ao banir {}: {}", user.id, e);
        bot.send_message(chat_id, messages::violation_generic(lang)).await.ok();
    } else {
        let username = user.username.as_deref().unwrap_or("user");
        bot.send_message(chat_id, messages::banned(lang, username)).await.ok();
        log::info!("Usuário {} banido por conteúdo proibido", user.id);
    }

    Ok(())
}