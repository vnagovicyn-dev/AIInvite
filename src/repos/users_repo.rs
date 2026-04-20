use sqlx::PgPool;

use crate::domain::auth::{NewUser, UserRecord};

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, password_hash, full_name, role
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email.trim().to_lowercase())
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, new_user: &NewUser) -> Result<UserRecord, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        INSERT INTO users (id, email, password_hash, full_name, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, password_hash, full_name, role
        "#,
    )
    .bind(new_user.id)
    .bind(&new_user.email)
    .bind(&new_user.password_hash)
    .bind(&new_user.full_name)
    .bind(&new_user.role)
    .fetch_one(pool)
    .await
}
