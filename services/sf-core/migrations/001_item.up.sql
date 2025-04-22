CREATE TABLE "item_categories" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) NOT NULL
);
INSERT INTO "item_categories" ("name")
VALUES ('helmet'), ('chest'), ('gloves'), ('boots'), 
('necklace'), ('belt'), ('ring'), ('amulet'), 
('shield'), ('weapon'), ('consumable'), ('sub_weapon');

CREATE TABLE "item_sub_categories" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) NOT NULL
);
INSERT INTO "item_sub_categories" ("name")
VALUES ('clothing'), ('light'), ('medium'), ('heavy'), 
('trinket'), ('attribute_potion'), ('staff'), ('sword'), ('bow'),
('shield'), ('dagger'), ('hp_potion');

CREATE TABLE "item_groups" (
    "name" VARCHAR(20) NOT NULL,
    "item_category_id" SMALLINT REFERENCES "item_categories" ("id")
);
INSERT INTO "item_groups" ("item_category_id", "name")
SELECT 
    "id",
    CASE
        WHEN "name" IN ('helmet', 'chest', 'gloves', 'boots', 'belt') THEN 'armor'
        WHEN "name" IN ('necklace', 'ring', 'amulet') THEN 'accessory'
        WHEN "name" IN ('weapon', 'sub_weapon') THEN 'weapon'
        WHEN "name" IN ('shield') THEN 'shield'
        WHEN "name" IN ('potion') THEN 'potion'
        ELSE 'other'
    END
FROM "item_categories";

CREATE TABLE "item_tiers" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) NOT NULL
);
INSERT INTO "item_tiers" ("name")
VALUES ('common'), ('uncommon'),
('rare'), ('epic'), ('legendary');

CREATE TABLE "preset_items" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(255),
    "description" TEXT,
    "item_category_id" INT REFERENCES "item_categories" ("id") NOT NULL,
    "item_sub_category_id" INT REFERENCES "item_sub_categories" ("id") NOT NULL,
    "item_tier_id" INT REFERENCES "item_tiers" ("id") NOT NULL
);

CREATE TABLE "items" (
    "id" SERIAL PRIMARY KEY,
    "preset_item_id" SMALLINT REFERENCES "preset_items" ("id") NOT NULL,
    "overridden_item_tier_id" INT REFERENCES "item_tiers" ("id") DEFAULT NULL,
    "price" INT DEFAULT 0
);

CREATE TABLE "equipable_items" (
    "item_id" INT PRIMARY KEY REFERENCES "items"("id") UNIQUE,
    "entity_id" INT REFERENCES "entities"("id") NOT NULL UNIQUE,
    "job_id" SMALLINT REFERENCES "jobs" ("id") NOT NULL
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

CREATE TABLE "equipped_items" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "item_category_id" INT REFERENCES "item_categories" ("id") NOT NULL,
    PRIMARY KEY ("character_id", "item_category_id")
);
