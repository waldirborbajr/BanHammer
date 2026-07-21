//! Internacionalização do BanHammer.
//!
//! Organização:
//!
//! - `lang`: definição dos idiomas suportados.
//! - `manager`: gerenciamento do idioma por chat.
//! - `messages`: interface para obtenção das mensagens traduzidas.
//! - `pt`, `en`, `es`: traduções.

pub mod en;
pub mod es;
pub mod lang;
pub mod manager;
pub mod messages;
pub mod pt;

pub use lang::Lang;
pub use manager::LanguageManager;
pub use messages::Messages;
