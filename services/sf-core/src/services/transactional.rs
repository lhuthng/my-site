use sqlx::{
    PgPool,
    Transaction,
    Postgres,
    Error
};

use async_trait::{
    async_trait
};

#[async_trait]
pub trait Transactional: Sized {
    fn get_pool(&self) -> &PgPool;
    async fn get_transaction(&self) -> Result<Transaction<'_, Postgres>, Error> {
        Ok(self.get_pool().begin().await?)
    }
}

macro_rules! commit {
    ($self:ident, |$tx:ident| $block:block) => {{
        let mut $tx = $self.get_transaction().await.into_status()?;
        let res = $block;
        $tx.commit().await.into_status()?;
        res
    }};
}
pub(crate) use commit;