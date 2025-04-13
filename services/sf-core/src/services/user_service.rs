use tonic::{Request, Response, Status};
use crate::proto::sf_user::{
    CreateUserRequest, CreateUserResponse,
    user_service_server::UserService,
};

#[derive(Debug, Default)]
pub struct UserServiceImpl;

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        println!("Received: {:?}", req);

        let user_id = format!("user-{}", req.external_id);

        let reply = CreateUserResponse {
            user_id: user_id,
        };

        Ok(Response::new(reply))
    }
}