//! Data models for storage layer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A skill indexed from the filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier (hash of path).
    pub id: String,
    /// Display name of the skill.
    pub name: String,
    /// Filesystem path to the skill.
    pub path: String,
    /// Short description of the skill.
    pub description: Option<String>,
    /// Embedding vector for similarity search.
    pub embedding: Vec<f32>,
    /// YAML frontmatter as JSON.
    pub frontmatter: Option<serde_json::Value>,
    /// Hash of the skill content for change detection.
    pub content_hash: String,
    /// When the skill was indexed.
    pub indexed_at: DateTime<Utc>,
    /// Source of the skill (user, project, plugin).
    pub source: SkillSource,
}

/// Source of a skill.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillSource {
    /// User's global skills directory.
    User,
    /// Project-specific skills.
    Project,
    /// Plugin-provided skills.
    Plugin,
}

/// File hash for incremental indexing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHash {
    /// Filesystem path.
    pub path: String,
    /// Hash of the file content.
    pub content_hash: String,
    /// When the file was last checked.
    pub last_checked: DateTime<Utc>,
}

/// A session with Claude Code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session identifier.
    pub session_id: String,
    /// Workspace path for this session.
    pub workspace_path: String,
    /// When the session started.
    pub started_at: DateTime<Utc>,
    /// When the session was last active.
    pub last_active: DateTime<Utc>,
}

/// A logged message in a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLog {
    /// Unique identifier.
    pub id: i64,
    /// Session this message belongs to.
    pub session_id: String,
    /// Order in the session.
    pub sequence: i32,
    /// Role (user, assistant, tool).
    pub role: MessageRole,
    /// Hook event that captured this.
    pub event_type: String,
    /// Tool name for tool use events.
    pub tool_name: Option<String>,
    /// Preview of the content.
    pub content_preview: Option<String>,
    /// Embedding of the content.
    pub content_embedding: Option<Vec<f32>>,
    /// Skills active at this point.
    pub active_skills: Option<serde_json::Value>,
    /// When the message was logged.
    pub logged_at: DateTime<Utc>,
}

/// Role of a message.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// User message.
    User,
    /// Assistant response.
    Assistant,
    /// Tool use.
    Tool,
}

/// A skill activated for a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSkill {
    /// Session identifier.
    pub session_id: String,
    /// Skill identifier.
    pub skill_id: String,
    /// When the skill was activated.
    pub activated_at: DateTime<Utc>,
    /// Reason for activation.
    pub activation_reason: Option<String>,
}
