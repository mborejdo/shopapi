use crate::types::PostgresPool;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct ImageInput {
    pub name: String,
    pub path: String,
    pub productId: i32,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub productId: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Image {
    pub async fn find_all(pool: &PostgresPool) -> Result<Vec<Image>> {
        let images = sqlx::query_as!(
            Image,
            r#"
              SELECT id, name, path, productId, updated_at, created_at
                  FROM images
               ORDER BY updated_at
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(images)
    }

    pub async fn find_by_id(id: i32, pool: &PostgresPool) -> Result<Image> {
        let image = sqlx::query_as!(
            Image,
            r#"
              SELECT * FROM images WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(image)
    }

    // pub async fn find_by_username(name: i32, pool: &PostgresPool) -> Result<Images> {
    //     let images = sqlx::query_as!(
    //         Images,
    //         r#"
    //           SELECT * FROM imagess WHERE name = $1
    //         "#,
    //         name
    //     )
    //     .fetch_one(&*pool)
    //     .await?;

    //     Ok(images)
    // }

    pub async fn create(input: ImageInput, pool: &PostgresPool) -> Result<Image> {
        let mut tx = pool.begin().await?;
        let image = sqlx::query_as!(
            Image,
            r#"
              INSERT INTO images (name, path, productId) VALUES ($1, $2, $3)
               RETURNING id, name, path, productId, updated_at, created_at
            "#,
            input.name,
            input.path,
            input.productId
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(image)
    }

    pub async fn update(id: i32, input: ImageInput, pool: &PostgresPool) -> Result<Image> {
        let mut tx = pool.begin().await.unwrap();
        let image = sqlx::query_as!(
            Image,
            r#"
              UPDATE images SET name = $1, path = $2, productId = $3 WHERE id = $4
               RETURNING id, name, path, productId, updated_at, created_at
            "#,
            input.name,
            input.path,
            input.productId,
            id
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await.unwrap();

        Ok(image)
    }

    pub async fn delete(id: i32, pool: &PostgresPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query_as!(Image, "DELETE FROM images WHERE id = $1", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected())
    }
}
