use teloxide::{dispatching::Dispatcher, prelude::*};

use crate::core::state::AppState;

use super::{
    commands::Command,
    handlers::{command_handler, message_handler},
};

/// Inicializa o dispatcher do Telegram.
///
/// Responsabilidades:
///
/// - registrar handlers;
/// - injetar dependências globais;
/// - iniciar o loop principal do bot.
pub async fn run(bot: Bot, state: AppState) {
    log::info!("Starting Telegram dispatcher...");

    let command_branch = dptree::entry()
        .filter_command::<Command>()
        .endpoint(command_handler);

    let message_branch = dptree::entry().endpoint(message_handler);

    let handler = dptree::entry().filter_async(dedupe_update).branch(
        Update::filter_message()
            .branch(command_branch)
            .branch(message_branch),
    );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

/// Evita processar o mesmo Update do Telegram mais de uma vez.
///
/// Long polling pode reentregar o mesmo Update em cenários de
/// reconexão/retry; sem essa checagem o bot poderia, por exemplo,
/// banir duas vezes em resposta ao mesmo evento.
async fn dedupe_update(update: Update, state: AppState) -> bool {
    if state.memory.was_update_processed(update.id).await {
        log::debug!("Update {} já processado, ignorando", update.id);

        return false;
    }

    state.memory.mark_update_processed(update.id).await;

    true
}
