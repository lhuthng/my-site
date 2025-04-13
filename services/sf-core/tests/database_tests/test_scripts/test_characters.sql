BEGIN;

SELECT plan(1);

SELECT add_user('0x1234', 'My User');

SELECT add_character(
    (SELECT "id" FROM "users" WHERE "username" = 'My User'),
    'warrior',
    'My Character'
);

SELECT is(
    (SELECT "name" FROM "characters" JOIN "users" ON "characters"."user_id" = "users"."id"
    WHERE "external_id" = '0x1234'),
    'My Character',
    'The add_character function did not insert the character correctly'
);

ROLLBACK;