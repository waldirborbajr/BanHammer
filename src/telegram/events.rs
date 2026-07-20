use teloxide::types::Message;


/// Tipos de eventos recebidos pelo Telegram
#[derive(Debug)]
pub enum TelegramEvent {

    /// Mensagem de texto normal
    Text {
        content: String,
    },


    /// Legenda de mídia
    Caption {
        content: String,
    },


    /// Mensagem encaminhada
    Forwarded {
        content: String,
    },


    /// Mensagem sem conteúdo textual analisável
    Empty,
}



impl TelegramEvent {


    /// Converte uma mensagem Telegram em um evento interno
    pub fn from_message(
        msg: &Message,
    ) -> Self {


        // Texto da mensagem
        if let Some(text) = msg.text() {

            return Self::Text {
                content: text.to_lowercase(),
            };
        }


        // Legenda de foto/vídeo/documento
        if let Some(caption) = msg.caption() {

            return Self::Caption {
                content: caption.to_lowercase(),
            };
        }


        // Mensagens encaminhadas
        if msg.forward_origin().is_some() {

            return Self::Forwarded {
                content: String::new(),
            };
        }


        Self::Empty
    }



    /// Retorna o texto analisável do evento
    pub fn content(
        &self,
    ) -> Option<&str> {

        match self {

            Self::Text { content }
            | Self::Caption { content }
            | Self::Forwarded { content } => {

                if content.is_empty() {
                    None
                } else {
                    Some(content)
                }
            }


            Self::Empty => None,
        }
    }



    /// Indica se a mensagem veio encaminhada
    pub fn is_forwarded(
        &self,
    ) -> bool {

        matches!(
            self,
            Self::Forwarded { .. }
        )
    }
}