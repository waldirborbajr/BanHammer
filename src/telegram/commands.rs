use teloxide::utils::command::BotCommands;


/// Comandos disponíveis no BanHammer
#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "BanHammer Commands"
)]
pub enum Command {


    /// Exibe ajuda do bot
    #[command(
        description = "Mostrar ajuda / Show help / Mostrar ayuda"
    )]
    Help,



    /// Mostra o status do bot
    #[command(
        description = "Status do bot / Bot status / Estado del bot"
    )]
    Status,



    /// Altera o idioma do grupo
    #[command(
        description = "Idioma: pt|en|es / Language: pt|en|es"
    )]
    Language(String),
}