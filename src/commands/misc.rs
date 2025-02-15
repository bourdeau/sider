use crate::errors::SiderError;

pub async fn pong() -> Result<String, SiderError> {
    Ok("PONG\n".to_string())
}
