use sqlx::FromRow;

pub mod user;
pub use user::{
    User
};

pub mod character;
pub use character::{
    Character,
    Appearance,
};

pub mod entity;
pub use entity::{
    Entity,
    EntityType,
    Attribute,
};

pub mod container;
pub use container::{
    Container,
    ContainerType,
    Shop,
    ResSlot,
    ResAccessory,
    ResArmor,
    ResEquipment,
    ResPotion,
    ResShield,
};

pub mod item;
pub use item::{
    Item,
    EquippableItem,
    Potion,
    ArmorItem,
    WeaponItem,
    ShieldItem,
    AccessoryItem,
};

#[derive(Debug, FromRow)]
pub struct LookUpValue {
    pub id: i16,
    pub name: String,
}