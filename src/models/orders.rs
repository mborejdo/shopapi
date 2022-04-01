use crate::types::PostgresPool;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct OrderInput {
    pub name: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Order {
    pub async fn find_all(pool: &PostgresPool) -> Result<Vec<Order>> {
        let order = sqlx::query_as!(
            Order,
            r#"
              SELECT id, name, updated_at, created_at
                  FROM orders
              ORDER BY updated_at
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(order)
    }

    pub async fn find_by_id(id: i32, pool: &PostgresPool) -> Result<Order> {
        let order = sqlx::query_as!(
            Order,
            r#"
              SELECT * FROM orders WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(order)
    }

    pub async fn create(input: OrderInput, pool: &PostgresPool) -> Result<Order> {
        let mut tx = pool.begin().await?;
        let order = sqlx::query_as!(
            Order,
            r#"
              INSERT INTO orders (name) VALUES ($1)
               RETURNING id, name, updated_at, created_at
            "#,
            input.name
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(order)
    }

    pub async fn update(id: i32, input: OrderInput, pool: &PostgresPool) -> Result<Order> {
        let mut tx = pool.begin().await.unwrap();
        let order = sqlx::query_as!(
            Order,
            r#"
              UPDATE orders SET name = $1 WHERE id = $2
               RETURNING id, name, updated_at, created_at
            "#,
            input.name,
            id
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await.unwrap();

        Ok(order)
    }

    pub async fn delete(id: i32, pool: &PostgresPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query_as!(Order, "DELETE FROM orders WHERE id = $1", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected())
    }
}
