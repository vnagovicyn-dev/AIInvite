use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::models::{
    CreateHealthcheck, Healthcheck, LeadRequest, LeadRequestInput, UpdateHealthcheck,
};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn connect_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

pub async fn fetch_database_version(pool: &PgPool) -> Result<String, sqlx::Error> {
    sqlx::query_scalar("select version()").fetch_one(pool).await
}

pub async fn list_healthchecks(pool: &PgPool) -> Result<Vec<Healthcheck>, sqlx::Error> {
    sqlx::query_as::<_, Healthcheck>(
        r#"
        select id, service_name, checked_at
        from healthchecks
        order by checked_at desc, id desc
        limit 100
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_healthcheck(pool: &PgPool, id: i64) -> Result<Option<Healthcheck>, sqlx::Error> {
    sqlx::query_as::<_, Healthcheck>(
        r#"
        select id, service_name, checked_at
        from healthchecks
        where id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_healthcheck(
    pool: &PgPool,
    payload: &CreateHealthcheck,
) -> Result<Healthcheck, sqlx::Error> {
    sqlx::query_as::<_, Healthcheck>(
        r#"
        insert into healthchecks (service_name)
        values ($1)
        returning id, service_name, checked_at
        "#,
    )
    .bind(payload.service_name.trim())
    .fetch_one(pool)
    .await
}

pub async fn update_healthcheck(
    pool: &PgPool,
    id: i64,
    payload: &UpdateHealthcheck,
) -> Result<Option<Healthcheck>, sqlx::Error> {
    sqlx::query_as::<_, Healthcheck>(
        r#"
        update healthchecks
        set service_name = $2
        where id = $1
        returning id, service_name, checked_at
        "#,
    )
    .bind(id)
    .bind(payload.service_name.trim())
    .fetch_optional(pool)
    .await
}

pub async fn delete_healthcheck(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        delete from healthchecks
        where id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn create_lead_request(
    pool: &PgPool,
    payload: &LeadRequestInput,
) -> Result<LeadRequest, sqlx::Error> {
    sqlx::query_as::<_, LeadRequest>(
        r#"
        insert into lead_requests (name, phone, event_date, event_type, comment)
        values ($1, $2, $3, $4, $5)
        returning id, name, phone, event_date, event_type, comment, created_at
        "#,
    )
    .bind(payload.name.trim())
    .bind(payload.phone.trim())
    .bind(payload.event_date.trim())
    .bind(payload.event_type.trim())
    .bind(payload.comment.trim())
    .fetch_one(pool)
    .await
}
