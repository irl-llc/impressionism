//! Lua sandbox configuration.
//!
//! Restricts the Lua environment for security.

use anyhow::Result;
use mlua::{Lua, Value};

/// List of globals to remove for security.
const BLOCKED_GLOBALS: &[&str] = &[
    "os",
    "io",
    "debug",
    "loadfile",
    "dofile",
    "load",
    "loadstring",
    "rawget",
    "rawset",
    "rawequal",
    "collectgarbage",
    "getfenv",
    "setfenv",
    "newproxy",
    "getmetatable",
    "setmetatable",
];

/// Apply sandbox restrictions to a Lua state.
pub fn apply_sandbox(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // Remove dangerous globals
    for name in BLOCKED_GLOBALS {
        globals.set(*name, Value::Nil)?;
    }

    // Restrict require to only allow safe modules
    let safe_require = lua.create_function(|_lua, module: String| {
        // Only allow requiring builtin modules (will be extended for rulesets)
        if module.starts_with("builtin.") || module.starts_with("custom.") {
            // For now, return nil - will be implemented with ruleset loading
            Ok(Value::Nil)
        } else {
            Err(mlua::Error::runtime(format!(
                "require '{}' is not allowed in sandbox",
                module
            )))
        }
    })?;
    globals.set("require", safe_require)?;

    Ok(())
}

/// Check if a Lua state has sandbox applied.
pub fn is_sandboxed(lua: &Lua) -> bool {
    let globals = lua.globals();

    // Check that blocked globals are nil
    for name in BLOCKED_GLOBALS {
        match globals.get::<Value>(*name) {
            Ok(Value::Nil) => continue,
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_blocks_os() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: Value = lua.globals().get("os").unwrap();
        assert!(matches!(result, Value::Nil));
    }

    #[test]
    fn test_sandbox_blocks_io() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: Value = lua.globals().get("io").unwrap();
        assert!(matches!(result, Value::Nil));
    }

    #[test]
    fn test_sandbox_blocks_debug() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: Value = lua.globals().get("debug").unwrap();
        assert!(matches!(result, Value::Nil));
    }

    #[test]
    fn test_sandbox_allows_string() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: String = lua.load("return string.upper('hello')").eval().unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_sandbox_allows_table() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: i32 = lua
            .load("local t = {1,2,3}; return #t")
            .eval()
            .unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sandbox_allows_math() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result: f64 = lua.load("return math.sqrt(16)").eval().unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_require_blocked_for_arbitrary_modules() {
        let lua = Lua::new();
        apply_sandbox(&lua).unwrap();

        let result = lua.load("require('socket')").exec();
        assert!(result.is_err());
    }

    #[test]
    fn test_is_sandboxed() {
        let lua = Lua::new();
        assert!(!is_sandboxed(&lua)); // Not sandboxed yet

        apply_sandbox(&lua).unwrap();
        assert!(is_sandboxed(&lua)); // Now sandboxed
    }
}
