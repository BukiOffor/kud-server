use crate::Connection;
use crate::dto::pagination::Pagination;
use crate::models::activity_logs::ActivityLogResponse;
use crate::{
    Pool,
    dto::pagination::PaginatedResult,
    errors::ModuleError,
    models::activity_logs::ActivityLog,
    schema::{activity_logs, users},
};
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use std::sync::Arc;
use uuid::Uuid;

pub async fn emit_log<'a>(
    payload: ActivityLog,
    mut conn: &mut Connection<'a>,
) -> Result<(), ModuleError> {
    diesel::insert_into(activity_logs::table)
        .values(&payload)
        .execute(&mut conn)
        .await?;
    tracing::info!("Log emitted successfully");
    Ok(())
}

pub async fn get_logs(
    pool: Arc<Pool>,
    pagination: Pagination,
    search: Option<String>,
) -> Result<PaginatedResult<ActivityLogResponse>, ModuleError> {
    let mut conn = pool.get().await?;
    // Build the query with joins to get user information
    let mut query = activity_logs::table
        .inner_join(users::table.on(activity_logs::user_id.eq(users::id)))
        .select((
            activity_logs::id,
            activity_logs::user_id,
            users::first_name,
            users::last_name,
            users::email,
            users::role,
            activity_logs::activity_type,
            activity_logs::created_at,
        ))
        .into_boxed();

    // Apply search filter if provided
    if let Some(ref search_term) = search {
        query = query.filter(
            users::first_name
                .ilike(format!("%{}%", search_term))
                .or(users::last_name.ilike(format!("%{}%", search_term)))
                .or(users::email.ilike(format!("%{}%", search_term))),
        );
    }

    // Get total count for pagination
    let mut count_query = activity_logs::table
        .inner_join(users::table.on(activity_logs::user_id.eq(users::id)))
        .into_boxed();

    if let Some(ref search_term) = search {
        count_query = count_query.filter(
            users::first_name
                .ilike(format!("%{}%", search_term))
                .or(users::last_name.ilike(format!("%{}%", search_term)))
                .or(users::email.ilike(format!("%{}%", search_term))),
        );
    }

    let total_count: i64 = count_query.count().get_result(&mut conn).await?;

    // Apply pagination and ordering
    let results: Vec<(
        Uuid,
        Uuid,
        String,
        String,
        String,
        crate::models::users::Role,
        crate::models::activity_logs::ActivityType,
        chrono::NaiveDateTime,
    )> = query
        .order(activity_logs::created_at.desc())
        .limit(pagination.size as i64)
        .offset(pagination.offset() as i64)
        .load(&mut conn)
        .await?;

    // Transform results into ActivityLogResponse
    let activity_logs: Vec<ActivityLogResponse> = results
        .into_iter()
        .map(
            |(id, user_id, first_name, last_name, email, role, activity_type, created_at)| {
                ActivityLogResponse {
                    id,
                    user_id,
                    user_name: format!("{} {}", first_name, last_name),
                    user_email: Some(email),
                    user_role: format!("{:?}", role),
                    first_name: Some(first_name),
                    last_name: Some(last_name),
                    activity_type: activity_type.message(),
                    created_at,
                }
            },
        )
        .collect();

    let result = PaginatedResult::new(activity_logs, total_count as i32, pagination);
    Ok(result)
}

pub async fn get_user_activity(pool: Arc<Pool>, id: Uuid) -> Result<Vec<ActivityLog>, ModuleError> {
    //Add pagination
    let mut conn = pool.get().await?;

    let res = activity_logs::table
        .filter(activity_logs::user_id.eq(id))
        .select(ActivityLog::as_select())
        .load(&mut conn)
        .await?;

    Ok(res)
}
