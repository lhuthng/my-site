use tonic::{Request, Response, Status};
use crate::proto::sf_core::{
    Empty, LookUpValue as LookUpValueGRPC,
    CreateUserRequest, CreateUserResponse,
    CreateCharacterRequest, CreateCharacterResponse,
    GetRacesResponse, GetGendersResponse,
    profile_service_server::ProfileService,
};
use sqlx::{PgPool, Transaction};
use crate::db::{
    user_queries,
    character_queries,
    get_look_up_values,
};
use crate::models::{
    LookUpValue as LookUpValueModel,
    Appearance,
};

#[derive(Debug)]
pub struct ProfileServiceImpl {
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
            Status::internal(format!("Transaction error: {}", e))
        })?;

        let user_id = user_queries::create_user(
            &mut tx,
            req.external_id.as_str(),
            req.username.as_str(),
        ).await.map_err(|e| {
            Status::internal(format!("Creating user error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("Transaction commiting error: {}", e))
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
            Status::internal(format!("Transaction error: {}", e))
        })?;
        let r_app = req.appearance.unwrap();
        let app = Appearance {
            character_id: 0,
            race_id: r_app.race_id as i16,
            gender_id: r_app.gender_id as i16,
            hair: r_app.hair as i16,
            hair_color: r_app.hair_color as i16,
            beard: r_app.beard as i16,
            mouth: r_app.mouth as i16,
            eyebrows: r_app.eyebrows as i16,
            nose: r_app.nose as i16,
            ears: r_app.ears as i16,
            extra: r_app.extra as i16,
        };

        let character_id = character_queries::create_character(
            &mut tx,
            req.user_id,
            req.job_id as i16,
            req.name.as_str(),
            &app,
            1, 0,
        ).await.map_err(|e| {
            Status::internal(format!("Creating character error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("Transaction commiting error: {}", e))
        })?;

        let reply = CreateCharacterResponse {
            character_id: character_id.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_races(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<GetRacesResponse>, Status> {
        let mut tx: Transaction<'_, sqlx::Postgres> = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("Transaction error: {}", e))
        })?;
        
        let values: Vec<LookUpValueModel> = get_look_up_values(&mut tx, "races").await.map_err(|e| {
            Status::internal(format!("Getting all races error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("Transaction commiting error: {}", e))
        })?;

        let reply = GetRacesResponse {
            races: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        };

        Ok(Response::new(reply))
    }

    async fn get_genders(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<GetGendersResponse>, Status> {
        let mut tx: Transaction<'_, sqlx::Postgres> = self.pool.begin().await.map_err(|e| {
            Status::internal(format!("Transaction error: {}", e))
        })?;
        
        let values: Vec<LookUpValueModel> = get_look_up_values(&mut tx, "genders").await.map_err(|e| {
            Status::internal(format!("Getting all genders error: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            Status::internal(format!("Transaction commiting error: {}", e))
        })?;

        let reply = GetGendersResponse {
            genders: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        };

        Ok(Response::new(reply))
    }
}