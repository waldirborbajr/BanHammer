use teloxide::{
    prelude::*,
    types::{ChatId, ChatMemberKind, UserId},
};

/// Verifica se um usuário é administrador ou dono do grupo
pub async fn is_chat_admin(bot: &Bot, chat_id: ChatId, user_id: UserId) -> bool {
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(member) => {
            matches!(
                member.kind,
                ChatMemberKind::Owner(_) | ChatMemberKind::Administrator(_)
            )
        }

        Err(error) => {
            log::warn!(
                "Falha ao consultar administrador {} no chat {}: {}",
                user_id,
                chat_id,
                error
            );

            false
        }
    }
}
