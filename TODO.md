# TODO — BanHammer

Roadmap de features sugeridas, organizadas por área. Não estão em ordem de prioridade fixa — cada item tem uma nota de esforço/impacto pra ajudar a decidir por onde começar.

---

## Governança e ações graduais

- [ ] **Sistema de strikes configurável** (aviso → mute → kick → ban), em vez de ban direto para toda violação.
  Hoje qualquer violação bane na hora — rígido demais para spam/gambling (severidade baixa), correto para csam/pornografia.
  _Esforço: médio · Impacto: alto_

- [x] **Comando `/unban` ou `/appeal`** para admin reverter um banimento incorreto sem sair do Telegram.
  _Esforço: baixo · Impacto: médio_

- [ ] **Whitelist de usuários confiáveis** — membros antigos sem histórico de violação recebem checagem mais branda.
  _Esforço: médio · Impacto: médio_

---

## Persistência que já existe mas está pela metade

- [ ] **Popular a tabela `users`** (schema já existe em `sqlite.rs`, mas nunca é escrita).
  Permitiria mostrar `@username` no `/stats` em vez de só `user_id` cru.
  _Esforço: baixo · Impacto: médio_ — **bom primeiro passo**, infra já pronta.

- [ ] **Usar `blocked_domains` de verdade** (tabela e funções `add_blocked_domain`/`get_blocked_domains` já existem, nada chama).
  Comando `/blockdomain <dominio>` para admins adicionarem na hora, sem editar TOML e reiniciar.
  _Esforço: baixo · Impacto: médio_

- [x] **Comando `/reload`** — recarrega `moderation.toml` em runtime sem reiniciar o processo.
  Faz sentido agora que a config é externa (ver histórico de migração keyword/domain → TOML).
  _Esforço: baixo · Impacto: médio_

---

## Observabilidade / operação

- [ ] **Endpoint Prometheus (`/metrics`)** via um mini servidor HTTP (`axum`/`warp`) rodando junto do bot.
  Contagem de violações, latência de análise, mensagens processadas.
  _Esforço: médio · Impacto: médio_

- [ ] **Modo dry-run** via variável de ambiente — só loga o que baniria, sem agir.
  Ótimo para testar regra nova em produção sem risco.
  _Esforço: baixo · Impacto: alto (segurança operacional)_

- [ ] **Rate limiting de mensagens por usuário** (flood / repetição) — hoje só cobre palavra-chave e link, não volume.
  _Esforço: médio · Impacto: médio_

---

## Detecção mais robusta

- [ ] **Normalização anti-evasão mais forte** em `normalize_text` (leetspeak, espaçamento entre letras, caracteres Unicode visualmente parecidos).
  Contorna o truque mais comum de burlar filtro de texto.
  _Esforço: médio · Impacto: alto_

- [ ] **Verificação de imagem via hash perceptual** contra bases conhecidas.
  Hoje o bot só analisa texto/legenda — imagem passa batido.
  _Esforço: alto · Impacto: alto_

- [ ] **Checagem de link malicioso/phishing** via Google Safe Browsing API (separada da lista estática de domínio suspeito).
  _Esforço: médio · Impacto: médio_

---

## Multi-tenant de verdade

- [ ] **Override de regras por grupo** (ex: `rules.chat_overrides[chat_id]`) — hoje `moderation.toml` é global para o bot inteiro.
  _Esforço: alto · Impacto: médio_

- [ ] **Severidade configurável por grupo** — ex: um grupo bane direto, outro só deleta e avisa.
  _Esforço: médio · Impacto: médio_

---

## Notas de manutenção (não-feature)

- [x] Migrar keywords/domínios de `gambling`, `pornography`, `spam`, `links` para `config/moderation.toml`.
- [x] Manter `csam.rs` fixo no binário, sem depender de config externa.
- [x] Validar no boot que nenhuma seção do `moderation.toml` está vazia (`ModerationRules::validate`).
- [x] Registrar violações no SQLite (`insert_violation`) e expor via `/stats`.
- [x] Corrigir `Cargo.toml` (`[alias]` movido para `.cargo/config.toml`, `rust-version` adicionado, feature `macros` do sqlx removida por estar sem uso).
- [ ] Rodar `cargo check` / `cargo clippy --all-targets --all-features -- -D warnings` localmente antes de cada release — o ambiente de review não tem toolchain compatível com edition 2024 para validar compilação.
