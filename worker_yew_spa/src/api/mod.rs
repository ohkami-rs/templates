pub mod errors;
pub mod utils;

use self::errors::ServerError;
use crate::Bindings;
use crate::models::{};


#[worker::send]
pub async fn hello(
    b: Bindings,
) -> Result<&'static str, ServerError> {
    Ok("Hello, worler!")
}
