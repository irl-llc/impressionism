//! Lua rules engine module.
//!
//! Provides embedded Lua runtime for evaluating skill activation and deactivation rules.

pub mod api;
pub mod engine;
pub mod ruleset;
pub mod sandbox;

pub use api::register_api;
pub use engine::LuaEngine;
pub use ruleset::{EvaluationContext, HookType, Ruleset};
