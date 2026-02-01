//! Lua engine for rule evaluation.

use anyhow::{Context, Result};
use mlua::{Lua, Value};

use super::sandbox::apply_sandbox;

/// Lua engine for evaluating skill activation rules.
pub struct LuaEngine {
    lua: Lua,
}

impl LuaEngine {
    /// Create a new Lua engine with sandbox applied.
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        apply_sandbox(&lua)?;
        Ok(Self { lua })
    }

    /// Create a new Lua engine without sandbox (for testing only).
    #[cfg(test)]
    pub fn new_unsandboxed() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// Evaluate a Lua expression and return the result as a string.
    pub fn eval<S: AsRef<str>>(&self, code: S) -> Result<String> {
        let result: Value = self
            .lua
            .load(code.as_ref())
            .eval()
            .context("Failed to evaluate Lua code")?;

        Ok(format_value(&result))
    }

    /// Execute Lua code without returning a value.
    pub fn exec<S: AsRef<str>>(&self, code: S) -> Result<()> {
        self.lua
            .load(code.as_ref())
            .exec()
            .context("Failed to execute Lua code")?;
        Ok(())
    }

    /// Load a Lua chunk and return it for later execution.
    pub fn load_chunk<S: AsRef<str>>(&self, code: S) -> Result<mlua::Function> {
        self.lua
            .load(code.as_ref())
            .into_function()
            .context("Failed to load Lua chunk")
    }

    /// Get a global value.
    pub fn get_global<S: AsRef<str>>(&self, name: S) -> Result<Value> {
        self.lua
            .globals()
            .get(name.as_ref())
            .context("Failed to get global")
    }

    /// Set a global value.
    pub fn set_global<S: AsRef<str>>(&self, name: S, value: Value) -> Result<()> {
        self.lua
            .globals()
            .set(name.as_ref(), value)
            .context("Failed to set global")
    }

    /// Get access to the underlying Lua state (for API registration).
    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}

impl Default for LuaEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default LuaEngine")
    }
}

/// Format a Lua value as a string for display.
fn format_value(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.to_str().map(|s| s.to_string()).unwrap_or_else(|_| "<invalid utf8>".to_string()),
        Value::Table(_) => "<table>".to_string(),
        Value::Function(_) => "<function>".to_string(),
        Value::Thread(_) => "<thread>".to_string(),
        Value::UserData(_) => "<userdata>".to_string(),
        Value::LightUserData(_) => "<lightuserdata>".to_string(),
        Value::Error(e) => format!("<error: {}>", e),
        _ => "<unknown>".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = LuaEngine::new().unwrap();
        assert!(engine.lua().globals().get::<Value>("print").is_ok());
    }

    #[test]
    fn test_eval_expression() {
        let engine = LuaEngine::new_unsandboxed().unwrap();
        let result = engine.eval("return 1 + 2").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_eval_string() {
        let engine = LuaEngine::new_unsandboxed().unwrap();
        let result = engine.eval("return 'hello'").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_exec() {
        let engine = LuaEngine::new_unsandboxed().unwrap();
        engine.exec("x = 42").unwrap();
        let result = engine.eval("return x").unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_globals() {
        let engine = LuaEngine::new_unsandboxed().unwrap();
        engine.exec("my_var = 'test'").unwrap();
        let value = engine.get_global("my_var").unwrap();
        assert_eq!(format_value(&value), "test");
    }
}
