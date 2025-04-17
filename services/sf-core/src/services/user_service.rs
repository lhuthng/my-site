use std::str::FromStr;
use tonic::{Request, Response, Status};
use crate::proto::sf_user::{
    CreateUserRequest, CreateUserResponse,
    CreateCharacterRequest, CreateCharacterResponse,
    profile_service_server::ProfileService,
};
use sqlx::{PgPool, Transaction};
use crate::db::{
    user_queries,
    character_queries,
};
use crate::models::CharacterClass;

#[derive(Debug)]
pub struct UserServiceImpl {
    pool: PgPool,
}

impl ProfileServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let mut tx: Transaction<'_, sqlx::Postgres> = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        let user_id = user_queries::create_user(
            &mut tx,
            req.external_id.as_str(),
            req.username.as_str(),
        ).await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        let reply = CreateUserResponse {
            user_id: user_id,
        };

        Ok(Response::new(reply))
    }
    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        let req = request.into_inner();
        let mut tx: Transaction<'_, sqlx::Postgres> = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        let character_id = character_queries::create_character(
            &mut tx,
            req.user_id,
            req.job.parse().unwrap(),
            req.name.as_str(),
            1,
            0,
        ).await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })?;

        let reply = CreateCharacterResponse {
            character_id: character_id.to_string(),
        };

        Ok(Response::new(reply))
    }
}