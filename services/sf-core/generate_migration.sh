#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: ./generate_migration.sh <migration_name>"
  exit 1
fi

TIMESTAMP=$(date +"%Y%m%d%H%M%S")
MIGRATION_NAME="$TIMESTAMP"_"$1"

touch "migrations/$MIGRATION_NAME.up.sql"
touch "migrations/$MIGRATION_NAME.down.sql"

echo "Created migration: migrations/$MIGRATION_NAME.sql"