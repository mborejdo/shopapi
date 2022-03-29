
use crate::errors::ServiceError;

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    Ok(true)
}