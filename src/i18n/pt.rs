pub fn help() -> String {
    format!(
        r#"🤖 *BotHammer v{}*

Detecta automaticamente:
- Pornografia
- Vendas / spam
- Apostas
- Pedofilia / CSAM
- Links suspeitos

Comandos:
/help — esta mensagem
/status — status do bot
/stats — estatísticas de moderação do grupo
/language <pt|en|es> — define o idioma do bot neste grupo (apenas administradores)
/reload — recarrega moderation.toml sem reiniciar o bot (apenas administradores)
/unban <user_id> — remove o banimento de um usuário (apenas administradores)"#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "✅ BotHammer está online e protegendo o grupo!";

pub const VIOLATION_GENERIC: &str = "🚫 Conteúdo proibido detectado e removido.";

pub fn BANNED(username: &str) -> String {
    format!("🚫 @{username} foi banido por violação das regras.")
}

pub const LANG_SET: &str = "✅ Idioma do bot definido para Português.";

pub const LANG_INVALID: &str =
    "⚠️ Idioma inválido. Use `/language pt`, `/language en` ou `/language es`.";

pub const LANG_NO_PERMISSION: &str = "⚠️ Apenas administradores podem alterar o idioma do bot.";

pub const STATS_TITLE: &str = "📊 *Estatísticas do grupo*";
pub const STATS_TOTAL: &str = "Violações totais";
pub const STATS_24H: &str = "Últimas 24h";
pub const STATS_BY_TYPE: &str = "Por categoria";
pub const STATS_TOP: &str = "Top infratores";
pub const STATS_EMPTY: &str = "✅ Nenhuma violação registrada neste grupo ainda.";

pub const RELOAD_SUCCESS: &str = "✅ Configuração de moderação recarregada com sucesso.";
pub const RELOAD_ERROR: &str =
    "⚠️ Falha ao recarregar moderation.toml. As regras antigas continuam ativas. Veja os logs do bot.";
pub const RELOAD_NO_PERMISSION: &str = "⚠️ Apenas administradores podem recarregar a configuração.";

pub const UNBAN_NO_PERMISSION: &str = "⚠️ Apenas administradores podem remover banimentos.";
pub const UNBAN_INVALID_ID: &str = "⚠️ Uso: `/unban <user_id>` — o ID precisa ser numérico.";

pub fn UNBAN_SUCCESS(user_id: u64) -> String {
    format!("✅ Usuário `{user_id}` foi desbanido.")
}

pub const UNBAN_ERROR: &str =
    "⚠️ Falha ao desbanir o usuário. Verifique se o ID está correto e se o bot tem permissão de admin.";
