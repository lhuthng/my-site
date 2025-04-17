use sqlx::{Transaction, Postgres};

pub async fn create_user(
    tx: &mut Transaction<'_, Postgres>,
    external_id: &str,
    username: &str,
) -> Result<i32, sqlx::Error> {

    #[cfg(debug_assertions)]
    println!("Adding a container.");
    
    let user_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO users (external_id, username)
        VALUES ($1, $2)
        RETURNING id
        "#,
        external_id,
        username
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(user_id)
}