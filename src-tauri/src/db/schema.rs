//! Database schema definitions.

/// Initial database schema - Version 1.
pub const INIT_SCHEMA: &str = r#"
-- Repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    remote_url TEXT,
    default_branch TEXT,
    last_scanned_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_repositories_path ON repositories(path);
CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT DEFAULT '#808080',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Repository-Tag relationship (many-to-many)
CREATE TABLE IF NOT EXISTS repository_tags (
    repo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (repo_id, tag_id),
    FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Scan paths configuration
CREATE TABLE IF NOT EXISTS scan_paths (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    enabled INTEGER DEFAULT 1,
    max_depth INTEGER DEFAULT 5,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Application settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value) VALUES 
    ('schema_version', '1'),
    ('theme', 'system'),
    ('auto_scan', 'true'),
    ('scan_interval_minutes', '30');

-- Triggers for updated_at
CREATE TRIGGER IF NOT EXISTS update_repositories_timestamp 
    AFTER UPDATE ON repositories
BEGIN
    UPDATE repositories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_settings_timestamp 
    AFTER UPDATE ON settings
BEGIN
    UPDATE settings SET updated_at = CURRENT_TIMESTAMP WHERE key = NEW.key;
END;
"#;
