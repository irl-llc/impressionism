//! Database connection and initialization.

use anyhow::{Context, Result};
use std::path::Path;

use super::schema::CREATE_SCHEMA;

/// Database handle for impressionism storage.
pub struct Database {
    /// Path to the database file.
    path: std::path::PathBuf,
    // TODO: Add actual DuckDB connection when implementing queries
}

impl Database {
    /// Open or create a database at the given path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }

        Ok(Self { path })
    }

    /// Initialize the database schema.
    pub fn init_schema(&self) -> Result<()> {
        // TODO: Execute CREATE_SCHEMA using DuckDB connection
        // For now, just log that we would initialize
        tracing_stub("Initializing schema at: {}", &self.path);
        let _ = CREATE_SCHEMA; // Reference to avoid unused warning
        Ok(())
    }

    /// Get the database path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

// Temporary logging stub until we add proper logging
fn tracing_stub(msg: &str, path: &Path) {
    eprintln!("[impressionism] {} {:?}", msg, path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_open() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.db");

        let db = Database::open(&db_path).unwrap();
        assert_eq!(db.path(), db_path);
    }

    #[test]
    fn test_database_init_schema() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.db");

        let db = Database::open(&db_path).unwrap();
        db.init_schema().unwrap();
    }
}
