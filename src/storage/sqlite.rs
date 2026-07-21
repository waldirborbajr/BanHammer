use sqlx::{
    Row,
    sqlite::{SqlitePool, SqlitePoolOptions},
};

use std::time::Duration;

/// Inicializa conexão com SQLite.
///
/// Exemplo:
/// sqlite://bothammer.db
pub async fn init_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    create_tables(&pool).await?;

    Ok(pool)
}

/// Criação das tabelas necessárias.
async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS chats (
            chat_id INTEGER PRIMARY KEY,
            language TEXT NOT NULL DEFAULT 'pt',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY,
            username TEXT,
            first_seen DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS violations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            chat_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            violation_type TEXT NOT NULL,
            message TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS blocked_domains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            domain TEXT UNIQUE NOT NULL,
            enabled INTEGER DEFAULT 1
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Salva ou atualiza idioma do grupo.
#[allow(dead_code)]
pub async fn set_chat_language(
    pool: &SqlitePool,
    chat_id: i64,
    language: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO chats(chat_id, language)
        VALUES (?, ?)

        ON CONFLICT(chat_id)
        DO UPDATE SET language = excluded.language;
        "#,
    )
    .bind(chat_id)
    .bind(language)
    .execute(pool)
    .await?;

    Ok(())
}

/// Recupera idioma configurado do grupo.
#[allow(dead_code)]
pub async fn get_chat_language(
    pool: &SqlitePool,
    chat_id: i64,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query(
        r#"
            SELECT language
            FROM chats
            WHERE chat_id = ?
            "#,
    )
    .bind(chat_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|row| row.get::<String, _>("language")))
}

/// Registra uma violação.
pub async fn insert_violation(
    pool: &SqlitePool,
    chat_id: i64,
    user_id: i64,
    violation_type: &str,
    message: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO violations(
            chat_id,
            user_id,
            violation_type,
            message
        )
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(chat_id)
    .bind(user_id)
    .bind(violation_type)
    .bind(message)
    .execute(pool)
    .await?;

    Ok(())
}

/// Cria ou atualiza o registro de um usuário (user_id + username).
///
/// Usa `COALESCE` para não sobrescrever um username já conhecido
/// com `NULL` caso o Telegram não informe o username naquele evento
/// (ex: usuário sem username público).
pub async fn upsert_user(
    pool: &SqlitePool,
    user_id: i64,
    username: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users(user_id, username)
        VALUES (?, ?)

        ON CONFLICT(user_id)
        DO UPDATE SET username = COALESCE(excluded.username, users.username);
        "#,
    )
    .bind(user_id)
    .bind(username)
    .execute(pool)
    .await?;

    Ok(())
}

/// Adiciona domínio bloqueado.
pub async fn add_blocked_domain(pool: &SqlitePool, domain: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO blocked_domains(domain)
        VALUES(?)
        "#,
    )
    .bind(domain)
    .execute(pool)
    .await?;

    Ok(())
}

/// Lista domínios bloqueados.
pub async fn get_blocked_domains(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
            SELECT domain
            FROM blocked_domains
            WHERE enabled = 1
            "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| row.get::<String, _>("domain"))
        .collect())
}

/// Estatísticas agregadas de violações de um chat.
#[derive(Debug, Default)]
pub struct ChatStats {
    /// Total de violações já registradas no chat.
    pub total: i64,

    /// Violações nas últimas 24 horas.
    pub last_24h: i64,

    /// Contagem por categoria (violation_type, count), ordenado desc.
    pub by_type: Vec<(String, i64)>,

    /// Top 5 usuários com mais violações (user_id, username opcional, count).
    pub top_offenders: Vec<(i64, Option<String>, i64)>,
}

/// Monta as estatísticas de um chat a partir da tabela `violations`,
/// com o username resolvido via LEFT JOIN em `users` quando disponível.
pub async fn get_chat_stats(pool: &SqlitePool, chat_id: i64) -> Result<ChatStats, sqlx::Error> {
    let total: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as c
        FROM violations
        WHERE chat_id = ?
        "#,
    )
    .bind(chat_id)
    .fetch_one(pool)
    .await?
    .get("c");

    let last_24h: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as c
        FROM violations
        WHERE chat_id = ?
          AND created_at >= datetime('now', '-1 day')
        "#,
    )
    .bind(chat_id)
    .fetch_one(pool)
    .await?
    .get("c");

    let by_type_rows = sqlx::query(
        r#"
        SELECT violation_type, COUNT(*) as c
        FROM violations
        WHERE chat_id = ?
        GROUP BY violation_type
        ORDER BY c DESC
        "#,
    )
    .bind(chat_id)
    .fetch_all(pool)
    .await?;

    let by_type = by_type_rows
        .into_iter()
        .map(|row| {
            (
                row.get::<String, _>("violation_type"),
                row.get::<i64, _>("c"),
            )
        })
        .collect();

    let top_rows = sqlx::query(
        r#"
        SELECT v.user_id AS user_id, u.username AS username, COUNT(*) as c
        FROM violations v
        LEFT JOIN users u ON u.user_id = v.user_id
        WHERE v.chat_id = ?
        GROUP BY v.user_id
        ORDER BY c DESC
        LIMIT 5
        "#,
    )
    .bind(chat_id)
    .fetch_all(pool)
    .await?;

    let top_offenders = top_rows
        .into_iter()
        .map(|row| {
            (
                row.get::<i64, _>("user_id"),
                row.get::<Option<String>, _>("username"),
                row.get::<i64, _>("c"),
            )
        })
        .collect();

    Ok(ChatStats {
        total,
        last_24h,
        by_type,
        top_offenders,
    })
}
