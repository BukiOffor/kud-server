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
                // for u_r in user_roster {
                //     diesel::update(crate::schema::users::table.find(u_r.user_id))
                //         .set(crate::schema::users::current_roster_hall.eq(u_r.hall))
                //         .execute(conn)
                //         .await?;
                // }
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

pub async fn activate_roster_gendered(
    conn_pool: Arc<Pool>,
    roster_id: Uuid,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    let mut conn = conn_pool.get().await?;

    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                // Load active users with their gender
                let users: Vec<(Uuid, Option<String>)> = crate::schema::users::table
                    .filter(crate::schema::users::is_active.eq(true))
                    .select((crate::schema::users::id, crate::schema::users::gender))
                    .load::<(Uuid, Option<String>)>(conn)
                    .await?;

                let roster: Roster = crate::schema::rosters::table
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

                // Build per-hall gender capacity tracking
                struct HallGenderSlots {
                    total_remaining: i32,
                    male_remaining: i32,
                    female_remaining: i32,
                }

                let mut hall_slots: std::collections::HashMap<Hall, HallGenderSlots> =
                    std::collections::HashMap::new();

                for hall in Hall::all() {
                    let (total, male, female) = match &hall {
                        Hall::HallOne => (
                            roster.num_for_hall_one,
                            roster.num_male_for_hall_one.unwrap_or(0),
                            roster.num_female_for_hall_one.unwrap_or(0),
                        ),
                        Hall::MainHall => (
                            roster.num_for_main_hall,
                            roster.num_male_for_main_hall.unwrap_or(0),
                            roster.num_female_for_main_hall.unwrap_or(0),
                        ),
                        Hall::Gallery => (
                            roster.num_for_gallery,
                            roster.num_male_for_gallery.unwrap_or(0),
                            roster.num_female_for_gallery.unwrap_or(0),
                        ),
                        Hall::Basement => (
                            roster.num_for_basement,
                            roster.num_male_for_basement.unwrap_or(0),
                            roster.num_female_for_basement.unwrap_or(0),
                        ),
                        Hall::Outside => (
                            roster.num_for_outside,
                            roster.num_male_for_outside.unwrap_or(0),
                            roster.num_female_for_outside.unwrap_or(0),
                        ),
                    };
                    hall_slots.insert(
                        hall,
                        HallGenderSlots {
                            total_remaining: total,
                            male_remaining: male,
                            female_remaining: female,
                        },
                    );
                }

                let mut user_roster = Vec::new();

                for (user_id, gender) in &users {
                    let past_halls: Vec<Hall> = crate::schema::users_rosters::table
                        .filter(crate::schema::users_rosters::user_id.eq(user_id))
                        .limit(5)
                        .filter(crate::schema::users_rosters::year.eq(&roster.year))
                        .select(crate::schema::users_rosters::hall)
                        .load::<Hall>(conn)
                        .await?;

                    let gender_lower = gender.as_deref().map(|s| s.to_lowercase());
                    let is_male = gender_lower.as_deref() == Some("male");
                    let is_female = gender_lower.as_deref() == Some("female");

                    // Build available halls: only halls that still have capacity
                    let available_halls: Vec<Hall> = Hall::all()
                        .into_iter()
                        .filter(|h| {
                            if let Some(slots) = hall_slots.get(h) {
                                if slots.total_remaining <= 0 {
                                    return false;
                                }
                                if is_male && slots.male_remaining <= 0 {
                                    // Check if there's general capacity (total > male + female assigned)
                                    return true; // still has total capacity, allow fallback
                                }
                                if is_female && slots.female_remaining <= 0 {
                                    return true; // still has total capacity, allow fallback
                                }
                                true
                            } else {
                                false
                            }
                        })
                        .collect();

                    // Prefer halls that have gender-specific slots available
                    let preferred_halls: Vec<Hall> = if is_male {
                        available_halls
                            .iter()
                            .filter(|h| {
                                hall_slots
                                    .get(h)
                                    .map(|s| s.male_remaining > 0)
                                    .unwrap_or(false)
                            })
                            .cloned()
                            .collect()
                    } else if is_female {
                        available_halls
                            .iter()
                            .filter(|h| {
                                hall_slots
                                    .get(h)
                                    .map(|s| s.female_remaining > 0)
                                    .unwrap_or(false)
                            })
                            .cloned()
                            .collect()
                    } else {
                        available_halls.clone()
                    };

                    // Try preferred halls first (gender-specific), then fallback to any available
                    let halls_to_try = if preferred_halls.is_empty() {
                        available_halls.clone()
                    } else {
                        preferred_halls
                    };

                    let mut assigned_hall = Hall::assign_hall(&past_halls, halls_to_try);

                    if assigned_hall.is_none() {
                        // Fallback: try any available hall
                        assigned_hall = Hall::assign_hall(&past_halls, available_halls);
                    }

                    if assigned_hall.is_none() {
                        // Last resort: random from all halls
                        assigned_hall = Hall::all().choose(&mut rand::thread_rng()).cloned();
                    }

                    if let Some(ref hall) = assigned_hall {
                        user_roster.push(UsersRoster::new(
                            *user_id,
                            roster_id,
                            hall.clone(),
                            roster.year.clone(),
                        ));

                        if let Some(slots) = hall_slots.get_mut(hall) {
                            slots.total_remaining -= 1;
                            if is_male {
                                slots.male_remaining -= 1;
                            } else if is_female {
                                slots.female_remaining -= 1;
                            }
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
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(())
}

pub async fn share_roster(
    pool: Arc<Pool>,
    roster_id: Uuid,
    performer_id: Uuid,
) -> Result<(), ModuleError> {
    let mut conn = pool.get().await?;
    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let roster: Roster = crate::schema::rosters::table
                    .find(roster_id)
                    .first::<Roster>(conn)
                    .await?;

                if !roster.is_active {
                    return Err(ModuleError::Error(
                        "Roster is not active, cannot share an inactive roster".into(),
                    ));
                }

                let user_roster = crate::schema::users_rosters::table
                    .filter(crate::schema::users_rosters::roster_id.eq(roster_id))
                    .select(UsersRoster::as_select())
                    .load::<UsersRoster>(conn)
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

    let log = ActivityLog::new(ActivityType::RosterShared, performer_id)
        .set_target_id(roster_id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;
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
            crate::schema::users::gender,
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
                    return Err(ModuleError::BadRequest(
                        "could not match any user in the uploaded roster".into(),
                    ));
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

pub async fn get_roster_stats_per_hall(
    roster_id: Uuid,
    pool: Arc<Pool>,
) -> Result<Vec<RosterStatsByHallDto>, ModuleError> {
    let mut conn = pool.get().await?;

    let roster = crate::schema::rosters::table
        .find(roster_id)
        .first::<Roster>(&mut conn)
        .await
        .map_err(|_| ModuleError::ResourceNotFound("Roster not found".into()))?;

    let mut stats = Vec::new();
    let halls = Hall::all();

    for hall in halls {
        let hall_stats = calculate_hall_stats(&mut conn, &roster, hall).await?;
        stats.push(hall_stats);
    }

    Ok(stats)
}

pub async fn get_roster_stats_for_hall(
    roster_id: Uuid,
    hall: Hall,
    pool: Arc<Pool>,
) -> Result<RosterStatsByHallDto, ModuleError> {
    let mut conn = pool.get().await?;

    let roster = crate::schema::rosters::table
        .find(roster_id)
        .first::<Roster>(&mut conn)
        .await
        .map_err(|_| ModuleError::ResourceNotFound("Roster not found".into()))?;

    calculate_hall_stats(&mut conn, &roster, hall).await
}

async fn calculate_hall_stats(
    conn: &mut crate::Connection<'_>,
    roster: &Roster,
    hall: Hall,
) -> Result<RosterStatsByHallDto, ModuleError> {
    let total_expected = match hall {
        Hall::HallOne => roster.num_for_hall_one,
        Hall::MainHall => roster.num_for_main_hall,
        Hall::Gallery => roster.num_for_gallery,
        Hall::Basement => roster.num_for_basement,
        Hall::Outside => roster.num_for_outside,
    };

    let assignments = crate::schema::users_rosters::table
        .filter(crate::schema::users_rosters::roster_id.eq(roster.id))
        .filter(crate::schema::users_rosters::hall.eq(&hall))
        .inner_join(crate::schema::users::table)
        .select((crate::schema::users::gender,))
        .load::<(Option<String>,)>(conn)
        .await?;

    let total_assigned = assignments.len() as i32;
    let total_unassigned = (total_expected - total_assigned).max(0);

    let female_count = assignments
        .iter()
        .filter(|(g,)| g.as_deref().map(|s| s.to_lowercase()) == Some("female".to_string()))
        .count() as u32;
    let male_count = assignments
        .iter()
        .filter(|(g,)| g.as_deref().map(|s| s.to_lowercase()) == Some("male".to_string()))
        .count() as u32;

    let percentage_assigned = if total_expected > 0 {
        (total_assigned as f64 / total_expected as f64) * 100.0
    } else {
        0.0
    };

    let percentage_unassigned = if total_expected > 0 {
        (total_unassigned as f64 / total_expected as f64) * 100.0
    } else {
        0.0
    };

    Ok(RosterStatsByHallDto {
        hall,
        roster_id: roster.id,
        total_expected,
        total_assigned,
        total_unassigned,
        percentage_assigned,
        percentage_unassigned,
        number_of_male: male_count,
        number_of_female: female_count,
    })
}
pub async fn add_user_to_roster(
    pool: Arc<Pool>,
    payload: AddUserToRosterRequest,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;

    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let roster: Roster = crate::schema::rosters::table
                    .find(payload.roster_id)
                    .first::<Roster>(conn)
                    .await
                    .map_err(|_| ModuleError::ResourceNotFound("Roster not found".into()))?;

                let new_assignment = UsersRoster::new(
                    payload.user_id,
                    payload.roster_id,
                    payload.hall.clone(),
                    roster.year.clone(),
                );

                diesel::insert_into(crate::schema::users_rosters::table)
                    .values(&new_assignment)
                    .on_conflict_do_nothing()
                    .execute(conn)
                    .await?;

                diesel::update(crate::schema::users::table.find(payload.user_id))
                    .set(crate::schema::users::current_roster_hall.eq(payload.hall.clone()))
                    .execute(conn)
                    .await?;

                Ok::<(), ModuleError>(())
            })
        })
        .await?;

    let log = ActivityLog::new(ActivityType::RosterActivated, performer_id) // Using RosterActivated as a placeholder if no specific type exists
        .set_target_id(payload.roster_id)
        .set_target_type("Roster".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok(Message::new("User added to roster successfully", None))
}

pub async fn get_user_roster_history(
    pool: Arc<Pool>,
    user_id: Uuid,
) -> Result<Vec<UserRosterHistoryDto>, ModuleError> {
    let mut conn = pool.get().await?;

    let history = crate::schema::users_rosters::table
        .filter(crate::schema::users_rosters::user_id.eq(user_id))
        .inner_join(crate::schema::rosters::table)
        .select((
            crate::schema::users_rosters::id,
            crate::schema::users_rosters::roster_id,
            crate::schema::rosters::name,
            crate::schema::users_rosters::hall,
            crate::schema::users_rosters::year,
            crate::schema::rosters::start_date,
            crate::schema::rosters::end_date,
            crate::schema::rosters::is_active,
            crate::schema::users_rosters::created_at,
        ))
        .order(crate::schema::users_rosters::created_at.desc())
        .load::<UserRosterHistoryDto>(&mut conn)
        .await?;

    Ok(history)
}
