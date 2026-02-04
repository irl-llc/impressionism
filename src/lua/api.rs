//! Lua API bindings for the impressionism module.
//!
//! Provides functions accessible from Lua rulesets for skill activation logic.

use anyhow::Result;
use mlua::{Function, Lua, Table, Value};

/// Register the `impressionism` module in the Lua global namespace.
pub fn register_api(lua: &Lua) -> Result<()> {
    let impressionism = lua.create_table()?;

    impressionism.set("get_recent_messages", create_get_recent_messages(lua)?)?;
    impressionism.set("get_active_skills", create_get_active_skills(lua)?)?;
    impressionism.set("search_skills", create_search_skills(lua)?)?;
    impressionism.set("embed_text", create_embed_text(lua)?)?;
    impressionism.set("cosine_similarity", create_cosine_similarity(lua)?)?;
    impressionism.set("get_param", create_get_param(lua)?)?;
    impressionism.set("log", create_log(lua)?)?;

    lua.globals().set("impressionism", impressionism)?;
    Ok(())
}

fn create_get_recent_messages(lua: &Lua) -> Result<Function> {
    let func = lua.create_function(|lua, (session_id, count): (String, usize)| {
        // TODO: Implement actual message retrieval from storage
        // For now, return empty array
        let messages = lua.create_table()?;
        tracing_stub("get_recent_messages", &format!("{}, {}", session_id, count));
        Ok(messages)
    })?;
    Ok(func)
}

fn create_get_active_skills(lua: &Lua) -> Result<Function> {
    let func = lua.create_function(|lua, session_id: String| {
        // TODO: Implement actual skill retrieval from storage
        // For now, return empty array
        let skills = lua.create_table()?;
        tracing_stub("get_active_skills", &session_id);
        Ok(skills)
    })?;
    Ok(func)
}

fn create_search_skills(lua: &Lua) -> Result<Function> {
    let func = lua.create_function(|lua, (query, limit): (String, usize)| {
        // TODO: Implement actual vector search
        // For now, return empty array
        let results = lua.create_table()?;
        tracing_stub("search_skills", &format!("{}, {}", query, limit));
        Ok(results)
    })?;
    Ok(func)
}

fn create_embed_text(lua: &Lua) -> Result<Function> {
    let func = lua.create_function(|lua, text: String| {
        // TODO: Implement actual embedding generation
        // For now, return empty array (placeholder for 384-dim vector)
        let embedding = lua.create_table()?;
        tracing_stub("embed_text", &text);
        Ok(embedding)
    })?;
    Ok(func)
}

fn create_cosine_similarity(_lua: &Lua) -> Result<Function> {
    let func = _lua.create_function(|_lua, (vec_a, vec_b): (Table, Table)| {
        let a = table_to_vec(&vec_a)?;
        let b = table_to_vec(&vec_b)?;
        Ok(cosine_similarity(&a, &b))
    })?;
    Ok(func)
}

fn create_get_param(_lua: &Lua) -> Result<Function> {
    let func = _lua.create_function(|_lua, (name, default): (String, Value)| {
        // TODO: Implement parameter retrieval from config
        // For now, return the default value
        tracing_stub("get_param", &name);
        Ok(default)
    })?;
    Ok(func)
}

fn create_log(_lua: &Lua) -> Result<Function> {
    let func = _lua.create_function(|_lua, (level, message): (String, String)| {
        log_message(&level, &message);
        Ok(())
    })?;
    Ok(func)
}

/// Convert a Lua table (array) to a Vec<f64>.
fn table_to_vec(table: &Table) -> mlua::Result<Vec<f64>> {
    let mut vec = Vec::new();
    for pair in table.clone().pairs::<usize, f64>() {
        let (_, value) = pair?;
        vec.push(value);
    }
    Ok(vec)
}

/// Compute cosine similarity between two vectors.
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let magnitude_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Stub for tracing API calls (will be replaced with actual logging).
fn tracing_stub(func_name: &str, args: &str) {
    // In production, this would use proper logging
    eprintln!("[impressionism.{}] called with: {}", func_name, args);
}

/// Log a message at the specified level.
fn log_message(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "debug" => eprintln!("[DEBUG] {}", message),
        "info" => eprintln!("[INFO] {}", message),
        "warn" | "warning" => eprintln!("[WARN] {}", message),
        "error" => eprintln!("[ERROR] {}", message),
        _ => eprintln!("[{}] {}", level.to_uppercase(), message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![-1.0, -2.0, -3.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_empty() {
        let a: Vec<f64> = vec![];
        let b: Vec<f64> = vec![];
        let sim = cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_cosine_similarity_different_lengths() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_register_api() {
        let lua = Lua::new();
        register_api(&lua).unwrap();

        // Verify the impressionism table exists
        let impressionism: Table = lua.globals().get("impressionism").unwrap();

        // Verify all functions are registered
        assert!(impressionism.get::<Function>("get_recent_messages").is_ok());
        assert!(impressionism.get::<Function>("get_active_skills").is_ok());
        assert!(impressionism.get::<Function>("search_skills").is_ok());
        assert!(impressionism.get::<Function>("embed_text").is_ok());
        assert!(impressionism.get::<Function>("cosine_similarity").is_ok());
        assert!(impressionism.get::<Function>("get_param").is_ok());
        assert!(impressionism.get::<Function>("log").is_ok());
    }

    #[test]
    fn test_cosine_similarity_from_lua() {
        let lua = Lua::new();
        register_api(&lua).unwrap();

        let result: f64 = lua
            .load(r#"
                local a = {1.0, 2.0, 3.0}
                local b = {1.0, 2.0, 3.0}
                return impressionism.cosine_similarity(a, b)
            "#)
            .eval()
            .unwrap();

        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_get_param_returns_default() {
        let lua = Lua::new();
        register_api(&lua).unwrap();

        let result: i32 = lua
            .load(r#"
                return impressionism.get_param("unknown_param", 42)
            "#)
            .eval()
            .unwrap();

        assert_eq!(result, 42);
    }
}
