use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommands};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Comandos do BanHammer")]
enum Command {
    #[command(description = "Mostrar ajuda")]
    Help,
    #[command(description = "Status do bot")]
    Status,
}

lazy_static! {
    static ref BAD_KEYWORDS: Vec<Regex> = vec![
        // Pornografia e conteúdo sexual
        Regex::new(r"(?i)(porn|sexo|puta|cu|piranha|boquete|anal|onlyfans|nudes|nsfw|hot|xxx|porno|putaria)").unwrap(),
        Regex::new(r"(?i)(foda|foder|transa|tesão|gozar|orgasmo|pênis|vagina|bunda|peito|pelada)").unwrap(),
        // Vendas / Spam comercial
        Regex::new(r"(?i)(compre|compre agora|promoção|desconto|oferta|barato|kg|unidade|entrega|frete|revenda)").unwrap(),
        // Apostas / BETs
        Regex::new(r"(?i)(bet|aposta|cassino|slot|roleta|fortuna|ganhe dinheiro|odds|aposte|bet365|sportbet)").unwrap(),
        // Pedofilia / CSAM
        Regex::new(r"(?i)(loli|shota|pedofilia|criança nua|underage|cp|child porn|menor de idade)").unwrap(),
        // Links suspeitos
        Regex::new(r"(?i)(onlyfans\.com|pornhub|xvideos|bit\.ly|tinyurl)").unwrap(),
    ];
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("🚀 BanHammer iniciado!");

    let bot = Bot::from_env(); // TELEGRAM_BOT_TOKEN

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
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, "🤖 *BanHammer* - Bot de moderação automática\n\nDetecta e bane automaticamente pornografia, apostas, vendas e pedofilia.").parse_mode(ParseMode::Markdown).await?;
        }
        Command::Status => {
            bot.send_message(msg.chat.id, "✅ BanHammer está online e protegendo o grupo!").await?;
        }
    }
    Ok(())
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let text = msg.text().or_else(|| msg.caption()).map(|s| s.to_lowercase());

    if let Some(content) = text {
        if BAD_KEYWORDS.iter().any(|re| re.is_match(&content)) {
            handle_violation(&bot, &msg).await?;
        }
    }

    Ok(())
}

async fn handle_violation(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user = match msg.from() {
        Some(u) => u,
        None => return Ok(()),
    };

    // Deletar mensagem
    bot.delete_message(chat_id, msg.id).await.ok();

    // Banir usuário
    if let Err(e) = bot.ban_chat_member(chat_id, user.id).await {
        log::warn!("Não consegui banir o usuário {}: {}", user.id, e);
        bot.send_message(chat_id, "🚫 Conteúdo proibido detectado e removido.").await.ok();
    } else {
        log::info!("Usuário {} banido por conteúdo proibido", user.id);
        bot.send_message(chat_id, format!("🚫 Usuário @{} banido por violação grave.", user.username.as_deref().unwrap_or("sem_username"))).await.ok();
    }

    Ok(())
}