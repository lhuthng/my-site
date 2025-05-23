use sqlx::PgPool;
use uuid::Uuid;
use chrono::Local;
use tonic::{
    Request, 
    Response, 
    Status
};
use crate::proto::sf_core::{equipment, GetEquipmentRequest, GetEquipmentResponse};
use crate::proto::sf_core::{
    Empty, LookUpValue as LookUpValueGRPC,
    CreateUserRequest, CreateUserResponse,
    CreateCharacterRequest, CreateCharacterResponse,
    GetRacesResponse, GetGendersResponse,
    GetShopRequest, GetShopResponse,
    BuyItemFromShopRequest, BuyItemFromShopResponse, 
    GetInventoryRequest, GetInventoryResponse,
    profile_service_server::ProfileService,
};
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
use crate::services::error::IntoStatus;
use crate::services::transactional::{
    Transactional,
    commit
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

impl Transactional for ProfileServiceImpl {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();

        let user_id = commit!(self, |tx| {
            user_queries::create_user( 
                &mut tx,
                req.external_id.as_str(),
                req.username.as_str(),
            ).await.into_status()?
        });

        let reply = CreateUserResponse {
            user_id
        };

        Ok(Response::new(reply))
    }
    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        let req = request.into_inner();

        let character_id = commit!(self, |tx| {
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
    
            character_queries::create_character(
                &mut tx,
                req.user_id,
                req.job_id as i16,
                req.name.as_str(),
                &app,
                1, 0,
            ).await.into_status()?.to_string()
        });

        Ok(Response::new(CreateCharacterResponse{
            character_id
        }))
    }

    async fn get_races(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<GetRacesResponse>, Status> {
                
        let values: Vec<LookUpValueModel> = look_up_table_queries::get_all_look_up_values(
            &self.pool, 
            "races"
        ).await.into_status()?;

        Ok(Response::new(GetRacesResponse {
            races: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        }))
    }

    async fn get_genders(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<GetGendersResponse>, Status> {
        
        let values: Vec<LookUpValueModel> = look_up_table_queries::get_all_look_up_values(
            &self.pool, 
            "genders"
        ).await.into_status()?;

        Ok(Response::new(GetGendersResponse {
            genders: values.into_iter().map(Into::into).collect::<Vec<LookUpValueGRPC>>(),
        }))
    }

    async fn get_gear_shop(
        &self,
        request: Request<GetShopRequest>,
    ) -> Result<Response<GetShopResponse>, Status> {

        let req = request.into_inner();

        let slots = commit!(self, |tx| {
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
                container_queries::refresh_shop(
                    &mut tx,
                    shop.container_id,
                    ContainerType::GearShop,
                ).await.into_status()?;
            }
            slots
        });

        Ok(Response::new(GetShopResponse {
            slots,
        }))
    }

    async fn get_magic_shop(
        &self,
        request: Request<GetShopRequest>
    ) -> Result<Response<GetShopResponse>, Status> {
        let req = request.into_inner();

        let slots = commit!(self, |tx| {
            let character_uid = Uuid::parse_str(&req.character_id).expect("Invalid UUID string");
            let slots = container_queries::get_items_from_gear_shop(
                &mut tx, character_uid,
                ContainerType::GearShop,
            ).await.into_status()?;

            let shop = container_queries::get_shop(
                &mut tx,
                character_uid,
                ContainerType::MagicShop,
            ).await.into_status()?;

            let need_refresh = match shop.last_refresh {
                None => true,
                Some(last_refresh) => last_refresh < Local::now().date_naive(),
            };

            if need_refresh {
                container_queries::refresh_shop(
                    &mut tx,
                    shop.container_id,
                    ContainerType::GearShop,
                ).await.into_status()?;
            }

            slots
        });

        Ok(Response::new(GetShopResponse {
            slots,
        }))
    }

    async fn get_inventory(
        &self,
        request: Request<GetInventoryRequest>,
    ) -> Result<Response<GetInventoryResponse>, Status> {
        let req = request.into_inner();

        let slots = commit!(self, |tx| {
            Vec::new()
        });

        Ok(Response::new(GetInventoryResponse { 
            slots, 
            capacity: 6
        }))
    }

    async fn buy_item_from_shop(
        &self,
        request: Request<BuyItemFromShopRequest>
    ) -> Result<Response<BuyItemFromShopResponse>, Status> {
        let req = request.into_inner();

        Ok(Response::new(BuyItemFromShopResponse {
            after: None
        }))
    }

    async fn get_equipment(
        &self,
        request: Request<GetEquipmentRequest>
    ) -> Result<Response<GetEquipmentResponse>, Status> {
        let req = request.into_inner();
        
        let equipment = commit!(self, |tx| {
            Vec::new()
        });

        Ok(Response::new(GetEquipmentResponse { 
            equipment
        }))
    }
}

