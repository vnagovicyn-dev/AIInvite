use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::{domain::templates::Template, dto::templates::TemplateListQuery};

pub async fn list(pool: &PgPool, query: &TemplateListQuery) -> Result<Vec<Template>, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT id, code, name, category, preview_url, is_active, created_at
        FROM templates
        WHERE 1 = 1
        "#,
    );

    apply_filters(&mut query_builder, query);

    query_builder.push(" ORDER BY is_active DESC, created_at DESC");
    query_builder.push(" LIMIT ");
    query_builder.push_bind(query.per_page as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(((query.page - 1) * query.per_page) as i64);

    query_builder
        .build_query_as::<Template>()
        .fetch_all(pool)
        .await
}

pub async fn count(pool: &PgPool, query: &TemplateListQuery) -> Result<i64, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT COUNT(*) AS total
        FROM templates
        WHERE 1 = 1
        "#,
    );

    apply_filters(&mut query_builder, query);

    let row: PgRow = query_builder.build().fetch_one(pool).await?;
    Ok(row.get::<i64, _>("total"))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Template>, sqlx::Error> {
    sqlx::query_as::<_, Template>(
        r#"
        SELECT id, code, name, category, preview_url, is_active, created_at
        FROM templates
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_categories(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT category
        FROM templates
        ORDER BY category ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| row.get::<String, _>("category"))
        .collect())
}

fn apply_filters<'a>(query_builder: &mut QueryBuilder<'a, Postgres>, query: &'a TemplateListQuery) {
    if let Some(category) = &query.category {
        query_builder.push(" AND category = ");
        query_builder.push_bind(category);
    }

    if let Some(is_active) = query.is_active {
        query_builder.push(" AND is_active = ");
        query_builder.push_bind(is_active);
    }
}
