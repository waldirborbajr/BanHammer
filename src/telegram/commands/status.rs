use teloxide::prelude::*;

use crate::i18n::{Lang, messages};

/// `/status` — exibe o status atual do bot.
pub async fn handle(bot: &Bot, chat_id: ChatId, lang: Lang) -> ResponseResult<()> {
    bot.send_message(chat_id, messages::status(lang)).await?;

    Ok(())
}
