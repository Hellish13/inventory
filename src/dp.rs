use rusqlite::{Connection, Result, params};
use anyhow::{anyhow, Context};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> anyhow::Result<Self> {
        let conn = Connection::open("write_off_system.db")?;
        Ok(Database { conn })
    }

    pub fn init(&self) -> anyhow::Result<()> {
        // Читаем миграции из файла
        let migration_sql = include_str!("../../migrations/001_initial.sql");
        
        self.conn.execute_batch(migration_sql)
            .context("Failed to execute migrations")?;
        
        println!("Database initialized successfully");
        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn begin_transaction(&self) -> anyhow::Result<()> {
        self.conn.execute("BEGIN TRANSACTION", [])
            .context("Failed to begin transaction")?;
        Ok(())
    }

    pub fn commit_transaction(&self) -> anyhow::Result<()> {
        self.conn.execute("COMMIT", [])
            .context("Failed to commit transaction")?;
        Ok(())
    }

    pub fn rollback_transaction(&self) -> anyhow::Result<()> {
        self.conn.execute("ROLLBACK", [])
            .context("Failed to rollback transaction")?;
        Ok(())
    }
}