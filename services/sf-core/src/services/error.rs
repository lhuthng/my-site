use tonic::{Status};
use sqlx::Error;

pub trait IntoStatus<T> {
    fn into_status(self) -> Result<T, Status>;
}

impl<T> IntoStatus<T> for Result<T, sqlx::Error> {
    fn into_status(self) -> Result<T, Status> {
        self.map_err(|e| {
            Status::internal(format!("DB error: {}", e))
        })
    }
}