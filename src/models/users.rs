use crate::errors::ServiceError;
use crate::{auth, types::PostgresPool};
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

// pub struct AuthUser {
//     pub id: i32,
//     pub username: String,
//     pub password: String,
// }

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

impl User {
    pub async fn find_all(pool: &PostgresPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
                SELECT id, first_name, last_name, email, username, password, created_at
                    FROM users
                ORDER BY created_at
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &PostgresPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(username: &str, pool: &PostgresPool) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_one(&*pool)
        .await?;

        Ok(user)
    }

    pub async fn create(input: UserInput, pool: &PostgresPool) -> Result<User> {
        let result = User::find_by_username(&input.username, pool).await;
        match result {
            Ok(user) => Ok(user),
            _ => {
                let mut tx = pool.begin().await?;
                let mut user = sqlx::query_as!(
                    User,
                    r#"
                        INSERT INTO users (first_name, last_name, email, username, password) VALUES ($1, $2, $3, $4, $5)
                         RETURNING id, first_name, last_name, email, username, password, created_at
                    "#,
                    input.first_name,
                    input.last_name,
                    input.email,
                    input.username,
                    auth::hash(&input.password),
                )
                .fetch_one(&mut tx)
                .await?;
                tx.commit().await?;

                user.password = "".to_string();
                Ok(user)
            }
        }
    }

    pub async fn update(id: i32, input: UserInput, pool: &PostgresPool) -> Result<User> {
        let mut tx = pool.begin().await.unwrap();
        let mut user = sqlx::query_as!(
            User,
            r#"
                UPDATE users SET first_name = $1, last_name = $2, email = $3 WHERE id = $4
                 RETURNING id, first_name, last_name, email, username, password, created_at
            "#,
            input.first_name,
            input.last_name,
            input.email,
            id
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await.unwrap();

        user.password = "".to_string();
        Ok(user)
    }

    pub async fn delete(id: i32, pool: &PostgresPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query_as!(User, "DELETE FROM users WHERE id = $1", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected())
    }

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
