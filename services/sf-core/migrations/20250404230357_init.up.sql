-- Create types
CREATE TYPE "entity_type" AS ENUM (
    'character', 'item'
);
CREATE TYPE "character_class" AS ENUM ('warrior', 'mage', 'archer');
CREATE TYPE "item_type" AS ENUM (
    'helmet', 'chest', 'gloves', 'boots', 
    'necklace', 'belt', 'ring', 'amulet', 
    'shield', 'weapon', 'potion', 'sub-weapon'
);
CREATE TYPE "item_tier" AS ENUM (
    'common', 'rare', 'epic', 'legendary'
);
CREATE TYPE "resource_type" AS ENUM (
    'gold', 'mushroom'
);
CREATE TYPE "container_type" AS ENUM (
    'inventory', 'shop'
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
    "entity_id" INT PRIMARY KEY REFERENCES "entities"("id") UNIQUE,
    "int_points" INT DEFAULT 0,
    "str_points" INT DEFAULT 0,
    "dex_points" INT DEFAULT 0,
    "lck_points" INT DEFAULT 0,
    "con_points" INT DEFAULT 0
);

CREATE TABLE "items" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "type" "item_type" NOT NULL,
    "tier" "item_tier" NOT NULL,
    "description" TEXT,
    "price" INT DEFAULT 0
);

CREATE TABLE "equipable_items" (
    "class" "character_class" NOT NULL,
    "entity_id" INT REFERENCES "entities"("id") UNIQUE
) INHERITS ("items");

CREATE TABLE "armor_items" (
    "armor_points" INT DEFAULT 0
) INHERITS ("equipable_items");

CREATE TABLE "weapon_items" (
    "min_damage" INT,
    "max_damage" INT
) INHERITS ("equipable_items");

CREATE TABLE "shield_items" (
    "armor_points" INT,
    "block_chance" INT
) INHERITS ("equipable_items"); 

CREATE TABLE "accessory_items" (
    
) INHERITS ("equipable_items");

CREATE TABLE "potions" (
    "quantity" INT DEFAULT 1
) INHERITS ("items");

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
    "character_id" INT REFERENCES "characters"("id") ON DELETE CASCADE,
    "type" "resource_type" NOT NULL,
    "amount" INT DEFAULT 0,
    PRIMARY KEY ("character_id", "type")
);

CREATE TABLE "class_slots" (
    "class" "character_class" NOT NULL,
    "type" "item_type" NOT NULL,
    PRIMARY KEY ("class", "type")
);

CREATE TABLE "equipped_items" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "character_id" INT REFERENCES "characters"("id") ON DELETE CASCADE,
    "type" "item_type" NOT NULL,
    PRIMARY KEY ("character_id", "type")
);

CREATE TABLE "containers" (
    "id" SERIAL PRIMARY KEY,
    "character_id" INT REFERENCES "characters"("id") ON DELETE CASCADE,
    "type" "container_type" NOT NULL,
    "capacity" INT DEFAULT 1
);


CREATE TABLE "item_locations" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "container_id" INT REFERENCES "containers"("id") ON DELETE CASCADE,
    "index" INT DEFAULT 1,
    CONSTRAINT "container_id_index_unique" UNIQUE ("container_id", "index")
);

-- Create functions
CREATE OR REPLACE FUNCTION "add_user"(
    p_external_id VARCHAR(255),
    p_username VARCHAR(255)
)
RETURNS INT AS $$
DECLARE
    p_user_id INT;
BEGIN
    INSERT INTO "users" ("external_id", "username")
    VALUES (p_external_id, p_username)
    RETURNING "id" INTO p_user_id;

    RETURN p_user_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_character"(
    p_user_id INT,
    p_class "character_class",
    p_name VARCHAR(255)
)
RETURNS INT AS $$
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
        "user_id", "entity_id", "class", "name"
    )
    VALUES (
        p_user_id, p_entity_id, p_class, p_name
    )
    RETURNING "id" INTO p_character_id;

    -- Insert a new inventory for the character
    INSERT INTO "containers" (
        "character_id", "type", "capacity"
    )
    VALUES (
        p_character_id, 'inventory', 6
    );

    -- Insert a new shop for the character
    INSERT INTO "containers" (
        "character_id", "type", "capacity"
    )
    VALUES (
        p_character_id, 'shop', 6
    );

    -- Insert default attributes for the character
    INSERT INTO "attributes" (
        "entity_id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points"
    )
    VALUES (
        p_entity_id, 10, 10, 
        10, 10, 10
    );

    RETURN p_character_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_armor_item"(
    p_container_id INT,
    p_index INT,
    p_class "character_class",
    p_int_points INT,
    p_str_points INT,
    p_dex_points INT,
    p_lck_points INT,
    p_con_points INT,
    p_armor_points INT,
    p_type "item_type",
    p_tier "item_tier",
    p_description TEXT,
    p_price INT
)
RETURNS INT AS $$
DECLARE
    p_entity_id INT;
    p_item_id INT;
BEGIN
    -- Check if the container slot is already occupied
    IF (
        SELECT 1
        FROM "item_locations"
        WHERE "container_id" = p_container_id
        AND "index" = p_index
    )
    THEN
        RAISE EXCEPTION 'Item already exists in the specified container and index';
    END IF;

    -- Insert a new entity
    INSERT INTO "entities" ("type")
    VALUES ('item')
    RETURNING "id" INTO p_entity_id;

    -- Insert a new armor item
    INSERT INTO "armor_items" (
        "name",
        "class", "entity_id", "tier", "type", 
        "armor_points", "description", "price"
    )
    VALUES (
        p_class, p_entity_id, p_tier, p_type,
        p_armor_points, p_description, p_price
    )
    RETURNING "id" INTO p_item_id;

    -- Insert a new attribute for the item
    INSERT INTO "attributes" (
        "entity_id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points",
        "armor_points"
    )
    VALUES (
        p_entity_id, p_str_points, p_dex_points, 
        p_int_points, p_con_points, p_lck_points
    );

    -- Insert the item into the container
    INSERT INTO "item_locations" (
        "item_id", "container_id", "index"
    )
    VALUES (
        p_item_id, p_container_id, p_index
    );
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION "add_accessory_item"(
    p_container_id INT,
    p_index INT,
    p_class "character_class",
    p_int_points INT,
    p_str_points INT,
    p_dex_points INT,
    p_lck_points INT,
    p_con_points INT,
    p_type "item_type",
    p_tier "item_tier",
    p_description TEXT,
    p_price INT
)
RETURNS INT AS $$
DECLARE
    p_entity_id INT;
    p_item_id INT;
BEGIN
    -- Check if the container slot is already occupied
    IF (
        SELECT 1
        FROM "item_locations"
        WHERE "container_id" = p_container_id
        AND "index" = p_index
    )
    THEN
        RAISE EXCEPTION 'Item already exists in the specified container and index';
    END IF;

    -- Insert a new entity
    INSERT INTO "entities" ("type")
    VALUES ('item')
    RETURNING "id" INTO p_entity_id;

    -- Insert a new armor item
    INSERT INTO "accessory_items" (
        "name",
        "class", "entity_id", "tier", 
        "type", "description", "price"
    )
    VALUES (
        p_class, p_entity_id, p_tier, 
        p_type, p_description, p_price
    )
    RETURNING "id" INTO p_item_id;

    -- Insert a new attribute for the item
    INSERT INTO "attributes" (
        "entity_id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points"
    )
    VALUES (
        p_entity_id, p_str_points, p_dex_points, 
        p_int_points, p_con_points, p_lck_points
    );

    -- Insert the item into the container
    INSERT INTO "item_locations" (
        "item_id", "container_id", "index"
    )
    VALUES (
        p_item_id, p_container_id, p_index
    );
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION "add_weapon_item"(
    p_container_id INT,
    p_index INT,
    p_class "character_class",
    p_int_points INT,
    p_str_points INT,
    p_dex_points INT,
    p_lck_points INT,
    p_con_points INT,
    p_min_damage INT,
    p_max_damage INT,
    p_type "item_type",
    p_tier "item_tier",
    p_description TEXT,
    p_price INT
)
RETURNS INT AS $$
DECLARE
    p_entity_id INT;
    p_item_id INT;
BEGIN
    -- Check if the container slot is already occupied
    IF (
        SELECT 1
        FROM "item_locations"
        WHERE "container_id" = p_container_id
        AND "index" = p_index
    )
    THEN
        RAISE EXCEPTION 'Item already exists in the specified container and index';
    END IF;

    -- Insert a new entity
    INSERT INTO "entities" ("type")
    VALUES ('item')
    RETURNING "id" INTO p_entity_id;

    -- Insert a new armor item
    INSERT INTO "weapon_items" (
        "name",
        "class", "entity_id", "tier", 
        "type", "description", "price",
        "min_damage", "max_damage"
    )
    VALUES (
        p_class, p_entity_id, p_tier, 
        p_type, p_description, p_price,
        p_min_damage, p_max_damage
    )
    RETURNING "id" INTO p_item_id;

    -- Insert a new attribute for the item
    INSERT INTO "attributes" (
        "entity_id", "str_points", "dex_points", 
        "int_points", "con_points", "lck_points"
    )
    VALUES (
        p_entity_id, p_str_points, p_dex_points, 
        p_int_points, p_con_points, p_lck_points
    );

    -- Insert the item into the container
    INSERT INTO "item_locations" (
        "item_id", "container_id", "index"
    )
    VALUES (
        p_item_id, p_container_id, p_index
    );
END;
$$ LANGUAGE plpgsql;