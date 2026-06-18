-- Free-form thinking space for ideas in develop.
ALTER TABLE items ADD COLUMN develop_note TEXT NOT NULL DEFAULT '';

-- Rebuild item search triggers so develop notes are searchable too.
DROP TRIGGER IF EXISTS search_items_ai;
DROP TRIGGER IF EXISTS search_items_ad;
DROP TRIGGER IF EXISTS search_items_au;

CREATE TRIGGER search_items_ai AFTER INSERT ON items BEGIN
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES (
        'item',
        NEW.id,
        NEW.id,
        trim(NEW.note || ' ' || NEW.develop_note || ' ' || NEW.title || ' ' || NEW.author)
    );
END;

CREATE TRIGGER search_items_ad AFTER DELETE ON items BEGIN
    DELETE FROM search_index WHERE entity_type = 'item' AND entity_id = OLD.id;
END;

CREATE TRIGGER search_items_au AFTER UPDATE ON items BEGIN
    DELETE FROM search_index WHERE entity_type = 'item' AND entity_id = OLD.id;
    INSERT INTO search_index (entity_type, entity_id, item_id, body)
    VALUES (
        'item',
        NEW.id,
        NEW.id,
        trim(NEW.note || ' ' || NEW.develop_note || ' ' || NEW.title || ' ' || NEW.author)
    );
END;
