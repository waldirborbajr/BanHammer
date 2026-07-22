pub mod en;
pub mod es;
pub mod lang;
pub mod manager;
pub mod messages;
pub mod pt;

pub use lang::Lang;
pub use manager::LanguageManager;
// removido: pub use messages::Messages;  (não existe — módulo usa funções livres)
