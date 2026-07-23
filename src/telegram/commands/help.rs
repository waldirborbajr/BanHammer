use teloxide::prelude::*;

use crate::i18n::{Lang, messages};

/// `/help` — exibe a lista de comandos disponíveis.
pub async fn handle(bot: &Bot, chat_id: ChatId, lang: Lang) -> ResponseResult<()> {
    bot.send_message(chat_id, messages::help(lang)).await?;

    Ok(())
}
