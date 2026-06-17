-- Regex core schema (v1)

PRAGMA foreign_keys = ON;

CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    brief TEXT NOT NULL DEFAULT '',
    format TEXT NOT NULL CHECK (format IN ('video', 'article', 'other')),
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL,
    shipped_at TEXT
);

CREATE TABLE items (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT,
    platform TEXT NOT NULL CHECK (platform IN ('instagram', 'x', 'tiktok', 'youtube', 'web', 'manual')),
    title TEXT NOT NULL DEFAULT '',
    author TEXT NOT NULL DEFAULT '',
    note TEXT NOT NULL DEFAULT '',
    thumbnail_path TEXT,
    status TEXT NOT NULL DEFAULT 'inbox' CHECK (
        status IN ('inbox', 'brewing', 'ready', 'producing', 'shipped', 'archived')
    ),
    project_id TEXT REFERENCES projects (id) ON DELETE SET NULL,
    captured_at TEXT NOT NULL,
    captured_on TEXT NOT NULL CHECK (captured_on IN ('desktop', 'mobile'))
);

CREATE UNIQUE INDEX idx_items_url ON items (url) WHERE url IS NOT NULL AND url != '';

CREATE TABLE tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE item_tags (
    item_id TEXT NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, tag_id)
);

CREATE TABLE transcripts (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    lang TEXT NOT NULL DEFAULT 'en',
    text TEXT NOT NULL DEFAULT '',
    source TEXT NOT NULL CHECK (source IN ('auto', 'whisper')),
    created_at TEXT NOT NULL
);

CREATE TABLE breakdowns (
    id TEXT PRIMARY KEY NOT NULL,
    item_id TEXT NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    project_id TEXT REFERENCES projects (id) ON DELETE SET NULL,
    status TEXT NOT NULL CHECK (status IN ('queued', 'running', 'done', 'failed')),
    error TEXT,
    created_at TEXT NOT NULL,
    finished_at TEXT
);

CREATE TABLE assets (
    id TEXT PRIMARY KEY NOT NULL,
    breakdown_id TEXT REFERENCES breakdowns (id) ON DELETE SET NULL,
    project_id TEXT REFERENCES projects (id) ON DELETE SET NULL,
    item_id TEXT REFERENCES items (id) ON DELETE CASCADE,
    type TEXT NOT NULL CHECK (
        type IN ('frame', 'clip', 'audio', 'transcript', 'structure')
    ),
    path TEXT NOT NULL,
    start_ms INTEGER,
    end_ms INTEGER,
    label TEXT NOT NULL DEFAULT '',
    meta TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE TABLE item_references (
    idea_id TEXT NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    reference_id TEXT NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    PRIMARY KEY (idea_id, reference_id),
    CHECK (idea_id != reference_id)
);

-- ---------------------------------------------------------------------------
-- FTS5 search index
-- ---------------------------------------------------------------------------

CREATE VIRTUAL TABLE search_index USING fts5 (
    entity_type UNINDEXED,
    entity_id UNINDEXED,
    item_id UNINDEXED,
    body,
    tokenize = 'porter unicode61'
);

CREATE TRIGGER search_items_ai AFTER INSERT ON items BEGIN
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('item', NEW.id, NEW.id, trim(NEW.note || ' ' || NEW.title || ' ' || NEW.author));
END;

CREATE TRIGGER search_items_ad AFTER DELETE ON items BEGIN
    DELETE FROM search_index WHERE entity_type = 'item' AND entity_id = OLD.id;
END;

CREATE TRIGGER search_items_au AFTER UPDATE ON items BEGIN
    DELETE FROM search_index WHERE entity_type = 'item' AND entity_id = OLD.id;
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('item', NEW.id, NEW.id, trim(NEW.note || ' ' || NEW.title || ' ' || NEW.author));
END;

CREATE TRIGGER search_transcripts_ai AFTER INSERT ON transcripts BEGIN
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('transcript', NEW.id, NEW.item_id, NEW.text);
END;

CREATE TRIGGER search_transcripts_ad AFTER DELETE ON transcripts BEGIN
    DELETE FROM search_index WHERE entity_type = 'transcript' AND entity_id = OLD.id;
END;

CREATE TRIGGER search_transcripts_au AFTER UPDATE ON transcripts BEGIN
    DELETE FROM search_index WHERE entity_type = 'transcript' AND entity_id = OLD.id;
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('transcript', NEW.id, NEW.item_id, NEW.text);
END;

CREATE TRIGGER search_assets_ai AFTER INSERT ON assets BEGIN
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('asset', NEW.id, NEW.item_id, NEW.label);
END;

CREATE TRIGGER search_assets_ad AFTER DELETE ON assets BEGIN
    DELETE FROM search_index WHERE entity_type = 'asset' AND entity_id = OLD.id;
END;

CREATE TRIGGER search_assets_au AFTER UPDATE ON assets BEGIN
    DELETE FROM search_index WHERE entity_type = 'asset' AND entity_id = OLD.id;
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES ('asset', NEW.id, NEW.item_id, NEW.label);
END;
