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
