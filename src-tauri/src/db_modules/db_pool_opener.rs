use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;
use anyhow::Result;
use std::ops::Deref;

#[derive(Clone)]
pub struct ReadPool(Arc<SqlitePool>);

impl Deref for ReadPool {
    type Target = Arc<SqlitePool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<SqlitePool> for ReadPool {
    fn as_ref(&self) -> &SqlitePool {
        self.0.as_ref()
    }
}

#[derive(Clone)]
pub struct WritePool(Arc<SqlitePool>);

impl Deref for WritePool {
    type Target = Arc<SqlitePool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<SqlitePool> for WritePool {
    fn as_ref(&self) -> &SqlitePool {
        self.0.as_ref()
    }
}

pub async fn open_pools(app_data_dir: &std::path::Path) -> Result<(ReadPool, WritePool)> {
    let db_path = app_data_dir.join("db").join("budget-stats-gui.db");

    // Ensure the directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // --- 1. Configure options ---
    let write_opts = SqliteConnectOptions::from_str(&db_path.to_string_lossy())?
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(false);

    let read_opts = SqliteConnectOptions::from_str(&db_path.to_string_lossy())?
        .journal_mode(SqliteJournalMode::Wal)
        .read_only(true);

    // --- 2. Create pools with max connections ---
    let write_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(write_opts)
        .await?;

    let read_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(read_opts)
        .await?;

    // --- 3. Share pools with Arc ---
    let read_pool = ReadPool(Arc::new(read_pool));
    let write_pool = WritePool(Arc::new(write_pool));

    Ok((read_pool, write_pool))
}