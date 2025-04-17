
-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS "item_locations";
DROP TABLE IF EXISTS "containers";
DROP TABLE IF EXISTS "equipped_items";
DROP TABLE IF EXISTS "class_slots";
DROP TABLE IF EXISTS "resources";
DROP TABLE IF EXISTS "characters";
DROP TABLE IF EXISTS "potions";
DROP TABLE IF EXISTS "accessory_items";
DROP TABLE IF EXISTS "shield_items";
DROP TABLE IF EXISTS "weapon_items";
DROP TABLE IF EXISTS "armor_items";
DROP TABLE IF EXISTS "equipable_items";
DROP TABLE IF EXISTS "items";
DROP TABLE IF EXISTS "attributes";
DROP TABLE IF EXISTS "entities";
DROP TABLE IF EXISTS "users";

-- Drop types in reverse order of creation
DROP TYPE IF EXISTS "container_type";
DROP TYPE IF EXISTS "resource_type";
DROP TYPE IF EXISTS "item_tier";
DROP TYPE IF EXISTS "item_type";
DROP TYPE IF EXISTS "character_class";
DROP TYPE IF EXISTS "entity_type";