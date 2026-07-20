use teloxide::{
    dispatching::Dispatcher,
    prelude::*,
};

use super::{
    commands::Command,
    handlers::{
        command_handler,
        message_handler,
    },
};



/// Inicializa o dispatcher do Telegram
pub async fn run(bot: Bot) {


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


        // Processa mensagens normais
        // texto, links, spam, conteúdo proibido
        .branch(
            dptree::entry()
                .endpoint(message_handler),
        );



    Dispatcher::builder(
        bot,
        handler,
    )

    // Permite finalizar com CTRL+C
    .enable_ctrlc_handler()


    .build()

    .dispatch()

    .await;
}