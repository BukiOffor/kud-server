use super::*;
use crate::models::roster::*;

use crate::dto::roster::*;

pub async fn create_roster(conn: Arc<Pool>, roster: NewRoster) -> Result<Roster, ModuleError> {
    let mut conn = conn.get().await?;
    let new_roster: Roster = roster.into();
    diesel::insert_into(crate::schema::rosters::table)
        .values(&new_roster)
        .returning(Roster::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(Into::into)
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
    conn: Arc<Pool>,
    roster: UpdateRosterRequest,
) -> Result<Roster, ModuleError> {
    let mut conn = conn.get().await?;
    let roster = diesel::update(crate::schema::rosters::table.find(roster.id))
        .set(&roster)
        .get_result::<Roster>(&mut conn)
        .await?;
    Ok(roster)
}

pub async fn get_all_rosters(conn: Arc<Pool>) -> Result<Vec<Roster>, ModuleError> {
    let mut conn = conn.get().await?;
    let rosters = crate::schema::rosters::table
        .load::<Roster>(&mut conn)
        .await?;
    Ok(rosters)
}
