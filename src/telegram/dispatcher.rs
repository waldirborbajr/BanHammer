use teloxide::{dispatching::Dispatcher, prelude::*};

use crate::core::state::AppState;

use super::{
    commands::Command,
    handlers::{command_handler, message_handler},
};

/// Inicializa o dispatcher do Telegram
pub async fn run(bot: Bot, state: AppState) {
    let handler = Update::filter_message()
        // Processa comandos:
        // /help
        // /status
        // /language
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        // Processa mensagens normais:
        // texto, links, spam, conteúdo proibido
        .branch(dptree::entry().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        // Injeta dependências globais nos handlers
        .dependencies(dptree::deps![state])
        // Permite finalizar com CTRL+C
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
