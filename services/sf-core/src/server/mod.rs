use tonic::transport::Server;
use crate::services::user_service::UserServiceImpl;
use crate::proto::sf_user::user_service_server::UserServiceServer;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = UserServiceImpl::default();

    println!("UserService listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}