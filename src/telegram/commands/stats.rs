use teloxide::{
    prelude::*,
    types::ParseMode,
};

use crate::{
    core::state::AppState,
    i18n::{Lang, messages},
    storage::sqlite,
    telegram::utils::markdown::escape_markdown_v2,
};

/// `/stats` — monta e envia a mensagem de estatísticas do grupo.
pub async fn handle(bot: &Bot, msg: &Message, state: &AppState, lang: Lang) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let labels = messages::stats_labels(lang);

    let stats = match sqlite::get_chat_stats(&state.db, chat_id.0).await {
        Ok(stats) => stats,

        Err(error) => {
            log::warn!(
                "Falha ao buscar estatísticas do chat {}: {}",
                chat_id,
                error
            );

            bot.send_message(chat_id, messages::violation_generic(lang))
                .await
                .ok();

            return Ok(());
        }
    };

    if stats.total == 0 {
        bot.send_message(chat_id, labels.empty).await?;

        return Ok(());
    }

    let mut text = format!(
        "{}\n\n{}: {}\n{}: {}\n\n*{}:*\n",
        labels.title, labels.total, stats.total, labels.last_24h, stats.last_24h, labels.by_type,
    );

    for (violation_type, count) in &stats.by_type {
        text.push_str(&format!(
            "• {}: {count}\n",
            escape_markdown_v2(violation_type)
        ));
    }

    if !stats.top_offenders.is_empty() {
        text.push_str(&format!("\n*{}:*\n", labels.top));

        for (user_id, username, count) in &stats.top_offenders {
            match username {
                Some(name) => {
                    text.push_str(&format!("• @{} — {count}\n", escape_markdown_v2(name)));
                }

                None => {
                    text.push_str(&format!("• `{user_id}` — {count}\n"));
                }
            }
        }
    }

    bot.send_message(chat_id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
