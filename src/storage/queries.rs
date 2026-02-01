//! Database query operations.
//!
//! CRUD operations for all storage tables.

use anyhow::Result;
use chrono::{DateTime, Utc};

use super::models::*;
use super::Database;

impl Database {
    // ========== Skill Operations ==========

    /// Insert or update a skill.
    pub fn upsert_skill(&self, skill: &Skill) -> Result<()> {
        // TODO: Implement with DuckDB
        let _ = skill;
        Ok(())
    }

    /// Get a skill by ID.
    pub fn get_skill(&self, id: &str) -> Result<Option<Skill>> {
        // TODO: Implement with DuckDB
        let _ = id;
        Ok(None)
    }

    /// List all skills.
    pub fn list_skills(&self) -> Result<Vec<Skill>> {
        // TODO: Implement with DuckDB
        Ok(Vec::new())
    }

    /// Delete a skill by ID.
    pub fn delete_skill(&self, id: &str) -> Result<bool> {
        // TODO: Implement with DuckDB
        let _ = id;
        Ok(false)
    }

    /// Search skills by embedding similarity.
    pub fn search_skills(&self, embedding: &[f32], limit: usize) -> Result<Vec<(Skill, f32)>> {
        // TODO: Implement with LanceDB vector search
        let _ = (embedding, limit);
        Ok(Vec::new())
    }

    // ========== File Hash Operations ==========

    /// Insert or update a file hash.
    pub fn upsert_file_hash(&self, hash: &FileHash) -> Result<()> {
        // TODO: Implement with DuckDB
        let _ = hash;
        Ok(())
    }

    /// Get a file hash by path.
    pub fn get_file_hash(&self, path: &str) -> Result<Option<FileHash>> {
        // TODO: Implement with DuckDB
        let _ = path;
        Ok(None)
    }

    /// Delete a file hash by path.
    pub fn delete_file_hash(&self, path: &str) -> Result<bool> {
        // TODO: Implement with DuckDB
        let _ = path;
        Ok(false)
    }

    // ========== Session Operations ==========

    /// Insert or update a session.
    pub fn upsert_session(&self, session: &Session) -> Result<()> {
        // TODO: Implement with DuckDB
        let _ = session;
        Ok(())
    }

    /// Get a session by ID.
    pub fn get_session(&self, session_id: &str) -> Result<Option<Session>> {
        // TODO: Implement with DuckDB
        let _ = session_id;
        Ok(None)
    }

    /// Update session last_active timestamp.
    pub fn touch_session(&self, session_id: &str) -> Result<bool> {
        // TODO: Implement with DuckDB
        let _ = session_id;
        Ok(false)
    }

    // ========== Message Log Operations ==========

    /// Insert a message log entry.
    pub fn insert_message(&self, message: &MessageLog) -> Result<i64> {
        // TODO: Implement with DuckDB
        let _ = message;
        Ok(0)
    }

    /// Get recent messages for a session.
    pub fn get_recent_messages(&self, session_id: &str, limit: usize) -> Result<Vec<MessageLog>> {
        // TODO: Implement with DuckDB
        let _ = (session_id, limit);
        Ok(Vec::new())
    }

    /// Get messages in a time range.
    pub fn get_messages_in_range(
        &self,
        session_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MessageLog>> {
        // TODO: Implement with DuckDB
        let _ = (session_id, start, end);
        Ok(Vec::new())
    }

    // ========== Session Skills Operations ==========

    /// Activate a skill for a session.
    pub fn activate_skill(&self, session_skill: &SessionSkill) -> Result<()> {
        // TODO: Implement with DuckDB
        let _ = session_skill;
        Ok(())
    }

    /// Deactivate a skill for a session.
    pub fn deactivate_skill(&self, session_id: &str, skill_id: &str) -> Result<bool> {
        // TODO: Implement with DuckDB
        let _ = (session_id, skill_id);
        Ok(false)
    }

    /// Get active skills for a session.
    pub fn get_active_skills(&self, session_id: &str) -> Result<Vec<SessionSkill>> {
        // TODO: Implement with DuckDB
        let _ = session_id;
        Ok(Vec::new())
    }

    /// Deactivate all skills for a session.
    pub fn deactivate_all_skills(&self, session_id: &str) -> Result<usize> {
        // TODO: Implement with DuckDB
        let _ = session_id;
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_db() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.db");
        let db = Database::open(&db_path).unwrap();
        db.init_schema().unwrap();
        (temp, db)
    }

    #[test]
    fn test_skill_crud() {
        let (_temp, db) = setup_db();

        // Insert
        let skill = Skill {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            path: "/path/to/skill".to_string(),
            description: Some("A test skill".to_string()),
            embedding: vec![0.1; 384],
            frontmatter: None,
            content_hash: "abc123".to_string(),
            indexed_at: Utc::now(),
            source: SkillSource::User,
        };
        db.upsert_skill(&skill).unwrap();

        // Get (stub returns None for now)
        let result = db.get_skill("test-skill").unwrap();
        assert!(result.is_none()); // Will be Some after implementation

        // List (stub returns empty for now)
        let skills = db.list_skills().unwrap();
        assert!(skills.is_empty()); // Will have items after implementation
    }

    #[test]
    fn test_session_crud() {
        let (_temp, db) = setup_db();

        let session = Session {
            session_id: "test-session".to_string(),
            workspace_path: "/path/to/workspace".to_string(),
            started_at: Utc::now(),
            last_active: Utc::now(),
        };
        db.upsert_session(&session).unwrap();

        let result = db.get_session("test-session").unwrap();
        assert!(result.is_none()); // Will be Some after implementation
    }

    #[test]
    fn test_message_log_crud() {
        let (_temp, db) = setup_db();

        let message = MessageLog {
            id: 0,
            session_id: "test-session".to_string(),
            sequence: 1,
            role: MessageRole::User,
            event_type: "UserPromptSubmit".to_string(),
            tool_name: None,
            content_preview: Some("Test message".to_string()),
            content_embedding: None,
            active_skills: None,
            logged_at: Utc::now(),
        };
        let id = db.insert_message(&message).unwrap();
        assert_eq!(id, 0); // Stub returns 0

        let messages = db.get_recent_messages("test-session", 10).unwrap();
        assert!(messages.is_empty()); // Will have items after implementation
    }

    #[test]
    fn test_session_skills_crud() {
        let (_temp, db) = setup_db();

        let session_skill = SessionSkill {
            session_id: "test-session".to_string(),
            skill_id: "test-skill".to_string(),
            activated_at: Utc::now(),
            activation_reason: Some("similarity=0.85".to_string()),
        };
        db.activate_skill(&session_skill).unwrap();

        let active = db.get_active_skills("test-session").unwrap();
        assert!(active.is_empty()); // Will have items after implementation
    }
}
