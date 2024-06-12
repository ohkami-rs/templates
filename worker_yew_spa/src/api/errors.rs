use ohkami::prelude::*;


#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Worker error: {0}")]
    Worker(#[from] worker::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        worker::console_error!("{self}");

        match self {
            Self::Worker(_) => Response::InternalServerError(),
        }
    }
}
