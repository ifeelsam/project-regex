-- Track which idea graduated into each project.
ALTER TABLE projects ADD COLUMN primary_item_id TEXT REFERENCES items (id) ON DELETE SET NULL;
