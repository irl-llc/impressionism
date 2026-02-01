//! Database schema definitions.

/// SQL statements to create the database schema.
pub const CREATE_SCHEMA: &str = r#"
-- Skill index with embeddings
CREATE TABLE IF NOT EXISTS skill_index (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    description TEXT,
    embedding FLOAT[384],
    frontmatter JSON,
    content_hash VARCHAR NOT NULL,
    indexed_at TIMESTAMP NOT NULL,
    source VARCHAR NOT NULL
);

-- File hash tracking for incremental indexing
CREATE TABLE IF NOT EXISTS file_hashes (
    path VARCHAR PRIMARY KEY,
    content_hash VARCHAR NOT NULL,
    last_checked TIMESTAMP NOT NULL
);

-- Session tracking
CREATE TABLE IF NOT EXISTS sessions (
    session_id VARCHAR PRIMARY KEY,
    workspace_path VARCHAR NOT NULL,
    started_at TIMESTAMP NOT NULL,
    last_active TIMESTAMP NOT NULL
);

-- Message log for rule evaluation
CREATE TABLE IF NOT EXISTS message_log (
    id INTEGER PRIMARY KEY,
    session_id VARCHAR NOT NULL,
    sequence INTEGER NOT NULL,
    role VARCHAR NOT NULL,
    event_type VARCHAR NOT NULL,
    tool_name VARCHAR,
    content_preview TEXT,
    content_embedding FLOAT[384],
    active_skills JSON,
    logged_at TIMESTAMP NOT NULL,
    UNIQUE(session_id, sequence)
);

-- Active skills per session
CREATE TABLE IF NOT EXISTS session_skills (
    session_id VARCHAR NOT NULL,
    skill_id VARCHAR NOT NULL,
    activated_at TIMESTAMP NOT NULL,
    activation_reason TEXT,
    PRIMARY KEY (session_id, skill_id)
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_message_log_session ON message_log(session_id);
CREATE INDEX IF NOT EXISTS idx_session_skills_session ON session_skills(session_id);
"#;
