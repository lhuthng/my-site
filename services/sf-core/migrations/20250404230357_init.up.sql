-- Create types
CREATE TYPE entity_type AS ENUM ('character', 'item');
CREATE TYPE character_class AS ENUM ('warrior', 'mage', 'archer');
CREATE TYPE item_type AS ENUM (
    'weapon', 'helmet', 'robe', 'gloves', 'boots', 
    'necklace', 'ring', 'amulet', 'belt', 'potion'
);
CREATE TYPE equipment_type AS ENUM (
    'weapon', 'helmet', 'robe', 'gloves', 'boots', 
    'necklace', 'ring', 'amulet', 'belt'
);

-- Create tables
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
    "entity_id" INT REFERENCES entities(id),
    "int_points" INT DEFAULT 0,
    "str_points" INT DEFAULT 0,
    "dex_points" INT DEFAULT 0,
    "lck_points" INT DEFAULT 0,
    "con_points" INT DEFAULT 0
);

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "description" TEXT,
    "item_type" item_type NOT NULL,
    "entity_id" INT REFERENCES entities(id),
    "armor_points" INT DEFAULT 0,
    "price" INT DEFAULT 0,
    "quantity" INT DEFAULT 1
);

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    "user_id" INT REFERENCES users(id),
    "class" character_class NOT NULL,
    "entity_id" INT REFERENCES entities(id),
    "name" VARCHAR(255) UNIQUE NOT NULL,
    "level" INT DEFAULT 1,
    "gold" INT DEFAULT 0,
    "exp" INT DEFAULT 0,
    "mushroom" INT DEFAULT 0,
    "inventory_capacity" INT DEFAULT 6
);

CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    "character_id" INT REFERENCES characters(id) ON DELETE CASCADE,
    "type" equipment_type NOT NULL,
    "entity_id" INT REFERENCES entities(id),
    CONSTRAINT unique_equipment_slot UNIQUE ("character_id", "type")
);

CREATE TABLE inventories (
    id SERIAL PRIMARY KEY,
    "character_id" INT REFERENCES characters(id) ON DELETE CASCADE,
    "item_id" INT REFERENCES items(id),
    "slot_number" INT NOT NULL,
    CONSTRAINT unique_inventory_slot UNIQUE ("character_id", "item_id", "slot_number")
);

-- Create functions
CREATE OR REPLACE FUNCTION add_new_user(
    p_external_id VARCHAR(255),
    p_username VARCHAR(255)
)
RETURNS VOID AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM users WHERE external_id = p_external_id) THEN
        RETURN;
    END IF;

    INSERT INTO users (external_id, username)
    VALUES (p_external_id, p_username);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_new_character(
    p_user_id INT,
    p_class character_class,
    p_name VARCHAR(255)
)
RETURNS VOID AS $$
DECLARE
    p_entity_id INT;
    p_character_id INT;
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE id = p_user_id) THEN
        RAISE NOTICE 'User with id % does not exist', p_user_id;
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
    INSERT INTO inventories ("character_id", "item_id", "slot_number")
    VALUES (p_character_id, NULL, 0); -- Set item_id to NULL and slot_number to a default value

    -- Insert default attributes for the character
    INSERT INTO attributes ("int_points", "str_points", "dex_points", "lck_points", "con_points", "entity_id")
    VALUES (10, 10, 10, 10, 10, p_entity_id);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_new_item(
    p_name VARCHAR(255),
    p_description TEXT,
    p_item_type item_type,
    p_slot_number INT,
    p_price INT,
    p_quantity INT,
    p_str_points INT,
    p_int_points INT,
    p_dex_points INT,
    p_lck_points INT,
    p_con_points INT,
    p_armor_points INT
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
    INSERT INTO items ("name", "description", "item_type", "entity_id", "price", "quantity", "armor_points")
    VALUES (p_name, p_description, p_item_type, p_entity_id, p_price, p_quantity, p_armor_points)
    RETURNING id INTO p_item_id;

    -- Insert attributes for the item
    INSERT INTO attributes ("int_points", "str_points", "dex_points", "lck_points", "con_points", "entity_id")
    VALUES (p_int_points, p_str_points, p_dex_points, p_lck_points, p_con_points, p_entity_id);
END;
$$ LANGUAGE plpgsql;