//! Ruleset loading and validation.
//!
//! Rulesets are Lua scripts that define skill activation and deactivation logic.
//! Each ruleset must export two functions:
//! - `evaluate_activation(context)` - returns skills to activate
//! - `evaluate_deactivation(context)` - returns skills to deactivate

use std::path::Path;

use anyhow::{bail, Context, Result};
use mlua::{Function, Lua, Table, Value};

/// Required function names that every ruleset must export.
const REQUIRED_FUNCTIONS: &[&str] = &["evaluate_activation", "evaluate_deactivation"];

/// A loaded and validated ruleset.
pub struct Ruleset {
    name: String,
    source: String,
}

impl Ruleset {
    /// Load a ruleset from a file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let name = extract_ruleset_name(path)?;
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read ruleset: {}", path.display()))?;

        Ok(Self { name, source })
    }

    /// Load a ruleset from a string with a given name.
    pub fn from_source(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
        }
    }

    /// Get the ruleset name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the ruleset source code.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Validate that the ruleset exports required functions.
    pub fn validate(&self, lua: &Lua) -> Result<()> {
        let chunk = lua
            .load(&self.source)
            .set_name(&self.name);

        let exports: Table = chunk
            .eval()
            .with_context(|| format!("Failed to evaluate ruleset '{}'", self.name))?;

        for func_name in REQUIRED_FUNCTIONS {
            validate_exported_function(&exports, func_name, &self.name)?;
        }

        Ok(())
    }

    /// Load the ruleset into a Lua state and return the exports table.
    pub fn load(&self, lua: &Lua) -> Result<Table> {
        let chunk = lua
            .load(&self.source)
            .set_name(&self.name);

        let exports: Table = chunk
            .eval()
            .with_context(|| format!("Failed to load ruleset '{}'", self.name))?;

        Ok(exports)
    }
}

fn extract_ruleset_name(path: &Path) -> Result<String> {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid ruleset path: {}", path.display()))?;

    Ok(stem.to_string())
}

fn validate_exported_function(exports: &Table, name: &str, ruleset_name: &str) -> Result<()> {
    match exports.get::<Value>(name)? {
        Value::Function(_) => Ok(()),
        Value::Nil => bail!(
            "Ruleset '{}' missing required function '{}'",
            ruleset_name,
            name
        ),
        other => bail!(
            "Ruleset '{}' exports '{}' as {} instead of function",
            ruleset_name,
            name,
            type_name(&other)
        ),
    }
}

fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Nil => "nil",
        Value::Boolean(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Table(_) => "table",
        Value::Function(_) => "function",
        Value::Thread(_) => "thread",
        Value::UserData(_) => "userdata",
        Value::LightUserData(_) => "lightuserdata",
        Value::Error(_) => "error",
        _ => "unknown",
    }
}

/// Context passed to ruleset evaluation functions.
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    /// Current session ID.
    pub session_id: String,
    /// Recent message content (for context).
    pub recent_message: Option<String>,
    /// Tool that was just used (for PostToolUse hook).
    pub tool_name: Option<String>,
    /// Hook that triggered evaluation.
    pub hook_type: HookType,
}

/// Types of hooks that can trigger evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    SessionStart,
    UserPromptSubmit,
    PostToolUse,
    Stop,
}

impl EvaluationContext {
    /// Convert context to a Lua table.
    pub fn to_lua_table(&self, lua: &Lua) -> Result<Table> {
        let table = lua.create_table()?;
        table.set("session_id", self.session_id.clone())?;

        if let Some(ref msg) = self.recent_message {
            table.set("recent_message", msg.clone())?;
        }

        if let Some(ref tool) = self.tool_name {
            table.set("tool_name", tool.clone())?;
        }

        table.set("hook_type", self.hook_type.as_str())?;

        Ok(table)
    }
}

impl HookType {
    fn as_str(&self) -> &'static str {
        match self {
            HookType::SessionStart => "session_start",
            HookType::UserPromptSubmit => "user_prompt_submit",
            HookType::PostToolUse => "post_tool_use",
            HookType::Stop => "stop",
        }
    }
}

/// Evaluate activation rules and return skill names to activate.
pub fn evaluate_activation(
    lua: &Lua,
    exports: &Table,
    context: &EvaluationContext,
) -> Result<Vec<String>> {
    let func: Function = exports.get("evaluate_activation")?;
    let ctx_table = context.to_lua_table(lua)?;
    let result: Table = func.call(ctx_table)?;

    table_to_string_vec(&result)
}

/// Evaluate deactivation rules and return skill names to deactivate.
pub fn evaluate_deactivation(
    lua: &Lua,
    exports: &Table,
    context: &EvaluationContext,
) -> Result<Vec<String>> {
    let func: Function = exports.get("evaluate_deactivation")?;
    let ctx_table = context.to_lua_table(lua)?;
    let result: Table = func.call(ctx_table)?;

    table_to_string_vec(&result)
}

fn table_to_string_vec(table: &Table) -> Result<Vec<String>> {
    let mut vec = Vec::new();
    for pair in table.clone().pairs::<usize, String>() {
        let (_, value) = pair?;
        vec.push(value);
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_RULESET: &str = r#"
        return {
            evaluate_activation = function(context)
                return {}
            end,
            evaluate_deactivation = function(context)
                return {}
            end
        }
    "#;

    const MISSING_FUNCTION_RULESET: &str = r#"
        return {
            evaluate_activation = function(context)
                return {}
            end
            -- Missing evaluate_deactivation
        }
    "#;

    const WRONG_TYPE_RULESET: &str = r#"
        return {
            evaluate_activation = "not a function",
            evaluate_deactivation = function(context)
                return {}
            end
        }
    "#;

    #[test]
    fn test_valid_ruleset() {
        let lua = Lua::new();
        let ruleset = Ruleset::from_source("test", VALID_RULESET);
        assert!(ruleset.validate(&lua).is_ok());
    }

    #[test]
    fn test_missing_function() {
        let lua = Lua::new();
        let ruleset = Ruleset::from_source("test", MISSING_FUNCTION_RULESET);
        let result = ruleset.validate(&lua);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing required function"));
    }

    #[test]
    fn test_wrong_type() {
        let lua = Lua::new();
        let ruleset = Ruleset::from_source("test", WRONG_TYPE_RULESET);
        let result = ruleset.validate(&lua);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("instead of function"));
    }

    #[test]
    fn test_evaluate_activation() {
        let lua = Lua::new();
        let ruleset_code = r#"
            return {
                evaluate_activation = function(context)
                    if context.hook_type == "session_start" then
                        return {"skill_a", "skill_b"}
                    end
                    return {}
                end,
                evaluate_deactivation = function(context)
                    return {}
                end
            }
        "#;

        let ruleset = Ruleset::from_source("test", ruleset_code);
        let exports = ruleset.load(&lua).unwrap();

        let context = EvaluationContext {
            session_id: "test-session".to_string(),
            recent_message: None,
            tool_name: None,
            hook_type: HookType::SessionStart,
        };

        let skills = evaluate_activation(&lua, &exports, &context).unwrap();
        assert_eq!(skills, vec!["skill_a", "skill_b"]);
    }

    #[test]
    fn test_context_to_lua() {
        let lua = Lua::new();
        let context = EvaluationContext {
            session_id: "sess-123".to_string(),
            recent_message: Some("Hello".to_string()),
            tool_name: Some("Read".to_string()),
            hook_type: HookType::PostToolUse,
        };

        let table = context.to_lua_table(&lua).unwrap();

        assert_eq!(table.get::<String>("session_id").unwrap(), "sess-123");
        assert_eq!(table.get::<String>("recent_message").unwrap(), "Hello");
        assert_eq!(table.get::<String>("tool_name").unwrap(), "Read");
        assert_eq!(table.get::<String>("hook_type").unwrap(), "post_tool_use");
    }
}
