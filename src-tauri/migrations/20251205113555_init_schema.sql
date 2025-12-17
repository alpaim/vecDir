CREATE TABLE IF NOT EXISTS app_config (
    key TEXT PRIMARY KEY NOT NULL,
    config JSON NOT NULL DEFAULT "{}"
);

CREATE TABLE IF NOT EXISTS spaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    
    llm_config JSON NOT NULL,
    embedding_config JSON NOT NULL, 
    
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS indexed_roots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    space_id INTEGER NOT NULL,            -- link to spaces
    path TEXT UNIQUE NOT NULL,            -- abosolute path 
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    status TEXT DEFAULT "active",         -- "active", "paused"

    FOREIGN KEY(space_id) REFERENCES spaces(id) ON DELETE CASCADE,
    UNIQUE(space_id, path)
);

CREATE TABLE IF NOT EXISTS files_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT, 
    root_id INTEGER NOT NULL,             -- root_id from indexed_roots
    absolute_path TEXT UNIQUE NOT NULL,   -- absolute path to the file
    filename TEXT NOT NULL,               -- filename (maybe I can get it from absolute_path instead) but it is useful for SQL queries                 
    file_extension TEXT NOT NULL,         -- file extension (same to filename, possible to get it from absolute path)
    
    file_size INTEGER NOT NULL,
    
    description TEXT,                     -- file description from LLM 

    -- Timestamps for incremental indexation
    modified_at_fs DATETIME NOT NULL,     -- Date of last file modification on filesystem  (mtime)
    last_indexed_at DATETIME,             -- Last indexed time. NULL if there were no indexations.
    
    content_hash TEXT,                    -- Optional HASH of the file

    -- Processing status
    indexing_status TEXT DEFAULT "pending", -- "pending", "indexed", "failed", "stale"
    indexing_error_message TEXT,          -- Adds error message if it fails to process

    FOREIGN KEY(root_id) REFERENCES indexed_roots(id) ON DELETE CASCADE,
    UNIQUE(root_id, absolute_path)
);

CREATE INDEX idx_files_path ON files_metadata(absolute_path);
CREATE INDEX idx_files_status ON files_metadata(indexing_status);
CREATE INDEX idx_files_ext ON files_metadata(file_extension);

CREATE TABLE IF NOT EXISTS file_chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    
    chunk_index INTEGER NOT NULL,  -- index of chunk in file
    content TEXT NOT NULL,         -- text part
    
    -- optional for syntax highlighting
    start_char_idx INTEGER,
    end_char_idx INTEGER,

    FOREIGN KEY(file_id) REFERENCES files_metadata(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_chunks_file_id ON file_chunks(file_id);

-- VECTORS

CREATE VIRTUAL TABLE IF NOT EXISTS vec_chunks USING vec0(
    embedding float[768] distance_metric=cosine      -- currently ONLY 768 
);

CREATE TRIGGER IF NOT EXISTS delete_vec_chunk
AFTER DELETE ON file_chunks
BEGIN
  DELETE FROM vec_chunks WHERE rowid = OLD.id;
END;