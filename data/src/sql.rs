use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;

#[tokio::main]
pub async fn database() -> Result<(), sqlx::Error > {
    let env = &std::env::var("DATABASE_URL");

    let mut conn = SqliteConnectOptions::from_str("sqlite://data.db")?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .connect().await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reactions (
                internal_name TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                result TEXT NOT NULL,
                mix_phrase TEXT NOT NULL,
                required_temp FLOAT,
                instant BOOLEAN,
                hidden BOOLEAN,
                recipes INT NOT NULL
            );"
        )
        .execute(&mut conn)
        .await?;
    
    sqlx::query(
        "INSERT INTO reactions
            (internal_name, name, result, mix_phrase, required_temp, instant, hidden, recipes)
            VALUES (1, 2, 3, 4, 5, 6, 7, 8);
            "
        )
        .execute(&mut conn)
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn connect(result: Result<(), sqlx::Error >) { 
    println!("{:?}", result)
}
