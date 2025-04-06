-- Create types
CREATE TYPE "entity_type" AS ENUM (
    'character', 'item'
);
CREATE TYPE "character_class" AS ENUM ('warrior', 'mage', 'archer');
CREATE TYPE "item_type" AS ENUM (
    'equipable', 'potion'
);
CREATE TYPE "equipment_type" AS ENUM (
    'weapon', 'helmet', 'robe', 'gloves', 'boots', 
    'necklace', 'ring', 'amulet', 'belt'
);
CREATE TYPE "resource_type" AS ENUM (
    'gold', 'mushroom'
);
CREATE TYPE "container_type" AS ENUM (
    'inventory', 'shop', 'quest', 'other'
);

-- Create tables
CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "external_id" VARCHAR(255) NOT NULL UNIQUE,
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "last_update" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "entities" (
    "id" SERIAL PRIMARY KEY,
    "type" "entity_type" NOT NULL
);

CREATE TABLE "attributes" (
    "id" INT PRIMARY KEY REFERENCES "entities"("id"),
    "int_points" INT DEFAULT 0,
    "str_points" INT DEFAULT 0,
    "dex_points" INT DEFAULT 0,
    "lck_points" INT DEFAULT 0,
    "con_points" INT DEFAULT 0,
    "armor_points" INT DEFAULT 0
);

CREATE TABLE "items" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "item_type" "item_type" NOT NULL,
    "description" TEXT,
    "price" INT DEFAULT 0
);

CREATE TABLE "equipable_items" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id"),
    "entity_id" INT REFERENCES "entities"("id") UNIQUE,
    "equipment_type" "equipment_type" NOT NULL
);

CREATE TABLE "potions" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id"),
    "quantity" INT DEFAULT 1
);

CREATE TABLE "characters" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INT REFERENCES "users"("id"),
    "entity_id" INT REFERENCES "entities"("id") UNIQUE,
    "class" "character_class" NOT NULL,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "level" INT DEFAULT 1,
    "exp" INT DEFAULT 0
);

CREATE TABLE "resources" (
    "character_id" INT PRIMARY KEY REFERENCES "characters"("id"),
    "type" "resource_type" NOT NULL,
    "amount" INT DEFAULT 0,
    CONSTRAINT "unique_resource" UNIQUE ("character_id", "type")
);

CREATE TABLE "equipment" (
    "item_id" INT PRIMARY KEY REFERENCES "equipable_items"("item_id"),
    "character_id" INT REFERENCES "characters"("id") ON DELETE CASCADE,
    "type" "equipment_type" NOT NULL,
    CONSTRAINT "unique_equipment_slot" UNIQUE ("character_id", "type")
);

CREATE TABLE "containers" (
    "id" SERIAL PRIMARY KEY,
    "character_id" INT REFERENCES "characters"("id"),
    "container_type" "container_type" NOT NULL,
    "capacity" INT DEFAULT 1,
    CONSTRAINT "unique_container" UNIQUE ("character_id", "container_type")
);

CREATE TABLE "item_locations" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id"),
    "container_id" INT REFERENCES "containers"("id"),
    "slot_number" INT DEFAULT 1,
    CONSTRAINT "unique_item_slot" UNIQUE ("container_id", "slot_number")
);

-- Create functions
CREATE OR REPLACE FUNCTION "add_user"(
    p_external_id VARCHAR(255),
    p_username VARCHAR(255)
)
RETURNS VOID AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM "users" WHERE "external_id" = p_external_id) THEN
        RETURN;
    END IF;

    INSERT INTO "users" ("external_id", "username")
    VALUES (p_external_id, p_username);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_character"(
    p_user_id INT,
    p_class "character_class",
    p_name VARCHAR(255)
)
RETURNS VOID AS $$
DECLARE
    p_entity_id INT;
    p_character_id INT;
BEGIN
    -- Insert a new entity
    INSERT INTO "entities" ("type")
    VALUES ('character')
    RETURNING "id" INTO p_entity_id;

    -- Insert a new character
    INSERT INTO "characters" (
        "user_id", "entity_id", "class", 
        "name"
    )
    VALUES (
        p_user_id, p_entity_id, p_class, 
        p_name
    )
    RETURNING "id" INTO p_character_id;

    -- Insert a new inventory for the character
    INSERT INTO "containers" (
        "character_id", "container_type", "capacity"
    )
    VALUES (
        p_character_id, 'inventory', 6
    );

    -- Insert a new shop for the character
    INSERT INTO "containers" (
        "character_id", "container_type", "capacity"
    )
    VALUES (
        p_character_id, 'shop', 6
    );

    -- Insert default attributes for the character
    INSERT INTO "attributes" (
        "id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points",
        "armor_points"
    )
    VALUES (
        p_entity_id, 10, 10, 
        10, 10, 10,
        0
    );
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_item"(
    p_name VARCHAR(255),
    p_description TEXT,
    p_item_type "item_type",
    p_price INT,
    p_quantity INT
)
RETURNS INTEGER AS $$
DECLARE
    p_item_id INT;
BEGIN
    -- Insert a new item
    INSERT INTO "items" (
        "name", "item_type", "description",
        "price"
    )
    VALUES (
        p_name, p_item_type, p_description, 
        p_price
    )
    RETURNING "id" INTO p_item_id;

    RETURN p_item_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_equipable_item"(
    p_name VARCHAR(255),
    p_description TEXT,
    p_price INT,
    p_equipment_type "equipment_type",
    p_str_points INT,
    p_dex_points INT,
    p_int_points INT,
    p_con_points INT,
    p_lck_points INT,
    p_armor_points INT
)
RETURNS VOID AS $$
DECLARE
    p_entity_id INT;
    p_item_id INT;
BEGIN
    -- Insert a new entity
    INSERT INTO "entities" ("type")
    VALUES ('item')
    RETURNING "id" INTO p_entity_id;

    -- Insert a new item
    SELECT "add_item"(
        p_name, p_description, 'equipable', 
        p_price, 1
    ) INTO p_item_id;

    -- Insert a new equipable item
    INSERT INTO "equipable_items" (
        "item_id", "entity_id", "equipment_type"
    )
    VALUES (
        p_item_id, p_entity_id, p_equipment_type
    );

    -- Insert attributes for the item
    INSERT INTO "attributes" (
        "id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points",
        "armor_points"
    )
    VALUES (
        p_entity_id, p_str_points, p_dex_points, 
        p_int_points, p_con_points, p_lck_points,
        p_armor_points
    );
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "move_item" (
    p_item_id INT,
    p_container_id INT,
    p_slot_number INT,
    p_swappable BOOLEAN DEFAULT FALSE
)
RETURNS VOID AS $$
DECLARE
    existing_item_id INT;
    current_container_id INT;
    current_slot_number INT;
BEGIN
    -- Check if the slot number is valid
    IF p_slot_number < 1 OR p_slot_number > (
            SELECT "capacity" FROM "containers" WHERE "containers"."id" = p_container_id
        ) 
    THEN
        RAISE EXCEPTION 'Invalid slot number: %', p_slot_number;
    END IF;

    -- Check if any item exists in the target slot
    SELECT "item_id" INTO existing_item_id
    FROM "item_locations"
    WHERE 
        "container_id" = p_container_id 
        AND "slot_number" = p_slot_number;

    IF 
        existing_item_id IS NULL OR existing_item_id = p_item_id
    THEN
        -- Update the new location for the item
        INSERT INTO "item_locations" (
            "item_id", "container_id", "slot_number"
        )
        VALUES (
            p_item_id, p_container_id, p_slot_number
        )
        ON CONFLICT ("item_id")
        DO UPDATE SET
            "container_id" = EXCLUDED."container_id",
            "slot_number" = EXCLUDED."slot_number";
    ELSIF
        p_swappable
        AND EXISTS (
            SELECT 1 FROM "item_locations" 
            WHERE 
                "item_id" = p_item_id
                AND "container_id" = p_container_id
        )
    THEN

        SELECT "slot_number"
        INTO current_slot_number
        FROM "item_locations"
        WHERE "item_id" = p_item_id;

        -- Remove the item from its current location
        DELETE FROM "item_locations" 
        WHERE "item_id" = p_item_id;

        -- Change the location of the existing item
        UPDATE "item_locations" 
        SET "slot_number" = current_slot_number
        WHERE "item_id" = existing_item_id;

        -- Insert the item into the new location
        INSERT INTO "item_locations" (
            "item_id", "container_id", "slot_number"
        )
        VALUES (
            p_item_id, p_container_id, p_slot_number
        );
    ELSE
        RAISE EXCEPTION 'Slot % is already occupied by item %', p_slot_number, existing_item_id;
    END IF;

END;
$$ LANGUAGE plpgsql;