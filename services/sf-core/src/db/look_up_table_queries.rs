use sqlx::{
    PgPool,
    Transaction,
    Postgres,
};
use tokio::sync::{OnceCell, RwLock};
use std::collections::HashMap;

use crate::misc::BiMap;
use crate::models::LookUpValue;
use crate::models::LookUpValue as LookUpValueDB;
use crate::proto::sf_core::LookUpValue as LookUpValueGRPC;

impl From<LookUpValueDB> for LookUpValueGRPC {
    fn from(db: LookUpValueDB) -> Self {
        LookUpValueGRPC {
            id: db.id as i32,
            name: db.name,
        }
    }
}

async fn get_look_up_values(
    tx: &mut Transaction<'_, Postgres>,
    name: &str
) -> Result<Vec<LookUpValue>, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Getting look up values for {}", name);

    let query = format!("SELECT id, name FROM {}", name);
    let result: Vec<LookUpValue> = sqlx::query_as::<_, LookUpValue>(&query)
    .fetch_all(&mut **tx)
    .await?;

    Ok(result)
}

static REGISTRY: OnceCell<RwLock<NameLookupRegistry>> = OnceCell::const_new();

pub struct NameLookupRegistry {
    pub look_up_tables: HashMap<String, BiMap<i16, String>>,
}

macro_rules! insert_into_registry {
    ($registry:ident, $($list:ident),+) => {
        $(
            $registry.look_up_tables.insert(stringify!($list).to_string(), BiMap::new());
            for item in $list {
                if let Some(val) = $registry.look_up_tables.get_mut(stringify!($list)) {
                    val.insert(item.id, item.name.clone());
                }
            }
        )+
    };
}

pub async fn initialize(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    let item_categories = get_look_up_values(&mut tx, "item_categories").await?;
    let item_sub_categories = get_look_up_values(&mut tx, "item_sub_categories").await?;
    let item_groups = get_look_up_values(&mut tx, "item_groups").await?;
    let item_tiers = get_look_up_values(&mut tx, "item_tiers").await?;
    let jobs = get_look_up_values(&mut tx, "jobs").await?;
    let resources = get_look_up_values(&mut tx, "resources").await?;
    let races = get_look_up_values(&mut tx, "races").await?;
    let genders = get_look_up_values(&mut tx, "genders").await?;     

    REGISTRY.get_or_try_init(|| async {
        let mut registry = NameLookupRegistry {
            look_up_tables: HashMap::new(),
        };

        insert_into_registry!(registry,
            item_categories,
            item_sub_categories,
            item_groups,
            item_tiers,
            jobs,
            resources,
            races,
            genders
        );

        Ok::<RwLock<NameLookupRegistry>, sqlx::Error>(RwLock::new(registry))
    }).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_all_look_up_values(
    pool: &PgPool,
    name: &str
) -> Result<Vec<LookUpValue>, sqlx::Error> {
    let registry = REGISTRY
        .get()
        .ok_or(sqlx::Error::Protocol("Registry not initialized".into()))?
        .read()
        .await;

    let iter = registry.look_up_tables.get(name).expect("No table found").iter_forward();
    let vec: Vec<LookUpValue> = iter
        .map(|(&id, name)| LookUpValue {
            id,
            name: name.clone(),
        })
        .collect();

    Ok(vec)
}