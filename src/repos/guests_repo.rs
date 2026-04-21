use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::{domain::guests::Guest, dto::guests::GuestListQuery};

pub struct NewGuest {
    pub event_id: Uuid,
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub group_name: Option<String>,
    pub tags: Vec<String>,
    pub plus_one_allowed: bool,
    pub is_child: bool,
    pub vip: bool,
    pub notes: Option<String>,
}

pub struct UpdateGuestChanges {
    pub full_name: Option<String>,
    pub phone: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub group_name: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
    pub plus_one_allowed: Option<bool>,
    pub is_child: Option<bool>,
    pub vip: Option<bool>,
    pub notes: Option<Option<String>>,
}

pub async fn create(pool: &PgPool, new_guest: &NewGuest) -> Result<Guest, sqlx::Error> {
    sqlx::query_as::<_, Guest>(
        r#"
        INSERT INTO guests (
            id,
            event_id,
            full_name,
            phone,
            email,
            group_name,
            tags,
            plus_one_allowed,
            is_child,
            vip,
            notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING
            id,
            event_id,
            full_name,
            phone,
            email,
            group_name,
            tags,
            plus_one_allowed,
            is_child,
            vip,
            notes,
            created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(new_guest.event_id)
    .bind(&new_guest.full_name)
    .bind(&new_guest.phone)
    .bind(&new_guest.email)
    .bind(&new_guest.group_name)
    .bind(&new_guest.tags)
    .bind(new_guest.plus_one_allowed)
    .bind(new_guest.is_child)
    .bind(new_guest.vip)
    .bind(&new_guest.notes)
    .fetch_one(pool)
    .await
}

pub async fn list_by_event(
    pool: &PgPool,
    event_id: Uuid,
    query: &GuestListQuery,
) -> Result<Vec<Guest>, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT
            id,
            event_id,
            full_name,
            phone,
            email,
            group_name,
            tags,
            plus_one_allowed,
            is_child,
            vip,
            notes,
            created_at
        FROM guests
        WHERE event_id =
        "#,
    );
    query_builder.push_bind(event_id);

    apply_filters(&mut query_builder, query);

    query_builder.push(" ORDER BY created_at DESC");
    query_builder.push(" LIMIT ");
    query_builder.push_bind(query.per_page as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(((query.page - 1) * query.per_page) as i64);

    query_builder
        .build_query_as::<Guest>()
        .fetch_all(pool)
        .await
}

pub async fn count_by_event(
    pool: &PgPool,
    event_id: Uuid,
    query: &GuestListQuery,
) -> Result<i64, sqlx::Error> {
    let mut query_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT COUNT(*) AS total
        FROM guests
        WHERE event_id =
        "#,
    );
    query_builder.push_bind(event_id);

    apply_filters(&mut query_builder, query);

    let row: PgRow = query_builder.build().fetch_one(pool).await?;
    Ok(row.get::<i64, _>("total"))
}

pub async fn find_by_id_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
) -> Result<Option<Guest>, sqlx::Error> {
    sqlx::query_as::<_, Guest>(
        r#"
        SELECT
            g.id,
            g.event_id,
            g.full_name,
            g.phone,
            g.email,
            g.group_name,
            g.tags,
            g.plus_one_allowed,
            g.is_child,
            g.vip,
            g.notes,
            g.created_at
        FROM guests g
        INNER JOIN events e ON e.id = g.event_id
        WHERE g.id = $1
          AND g.event_id = $2
          AND e.owner_id = $3
        "#,
    )
    .bind(guest_id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id_and_event(
    pool: &PgPool,
    event_id: Uuid,
    guest_id: Uuid,
) -> Result<Option<Guest>, sqlx::Error> {
    sqlx::query_as::<_, Guest>(
        r#"
        SELECT
            id,
            event_id,
            full_name,
            phone,
            email,
            group_name,
            tags,
            plus_one_allowed,
            is_child,
            vip,
            notes,
            created_at
        FROM guests
        WHERE id = $1
          AND event_id = $2
        "#,
    )
    .bind(guest_id)
    .bind(event_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
    changes: &UpdateGuestChanges,
) -> Result<Option<Guest>, sqlx::Error> {
    sqlx::query_as::<_, Guest>(
        r#"
        UPDATE guests g
        SET
            full_name = CASE WHEN $1 THEN $2 ELSE g.full_name END,
            phone = CASE WHEN $3 THEN $4 ELSE g.phone END,
            email = CASE WHEN $5 THEN $6 ELSE g.email END,
            group_name = CASE WHEN $7 THEN $8 ELSE g.group_name END,
            tags = CASE WHEN $9 THEN $10 ELSE g.tags END,
            plus_one_allowed = CASE WHEN $11 THEN $12 ELSE g.plus_one_allowed END,
            is_child = CASE WHEN $13 THEN $14 ELSE g.is_child END,
            vip = CASE WHEN $15 THEN $16 ELSE g.vip END,
            notes = CASE WHEN $17 THEN $18 ELSE g.notes END
        FROM events e
        WHERE g.id = $19
          AND g.event_id = $20
          AND e.id = g.event_id
          AND e.owner_id = $21
        RETURNING
            g.id,
            g.event_id,
            g.full_name,
            g.phone,
            g.email,
            g.group_name,
            g.tags,
            g.plus_one_allowed,
            g.is_child,
            g.vip,
            g.notes,
            g.created_at
        "#,
    )
    .bind(changes.full_name.is_some())
    .bind(&changes.full_name)
    .bind(changes.phone.is_some())
    .bind(changes.phone.clone().flatten())
    .bind(changes.email.is_some())
    .bind(changes.email.clone().flatten())
    .bind(changes.group_name.is_some())
    .bind(changes.group_name.clone().flatten())
    .bind(changes.tags.is_some())
    .bind(&changes.tags)
    .bind(changes.plus_one_allowed.is_some())
    .bind(changes.plus_one_allowed)
    .bind(changes.is_child.is_some())
    .bind(changes.is_child)
    .bind(changes.vip.is_some())
    .bind(changes.vip)
    .bind(changes.notes.is_some())
    .bind(changes.notes.clone().flatten())
    .bind(guest_id)
    .bind(event_id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_for_owner(
    pool: &PgPool,
    owner_id: Uuid,
    event_id: Uuid,
    guest_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM guests g
        USING events e
        WHERE g.id = $1
          AND g.event_id = $2
          AND e.id = g.event_id
          AND e.owner_id = $3
        "#,
    )
    .bind(guest_id)
    .bind(event_id)
    .bind(owner_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

fn apply_filters<'a>(query_builder: &mut QueryBuilder<'a, Postgres>, query: &'a GuestListQuery) {
    if let Some(search) = &query.search {
        query_builder.push(" AND full_name ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
    }

    if let Some(vip) = query.vip {
        query_builder.push(" AND vip = ");
        query_builder.push_bind(vip);
    }

    if let Some(group_name) = &query.group_name {
        query_builder.push(" AND group_name = ");
        query_builder.push_bind(group_name);
    }
}
