use tonic::{Request, Response, Status};
use crate::proto::sf_core::{
    Empty, LookUpValue as LookUpValueGRPC,
    Slot,
    CreateUserRequest, CreateUserResponse,
    CreateCharacterRequest, CreateCharacterResponse,
    GetRacesResponse, GetGendersResponse,
    GetGearShopRequest, GetContainerResponse,
    profile_service_server::ProfileService,
};
use sqlx::{PgPool, Transaction, Postgres};
use crate::db::{
    user_queries,
    character_queries,
    look_up_table_queries,
    container_queries,
};
use crate::models::{
    LookUpValue as LookUpValueModel,
    Appearance,
    ContainerType,
};
use uuid::Uuid;
use chrono::{
    NaiveDate,
    Local,
};
use crate::services::error::IntoStatus;

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
        let mut tx: Transaction<'_, sqlx::Postgres> = self.pool.begin().await.into_status()?;
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
        ).await.into_status()?;

        tx.commit().await.into_status()?;

        let reply = CreateCharacterResponse {
            character_id: character_id.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_races(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<GetRacesResponse>, Status> {
                
        let values: Vec<LookUpValueModel> = look_up_table_queries::get_all_look_up_values(
            &self.pool, 
            "races"
        ).await.into_status()?;

        let reply = GetRacesResponse {
            races: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        };

        Ok(Response::new(reply))
    }

    async fn get_genders(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<GetGendersResponse>, Status> {
        
        let values: Vec<LookUpValueModel> = look_up_table_queries::get_all_look_up_values(
            &self.pool, 
            "genders"
        ).await.into_status()?;

        let reply = GetGendersResponse {
            genders: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        };

        Ok(Response::new(reply))
    }

    async fn get_gear_shop(
        &self,
        request: Request<GetGearShopRequest>,
    ) -> Result<Response<GetContainerResponse>, Status> {

        let req = request.into_inner();
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await.into_status()?;
        let character_uid = Uuid::parse_str(&req.character_id).expect("Invalid UUID string");
        let slots = container_queries::get_items_from_gear_shop(
            &mut tx, character_uid,
            ContainerType::GearShop,
        ).await.into_status()?;

        let shop = container_queries::get_shop(
            &mut tx,
            character_uid,
            ContainerType::GearShop,
        ).await.into_status()?;

        let need_refresh = match shop.last_refresh {
            None => true,
            Some(last_refresh) => last_refresh < Local::now().date_naive(),
        };

        if need_refresh {
            container_queries::update_last_refresh(
                &mut tx,
                shop.container_id,
            ).await.into_status()?;
        }

        tx.commit().await.into_status()?;

        let reply = GetContainerResponse {
            slots: slots,
        };

        Ok(Response::new(reply))
    }
}

