/// Escapa caracteres reservados do Telegram MarkdownV2 (username,
/// violation_type e qualquer outro texto dinâmico) para não quebrar
/// a formatação — ou o envio — da mensagem.
pub fn escape_markdown_v2(text: &str) -> String {
    const RESERVED: &[char] = &[
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    let mut escaped = String::with_capacity(text.len());

    for ch in text.chars() {
        if RESERVED.contains(&ch) {
            escaped.push('\\');
        }

        escaped.push(ch);
    }

    escaped
}
