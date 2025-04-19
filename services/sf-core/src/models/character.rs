use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub user_id: i32,
    pub entity_id: i32,
    pub job_id: i16,
    pub name: String,
    pub level: i16,
    pub exp: i32,
}

#[derive(Debug, FromRow)]
pub struct CharacterResource {
    pub character_id: i32,
    pub resource_id: i16,
    pub amount: i32,
}

#[derive(Debug, FromRow, Default)]
pub struct Appearance {
    pub character_id: i32,
    pub race_id: i16,
    pub gender_id: i16,
    pub hair: i16,
    pub hair_color: i16,
    pub beard: i16,
    pub mouth: i16,
    pub eyebrows: i16,
    pub nose: i16,
    pub ears: i16,
    pub extra: i16,
}