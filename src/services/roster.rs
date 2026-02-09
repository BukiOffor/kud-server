use super::*;
use crate::dto::roster::*;
use crate::models::activity_logs::{ActivityLog, ActivityType};
use crate::models::roster::*;

pub async fn create_roster(
    conn_pool: Arc<Pool>,
    roster: NewRoster,
    performer_id: Uuid,
) -> Result<Roster, ModuleError> {
    let mut conn = conn_pool.get().await?;
    let new_roster: Roster = roster.into();
    let roster: Roster = diesel::insert_into(crate::schema::rosters::table)
        .values(&new_roster)
        .returning(Roster::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Into::<ModuleError>::into)?;

    let log = ActivityLog::new(ActivityType::RosterCreated, performer_id)
        .set_target_id(roster.id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(roster)
}

pub async fn get_roster(conn: Arc<Pool>, id: Uuid) -> Result<Roster, ModuleError> {
    let mut conn = conn.get().await?;
    let roster = crate::schema::rosters::table
        .find(id)
        .first::<Roster>(&mut conn)
        .await
        .map_err(|_| ModuleError::ResourceNotFound("Roster not found".into()))?;
    Ok(roster)
}

pub async fn update_roster(
    conn_pool: Arc<Pool>,
    roster_req: UpdateRosterRequest,
    performer_id: Uuid,
) -> Result<Roster, ModuleError> {
    let mut conn = conn_pool.get().await?;
    let roster = diesel::update(crate::schema::rosters::table.find(roster_req.id))
        .set(&roster_req)
        .get_result::<Roster>(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::RosterUpdated, performer_id)
        .set_target_id(roster.id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(roster)
}

pub async fn get_all_rosters(conn: Arc<Pool>) -> Result<Vec<Roster>, ModuleError> {
    let mut conn = conn.get().await?;
    let rosters = crate::schema::rosters::table
        .load::<Roster>(&mut conn)
        .await?;
    Ok(rosters)
}
