-- Drop functions
DROP FUNCTION IF EXISTS move_item;
DROP FUNCTION IF EXISTS add_equipable_item;
DROP FUNCTION IF EXISTS add_item;
DROP FUNCTION IF EXISTS add_character;
DROP FUNCTION IF EXISTS add_user;

-- Drop tables
DROP TABLE IF EXISTS item_locations;
DROP TABLE IF EXISTS containers;
DROP TABLE IF EXISTS equipment;
DROP TABLE IF EXISTS resources;
DROP TABLE IF EXISTS characters;
DROP TABLE IF EXISTS potions;
DROP TABLE IF EXISTS equipable_items;
DROP TABLE IF EXISTS items;
DROP TABLE IF EXISTS attributes;
DROP TABLE IF EXISTS entities;
DROP TABLE IF EXISTS users;

-- Drop types
DROP TYPE IF EXISTS container_type;
DROP TYPE IF EXISTS resource_type;
DROP TYPE IF EXISTS equipment_type;
DROP TYPE IF EXISTS item_type;
DROP TYPE IF EXISTS character_class;
DROP TYPE IF EXISTS entity_type;