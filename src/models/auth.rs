use crate::errors::ServiceError;
use crate::{
    auth,
    models::users::{User},
    types::PostgresPool
};
use anyhow::Result;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {

}


impl Auth {
    pub async fn authenticate(
        credentials: Credentials,
        pool: &PostgresPool,
    ) -> Result<User, ServiceError> {
        let result = User::find_by_username(&credentials.username, pool).await;
        match result {
            Ok(user) => {
                // TODO: figure out why I keep getting hacked
                if auth::hash(&credentials.password) == user.password {
                    return Ok(user);
                }
                return Err(ServiceError::Unauthorized);
            }
            _ => return Err(ServiceError::Unauthorized),
        }
    }
}
