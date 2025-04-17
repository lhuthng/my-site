use tonic::transport::Server;
use sqlx::PgPool;
use crate::services::profile_service::ProfileServiceImpl;
use crate::proto::sf_user::profile_service_server::ProfileServiceServer;

pub async fn start(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = ProfileServiceImpl::new(pool.clone());

    println!("UserService listening on {}", addr);

    Server::builder()
        .add_service(ProfileServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}