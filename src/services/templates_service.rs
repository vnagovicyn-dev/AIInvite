use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::error::AppError,
    dto::templates::{
        TemplateCategoryResponse, TemplateItemResponse, TemplateListQuery, TemplateListResponse,
    },
    repos::templates_repo,
};

pub async fn list(
    pool: &PgPool,
    query: TemplateListQuery,
) -> Result<TemplateListResponse, AppError> {
    let normalized_query = normalize_query(query);
    let items = templates_repo::list(pool, &normalized_query)
        .await?
        .into_iter()
        .map(TemplateItemResponse::from)
        .collect::<Vec<TemplateItemResponse>>();
    let total = templates_repo::count(pool, &normalized_query).await?;

    Ok(TemplateListResponse {
        items,
        page: normalized_query.page,
        per_page: normalized_query.per_page,
        total,
    })
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<TemplateItemResponse>, AppError> {
    let template = templates_repo::find_by_id(pool, id).await?;
    Ok(template.map(TemplateItemResponse::from))
}

pub async fn list_categories(pool: &PgPool) -> Result<Vec<TemplateCategoryResponse>, AppError> {
    let categories = templates_repo::list_categories(pool)
        .await?
        .into_iter()
        .map(|category| TemplateCategoryResponse { category })
        .collect();

    Ok(categories)
}

fn normalize_query(mut query: TemplateListQuery) -> TemplateListQuery {
    if query.page == 0 {
        query.page = 1;
    }

    if query.per_page == 0 {
        query.per_page = 20;
    } else if query.per_page > 100 {
        query.per_page = 100;
    }

    query.category = query.category.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    query
}
