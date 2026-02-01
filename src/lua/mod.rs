//! Lua rules engine module.
//!
//! Provides embedded Lua runtime for evaluating skill activation and deactivation rules.

pub mod engine;
pub mod sandbox;

pub use engine::LuaEngine;
