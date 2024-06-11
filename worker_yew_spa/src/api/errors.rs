use ohkami::prelude::*;


#[derive(Debug, thiserror::Error)]
pub enum ServerError {
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        todo!()
    }
}
