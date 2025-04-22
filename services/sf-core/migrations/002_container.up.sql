CREATE TYPE "container_type" AS ENUM (
    'inventory', 'gear_shop', 'magic_shop'
);

CREATE TABLE "containers" (
    "id" SERIAL PRIMARY KEY,
    "character_id" UUID REFERENCES "characters"("id") ON DELETE CASCADE,
    "kind" "container_type" NOT NULL,
    "capacity" INT DEFAULT 1
);

CREATE TABLE "shops" (
    "container_id" INT REFERENCES "containers"("id") ON DELETE CASCADE,
    "last_refresh" DATE
);

CREATE TABLE "item_locations" (
    "item_id" INT REFERENCES "items"("id") ON DELETE CASCADE,
    "container_id" INT REFERENCES "containers"("id") ON DELETE CASCADE,
    "index" INT DEFAULT 1,
    CONSTRAINT "container_id_index_unique" UNIQUE ("container_id", "index")
);