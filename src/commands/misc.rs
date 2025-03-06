use crate::errors::SiderError;
use crate::response::SiderResponse;

pub async fn pong() -> Result<SiderResponse, SiderError> {
    Ok(SiderResponse::SimpleString("PONG".to_string()))
}

pub async fn docs() -> Result<SiderResponse, SiderError> {
    Ok(SiderResponse::SimpleString(
        "DOCS is not implemented yet".to_string(),
    ))
}

pub async fn client() -> Result<SiderResponse, SiderError> {
    Ok(SiderResponse::SimpleString(
        "CLIENT SETINFO is not implemented yet".to_string(),
    ))
}
