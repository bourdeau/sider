use crate::errors::SiderError;
use crate::response::SiderResponse;

pub async fn pong() -> Result<SiderResponse, SiderError> {
    Ok(SiderResponse::SimpleString("PONG".to_string()))
}
