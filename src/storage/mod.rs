//! Storage module for impressionism.
//!
//! Provides LanceDB + DuckDB storage layer for skills, sessions, and message history.

pub mod db;
pub mod models;
pub mod schema;

pub use db::Database;
pub use models::*;
