-- Create types
CREATE TYPE "entity_type" AS ENUM (
    'character', 'item'
);
CREATE TYPE "character_class" AS ENUM ('warrior', 'mage', 'archer');
CREATE TYPE "item_type" AS ENUM (
    'helmet', 'chest', 'gloves', 'boots', 
    'necklace', 'belt', 'ring', 'amulet', 
    'shield', 'weapon', 'potion', 'sub_weapon'
);
CREATE TYPE "item_tier" AS ENUM (
    'common', 'uncommon', 'rare', 'epic', 'legendary'
);
CREATE TYPE "resource_type" AS ENUM (
    'gold', 'mushroom'
);
CREATE TYPE "container_type" AS ENUM (
    'inventory', 'weapon_shop', 'magic_shop'
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
    "kind" "entity_type" NOT NULL
);

CREATE TABLE "attributes" (
    "entity_id" INT PRIMARY KEY REFERENCES "entities"("id") UNIQUE,
    "int_points" INT DEFAULT 1,
    "str_points" INT DEFAULT 1,
    "dex_points" INT DEFAULT 1,
    "lck_points" INT DEFAULT 1,
    "con_points" INT DEFAULT 1
);

CREATE TABLE "items" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "kind" "item_type" NOT NULL,
    "tier" "item_tier" NOT NULL,
    "description" TEXT,
    "price" INT DEFAULT 0
);

CREATE TABLE "equipable_items" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id") UNIQUE,
    "entity_id" INT REFERENCES "entities"("id") UNIQUE,
    "job" "character_class" NOT NULL
);

CREATE TABLE "armor_items" (
    "item_id" INT PRIMARY KEY REFERENCES "equipable_items"("item_id") UNIQUE,
    "armor_points" INT DEFAULT 0
);

CREATE TABLE "weapon_items" (
    "item_id" INT PRIMARY KEY REFERENCES "equipable_items"("item_id") UNIQUE,
    "min_damage" INT,
    "max_damage" INT
);

CREATE TABLE "shield_items" (
    "item_id" INT PRIMARY KEY REFERENCES "equipable_items"("item_id") UNIQUE,
    "armor_points" INT,
    "block_points" INT
);

CREATE TABLE "accessory_items" (
    "item_id" INT PRIMARY KEY REFERENCES "equipable_items"("item_id") UNIQUE
);

CREATE TABLE "potions" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id") UNIQUE,
    "quantity" INT DEFAULT 1
);

CREATE TABLE "characters" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id" INT REFERENCES "users"("id"),
    "entity_id" INT REFERENCES "entities"("id") UNIQUE,
    "job" "character_class" NOT NULL,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "level" INT DEFAULT 1,
    "exp" INT DEFAULT 0
);

CREATE TABLE "resources" (
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "kind" "resource_type" NOT NULL,
    "amount" INT DEFAULT 0,
    PRIMARY KEY ("character_id", "kind")
);

CREATE TABLE "class_slots" (
    "job" "character_class" NOT NULL,
    "kind" "item_type" NOT NULL,
    PRIMARY KEY ("job", "kind")
);

CREATE TABLE "equipped_items" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "kind" "item_type" NOT NULL,
    PRIMARY KEY ("character_id", "kind")
);

CREATE TABLE "containers" (
    "id" SERIAL PRIMARY KEY,
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "kind" "container_type" NOT NULL,
    "capacity" INT DEFAULT 1
);

CREATE TABLE "item_locations" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "container_id" INT REFERENCES "containers"("id") ON DELETE CASCADE,
    "index" INT DEFAULT 1,
    CONSTRAINT "container_id_index_unique" UNIQUE ("container_id", "index")
);