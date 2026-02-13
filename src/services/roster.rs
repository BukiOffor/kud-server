#![allow(deprecated)]
use super::*;
use crate::dto::roster::*;
use crate::models::activity_logs::{ActivityLog, ActivityType};
use crate::models::roster::*;
use crate::models::users::User;
use crate::models::users_roster::UsersRoster;
use axum::extract::Multipart;
use rand::seq::IndexedRandom;

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

pub async fn activate_roster(
    conn_pool: Arc<Pool>,
    roster_id: Uuid,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    let mut conn = conn_pool.get().await?;

    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let users = crate::schema::users::table
                    .filter(crate::schema::users::is_active.eq(true))
                    .select(crate::schema::users::id)
                    .load::<Uuid>(conn)
                    .await?;

                let mut roster: Roster = crate::schema::rosters::table
                    .find(roster_id)
                    .first::<Roster>(conn)
                    .await?;

                if roster.is_active {
                    return Err(ModuleError::Error("Roster is already active".into()));
                }

                if roster.end_date < chrono::Utc::now().date_naive() {
                    return Err(ModuleError::Error("Roster end date is in the past".into()));
                }

                if chrono::Utc::now().date_naive() > roster.start_date {
                    return Err(ModuleError::Error(
                        "Roster start date is in the future".into(),
                    ));
                }

                let mut user_roster = Vec::new();
                let mut available_halls: Vec<Hall> = Hall::all();
                //let mut exhausted_halls: HashSet<Hall> = HashSet::new();

                for user in users {
                    let past_halls: Vec<Hall> = crate::schema::users_rosters::table
                        .filter(crate::schema::users_rosters::user_id.eq(user))
                        .limit(5)
                        .filter(crate::schema::users_rosters::year.eq(&roster.year))
                        .select(crate::schema::users_rosters::hall)
                        .load::<Hall>(conn)
                        .await?;

                    // Assign a random hall from available ones
                    let mut assigned_hall = Hall::assign_hall(&past_halls, available_halls.clone());

                    if assigned_hall.is_none() {
                        // Fallback: assign any available hall or a random one if all are full
                        assigned_hall = Hall::all().choose(&mut rand::thread_rng()).cloned();
                    }

                    if let Some(hall) = assigned_hall {
                        user_roster.push(UsersRoster::new(
                            user,
                            roster_id,
                            hall.clone(),
                            roster.year.clone(),
                        ));
                        match hall {
                            Hall::Outside => roster.num_for_outside -= 1,
                            Hall::Basement => roster.num_for_basement -= 1,
                            Hall::HallOne => roster.num_for_hall_one -= 1,
                            Hall::Gallery => roster.num_for_gallery -= 1,
                            Hall::MainHall => roster.num_for_main_hall -= 1,
                        }

                        // Remove hall from available list if capacity is reached
                        if roster.num_for_outside <= 0 {
                            available_halls.retain(|h| h != &Hall::Outside);
                            //exhausted_halls.insert(Hall::Outside);
                        }
                        if roster.num_for_basement <= 0 {
                            available_halls.retain(|h| h != &Hall::Basement);
                            //exhausted_halls.insert(Hall::Basement);
                        }
                        if roster.num_for_hall_one <= 0 {
                            available_halls.retain(|h| h != &Hall::HallOne);
                            //exhausted_halls.insert(Hall::HallOne);
                        }
                        if roster.num_for_gallery <= 0 {
                            available_halls.retain(|h| h != &Hall::Gallery);
                            //exhausted_halls.insert(Hall::Gallery);
                        }
                        if roster.num_for_main_hall <= 0 {
                            available_halls.retain(|h| h != &Hall::MainHall);
                            // exhausted_halls.insert(Hall::MainHall);
                        }
                    }
                }

                // Delete previous assignments for this roster if any
                diesel::delete(
                    crate::schema::users_rosters::table
                        .filter(crate::schema::users_rosters::roster_id.eq(roster_id)),
                )
                .execute(conn)
                .await?;

                // Batch insert roster assignments
                diesel::insert_into(crate::schema::users_rosters::table)
                    .values(&user_roster)
                    .execute(conn)
                    .await?;

                // deactivate previous active roster
                let prev_active: Option<Roster> = crate::schema::rosters::table
                    .filter(crate::schema::rosters::is_active.eq(true))
                    .first::<Roster>(conn)
                    .await
                    .optional()?;

                if let Some(prev) = prev_active {
                    diesel::update(crate::schema::rosters::table.find(prev.id))
                        .set(crate::schema::rosters::is_active.eq(false))
                        .execute(conn)
                        .await?;
                }

                // Activate new roster
                diesel::update(crate::schema::rosters::table.find(roster_id))
                    .set(crate::schema::rosters::is_active.eq(true))
                    .execute(conn)
                    .await?;

                // update all users in the roster to have the new roster id
                for u_r in user_roster {
                    diesel::update(crate::schema::users::table.find(u_r.user_id))
                        .set(crate::schema::users::current_roster_hall.eq(u_r.hall))
                        .execute(conn)
                        .await?;
                }
                Ok::<(), ModuleError>(())
            })
        })
        .await?;

    let log = ActivityLog::new(ActivityType::RosterActivated, performer_id)
        .set_target_id(roster_id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(())
}

pub async fn share_roster(
    conn: &mut crate::Connection<'_>,
    roster_id: Uuid,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let users = crate::schema::users::table
                    .select(crate::schema::users::id)
                    .load::<Uuid>(conn)
                    .await?;

                let mut roster: Roster = crate::schema::rosters::table
                    .find(roster_id)
                    .first::<Roster>(conn)
                    .await?;

                let mut user_roster = Vec::new();
                let mut available_halls: Vec<Hall> = Hall::all();
                //let mut exhausted_halls: HashSet<Hall> = HashSet::new();

                for user in users {
                    let past_halls = crate::schema::users_rosters::table
                        .filter(crate::schema::users_rosters::user_id.eq(user))
                        .limit(5)
                        .filter(crate::schema::users_rosters::year.eq(&roster.year))
                        .select(crate::schema::users_rosters::hall)
                        .load::<Hall>(conn)
                        .await?;

                    // let past_halls = past_halls.into_iter()
                    //     .filter(|hall| !exhausted_halls.contains(hall))
                    //     .collect::<Vec<Hall>>();

                    // Assign a random hall from available ones
                    let mut assigned_hall = Hall::assign_hall(&past_halls, available_halls.clone());

                    if assigned_hall.is_none() {
                        // Fallback: assign any available hall or a random one if all are full
                        assigned_hall = available_halls.choose(&mut rand::thread_rng()).cloned();
                    }

                    if let Some(hall) = assigned_hall {
                        user_roster.push(UsersRoster::new(
                            user,
                            roster_id,
                            hall.clone(),
                            roster.year.clone(),
                        ));
                        match hall {
                            Hall::Outside => roster.num_for_outside -= 1,
                            Hall::Basement => roster.num_for_basement -= 1,
                            Hall::HallOne => roster.num_for_hall_one -= 1,
                            Hall::Gallery => roster.num_for_gallery -= 1,
                            Hall::MainHall => roster.num_for_main_hall -= 1,
                        }

                        // Remove hall from available list if capacity is reached
                        if roster.num_for_outside <= 0 {
                            available_halls.retain(|h| h != &Hall::Outside);
                            //exhausted_halls.insert(Hall::Outside);
                        }
                        if roster.num_for_basement <= 0 {
                            available_halls.retain(|h| h != &Hall::Basement);
                            //exhausted_halls.insert(Hall::Basement);
                        }
                        if roster.num_for_hall_one <= 0 {
                            available_halls.retain(|h| h != &Hall::HallOne);
                            //exhausted_halls.insert(Hall::HallOne);
                        }
                        if roster.num_for_gallery <= 0 {
                            available_halls.retain(|h| h != &Hall::Gallery);
                            //exhausted_halls.insert(Hall::Gallery);
                        }
                        if roster.num_for_main_hall <= 0 {
                            available_halls.retain(|h| h != &Hall::MainHall);
                            // exhausted_halls.insert(Hall::MainHall);
                        }
                    }
                }

                // Delete previous assignments for this roster if any
                diesel::delete(
                    crate::schema::users_rosters::table
                        .filter(crate::schema::users_rosters::roster_id.eq(roster_id)),
                )
                .execute(conn)
                .await?;

                // Batch insert roster assignments
                diesel::insert_into(crate::schema::users_rosters::table)
                    .values(&user_roster)
                    .execute(conn)
                    .await?;

                // deactivate previous active roster
                let prev_active: Option<Roster> = crate::schema::rosters::table
                    .filter(crate::schema::rosters::is_active.eq(true))
                    .first::<Roster>(conn)
                    .await
                    .optional()?;

                if let Some(prev) = prev_active {
                    diesel::update(crate::schema::rosters::table.find(prev.id))
                        .set(crate::schema::rosters::is_active.eq(false))
                        .execute(conn)
                        .await?;
                }

                // Activate new roster
                diesel::update(crate::schema::rosters::table.find(roster_id))
                    .set(crate::schema::rosters::is_active.eq(true))
                    .execute(conn)
                    .await?;

                Ok::<(), ModuleError>(())
            })
        })
        .await?;

    let log = ActivityLog::new(ActivityType::RosterActivated, performer_id)
        .set_target_id(roster_id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, conn).await?;

    Ok(())
}

pub async fn delete_roster(
    conn_pool: Arc<Pool>,
    id: Uuid,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    let mut conn = conn_pool.get().await?;
    diesel::delete(crate::schema::rosters::table.find(id))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::RosterDeleted, performer_id)
        .set_target_id(id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(())
}

pub async fn export_roster(
    id: Uuid,
    conn_pool: Arc<Pool>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let mut conn = conn_pool.get().await?;
    let roster = crate::schema::rosters::table
        .filter(crate::schema::rosters::id.eq(id))
        .first::<Roster>(&mut conn)
        .await
        .map_err(|_| ModuleError::ResourceNotFound("No active roster found".into()))?;

    export_roster_data(conn, roster.id, roster.name).await
}

pub async fn export_roster_by_hall(
    id: Uuid,
    conn_pool: Arc<Pool>,
    hall: Hall,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let mut conn = conn_pool.get().await?;
    let roster = crate::schema::rosters::table
        .filter(crate::schema::rosters::id.eq(id))
        .first::<Roster>(&mut conn)
        .await
        .map_err(|_| ModuleError::ResourceNotFound("No active roster found".into()))?;

    export_roster_data_filtered(conn, roster.id, roster.name, Some(hall)).await
}

async fn export_roster_data(
    conn: crate::Connection<'_>,
    roster_id: Uuid,
    roster_name: String,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    export_roster_data_filtered(conn, roster_id, roster_name, None).await
}

async fn export_roster_data_filtered(
    mut conn: crate::Connection<'_>,
    roster_id: Uuid,
    roster_name: String,
    filter_hall: Option<Hall>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let mut query = crate::schema::users_rosters::table
        .filter(crate::schema::users_rosters::roster_id.eq(roster_id))
        .into_boxed();

    if let Some(ref hall) = filter_hall {
        query = query.filter(crate::schema::users_rosters::hall.eq(hall));
    }

    let data = query
        .inner_join(crate::schema::users::table)
        .select((
            crate::schema::users::reg_no,
            crate::schema::users::first_name,
            crate::schema::users::last_name,
            crate::schema::users_rosters::hall,
        ))
        .load::<(String, String, String, Hall)>(&mut conn)
        .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["S/N", "Reg No", "Full Name", "Hall"])
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;

    for (i, (reg_no, first_name, last_name, hall)) in data.into_iter().enumerate() {
        let full_name = format!("{} {}", first_name, last_name);
        let hall_str = serde_json::to_string(&hall)
            .unwrap_or_else(|_| "Unknown".to_string())
            .replace("\"", "");
        wtr.write_record(&[(i + 1).to_string(), reg_no, full_name, hall_str])
            .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    }

    let csv_data = wtr
        .into_inner()
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::header::HeaderValue::from_static("text/csv"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::header::HeaderValue::from_str(&format!(
            "attachment; filename=\"roster_{}.csv\"",
            roster_name.replace(" ", "_")
        ))
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?,
    );

    Ok((headers, csv_data))
}

pub async fn view_roster_assignments(
    pool: Arc<Pool>,
    roster_id: Uuid,
) -> Result<Vec<RosterAssignmentDto>, ModuleError> {
    let mut conn = pool.get().await?;
    let assignments = crate::schema::users_rosters::table
        .filter(crate::schema::users_rosters::roster_id.eq(roster_id))
        .inner_join(crate::schema::users::table)
        .select((
            crate::schema::users_rosters::id,
            crate::schema::users_rosters::user_id,
            crate::schema::users::first_name,
            crate::schema::users::last_name,
            crate::schema::users::reg_no,
            crate::schema::users_rosters::hall,
        ))
        .load::<RosterAssignmentDto>(&mut conn)
        .await?;

    Ok(assignments)
}

pub async fn import_roster(
    pool: Arc<Pool>,
    roster_id: Uuid,
    mut multipart: Multipart,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    let mut conn = pool.get().await?;

    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let roster: Roster = crate::schema::rosters::table
                    .find(roster_id)
                    .first::<Roster>(conn)
                    .await
                    .map_err(|_| ModuleError::ResourceNotFound("Roster not found".into()))?;

                let mut user_roster = Vec::new();
                while let Some(field) = multipart
                    .next_field()
                    .await
                    .map_err(|e| ModuleError::InternalError(e.to_string().into()))?
                {
                    let data = field
                        .bytes()
                        .await
                        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
                    let body = String::from_utf8(data.to_vec())
                        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
                    let mut rdr = csv::Reader::from_reader(body.as_bytes());

                    for result in rdr.deserialize() {
                        let csv_user: CsvUserRoster =
                            result.map_err(|e| ModuleError::InternalError(e.to_string().into()))?;

                        let user = crate::schema::users::table
                            .filter(crate::schema::users::first_name.eq(&csv_user.first_name))
                            .filter(crate::schema::users::last_name.eq(&csv_user.last_name))
                            .select(User::as_select())
                            .first::<User>(conn)
                            .await
                            .optional()?;

                        if let Some(user) = user {
                            match csv_user.hall {
                                Some(hall) => {
                                    let new_assignment = UsersRoster::new(
                                        user.id,
                                        roster_id,
                                        hall,
                                        roster.year.clone(),
                                    );
                                    user_roster.push(new_assignment);
                                }
                                None => {
                                    continue;
                                }
                            }
                        } else {
                            tracing::warn!(
                                "User not found for roster assignment: {} {}",
                                csv_user.first_name,
                                csv_user.last_name
                            );
                        }
                    }
                }
                if user_roster.is_empty() {
                    return Err(ModuleError::BadRequest("could not match any user in the uploaded roster".into()));
                }
                diesel::insert_into(crate::schema::users_rosters::table)
                    .values(&user_roster)
                    .on_conflict_do_nothing()
                    .execute(conn)
                    .await?;

                for u_r in user_roster {
                    diesel::update(crate::schema::users::table.find(u_r.user_id))
                        .set(crate::schema::users::current_roster_hall.eq(u_r.hall))
                        .execute(conn)
                        .await?;
                }
                Ok::<(), ModuleError>(())
            })
        })
        .await?;

    let log = ActivityLog::new(ActivityType::RosterActivated, performer_id)
        .set_target_id(roster_id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(())
}

pub async fn update_user_hall(
    pool: Arc<Pool>,
    user_id: Uuid,
    user_roster_id: Uuid,
    hall: Hall,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;

    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                diesel::update(crate::schema::users_rosters::table.find(user_roster_id))
                    .set(crate::schema::users_rosters::hall.eq(&hall))
                    .execute(conn)
                    .await?;

                diesel::update(crate::schema::users::table.find(user_id))
                    .set(crate::schema::users::current_roster_hall.eq(&hall))
                    .execute(conn)
                    .await?;
                Ok::<(), ModuleError>(())
            })
        })
        .await?;

    let log = ActivityLog::new(ActivityType::UserHallUpdated, performer_id)
        .set_target_id(user_id)
        .set_target_type("UserRoster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User hall updated successfully".into())
}
