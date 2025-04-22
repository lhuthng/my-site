-- Create types
CREATE TYPE "entity_type" AS ENUM (
    'character', 'item', 'pet'
);

CREATE TABLE "jobs" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) UNIQUE
);
INSERT INTO "jobs" ("name")
VALUES ('warrior'), ('mage'), ('archer');

CREATE TABLE "resources" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) UNIQUE
);
INSERT INTO "resources" ("name")
VALUES ('gold'), ('mushroom');

CREATE TABLE "races" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) UNIQUE
);
INSERT INTO "races" ("name")
VALUES ('human'), ('elf'), ('drawf'), ('gnome'), ('orc'), ('dark_elf'), ('goblin'), ('demon');

CREATE TABLE "genders" (
    "id" SMALLSERIAL PRIMARY KEY,
    "name" VARCHAR(20) UNIQUE
);
INSERT INTO "genders" ("name")
VALUES ('male'), ('female');

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
    "int" INT DEFAULT 1,
    "str" INT DEFAULT 1,
    "dex" INT DEFAULT 1,
    "lck" INT DEFAULT 1,
    "con" INT DEFAULT 1
);

CREATE TABLE "characters" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "user_id" INT REFERENCES "users"("id") ON DELETE CASCADE,
    "entity_id" INT REFERENCES "entities"("id") ON DELETE CASCADE,
    "job_id" SMALLINT REFERENCES "jobs"("id"),
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "level" SMALLINT DEFAULT 1,
    "exp" INT DEFAULT 0
);

CREATE TABLE "character_resources" (
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "resource_id" SMALLINT REFERENCES "resources"("id") ON DELETE CASCADE,
    "amount" INT DEFAULT 0,
    PRIMARY KEY ("character_id", "resource_id")
);

CREATE TABLE "appearances" (
    "entity_id" INT REFERENCES "entities"("id") ON DELETE CASCADE,
    "race_id" SMALLINT REFERENCES "races"("id") NOT NULL,
    "gender_id" SMALLINT REFERENCES "genders"("id") NOT NULL,
    "beard" SMALLINT DEFAULT 0,
    "mouth" SMALLINT DEFAULT 0,
    "eyebrows" SMALLINT DEFAULT 0,
    "nose" SMALLINT DEFAULT 0,
    "ears" SMALLINT DEFAULT 0,
    "hair" SMALLINT DEFAULT 0,
    "hair_color" SMALLINT DEFAULT 0,
    "extra" SMALLINT DEFAULT 0
);