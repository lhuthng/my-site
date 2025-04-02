CREATE TYPE entity_type AS ENUM ('character', 'item');

CREATE TABLE entities (
    id SERIAL PRIMARY KEY,
    "type" entity_type NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    "external_id" VARCHAR(255) NOT NULL UNIQUE,
    "last_update" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "username" VARCHAR(255) NOT NULL
);

CREATE TABLE attributes (
    id SERIAL PRIMARY KEY,
    "int_point" INT DEFAULT 0,
    "str_point" INT DEFAULT 0,
    "dex_point" INT DEFAULT 0,
    "lck_point" INT DEFAULT 0,
    "con_point" INT DEFAULT 0,
    "entity_id" INT REFERENCES entities(id)
);

CREATE TYPE character_class AS ENUM ('warrior', 'mage', 'archer');

CREATE TYPE item_type AS ENUM ('weapon', 'helmet', 'robe', 'gloves', 'boots', 'necklace', 'ring', 'amulet', 'belt');

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    "user_id" INT REFERENCES users(id),
    "class" character_class NOT NULL,
    "entity_id" INT REFERENCES entities(id),
    "name" VARCHAR(255) UNIQUE NOT NULL,
    "level" INT DEFAULT 1,
    "gold" INT DEFAULT 0,
    "exp" INT DEFAULT 0,
    "mushroom" INT DEFAULT 0
);

CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    "character_id" INT REFERENCES characters(id) ON DELETE CASCADE,
    "capacity" INT DEFAULT 6
);

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "description" TEXT,
    "item_type" item_type NOT NULL,
    "inventory_id" INT REFERENCES inventory(id),
    "entity_id" INT REFERENCES entities(id),
    "slot_number" INT,
    "armor_point" INT DEFAULT 0,
    "price" INT DEFAULT 0,
    "quantity" INT DEFAULT 1,
    CONSTRAINT unique_slot UNIQUE (inventory_id, slot_number)
);

CREATE OR REPLACE FUNCTION add_new_user(
    p_external_id VARCHAR(255),
    p_username VARCHAR(255)
)
RETURNS VOID AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM users WHERE external_id= p_external_id) THEN
        RETURN;
    END IF;

    -- Insert the new user
    INSERT INTO users (external_id, username)
    VALUES (p_external_id, p_username);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_new_character(
    p_user_id INT REFERENCES users(id),
    p_class character_class,
    p_name VARCHAR(255)
)
RETURNS VOID AS $$
DECLARE
    p_entity_id INT;
    p_character_id INT;
    p_inventory_id INT;
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE id = p_user_id) THEN
        RAISE NOTICE 'User with external_id % already exists', p_user_id;
    END IF;

    -- Insert a new entity
    INSERT INTO entities ("type")
    VALUES ('character')
    RETURNING id INTO p_entity_id;

    -- Insert a new character
    INSERT INTO characters ("user_id", "class", "entity_id", "name")
    VALUES (p_user_id, p_class, p_entity_id, p_name)
    RETURNING id INTO p_character_id;

    -- Insert a new inventory for the character
    INSERT INTO inventory ("character_id")
    VALUES (p_character_id)
    RETURNING id INTO p_inventory_id;

    -- Insert default attributes for the character
    INSERT INTO attributes ("int_point", "str_point", "dex_point", "lck_point", "con_point", "entity_id")
    VALUES (10, 10, 10, 10, 10, p_entity_id);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_new_item(
    p_name VARCHAR(255),
    p_description TEXT,
    p_item_type item_type,
    p_inventory_id INT REFERENCES inventory(id),
    p_slot_number INT,
    p_price INT,
    p_quantity INT,
    p_str_point INT,
    p_int_point INT,
    p_dex_point INT,
    p_lck_point INT,
    p_con_point INT,
    p_armor_point INT
)
RETURNS VOID AS $$
DECLARE
    p_entity_id INT;
    p_item_id INT;
BEGIN
    -- Insert a new entity
    INSERT INTO entities ("type")
    VALUES ('item')
    RETURNING id INTO p_entity_id;

    -- Insert a new item
    INSERT INTO items ("name", "description", "item_type", "inventory_id", "entity_id", "slot_number", "price", "quantity", "armor_point")
    VALUES (p_name, p_description, p_item_type, p_inventory_id, p_entity_id, p_slot_number, p_price, p_quantity, p_armor_point)
    RETURNING id INTO p_item_id;

    -- Insert attributes for the item
    INSERT INTO attributes ("int_point", "str_point", "dex_point", "lck_point", "con_point", "entity_id")
    VALUES (p_int_point, p_str_point, p_dex_point, p_lck_point, p_con_point, p_entity_id);
END;
$$ LANGUAGE plpgsql;