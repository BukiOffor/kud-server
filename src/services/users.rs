use std::collections::{HashMap, HashSet};

use axum::extract::Multipart;
use chrono::Datelike;

use super::*;
use crate::models::activity_logs::{ActivityLog, ActivityType};
use crate::{dto::user::*, models::users::*};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;

pub async fn seed_default_admin(pool: Arc<Pool>) -> Result<(), ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    let count = schema::users::table
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    if count > 0 {
        return Ok(());
    }

    tracing::info!("Seeding default admin user...");

    let password_hash = crate::helpers::password_hasher("password")?;
    let year = chrono::Local::now().year().to_string();

    let admin = User {
        id: Uuid::now_v7(),
        username: Some("admin".to_string()),
        reg_no: format!("{}/KUD/001", year),
        first_name: "Admin".to_string(),
        last_name: "User".to_string(),
        email: "admin@kud.com".to_string(),
        password: password_hash,
        dob: None,
        avatar_url: None,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        year_joined: year,
        current_roster_hall: None,
        current_roster_allocation: None,
        role: Role::Admin,
        last_seen: None,
        device_id: None,
        is_active: true,
        gender: None,
        address: None,
        city: None,
        state: None,
        country: None,
        phone: None,
        hall_derivation: 0,
    };

    diesel::insert_into(schema::users::table)
        .values(&admin)
        .execute(&mut conn)
        .await?;

    tracing::info!("Default admin seeded successfully.");
    Ok(())
}

pub async fn register_user(
    pool: Arc<Pool>,
    payload: NewUser,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    let mut user: User = payload.try_into()?;

    let max_reg_no = schema::users::table
        .filter(schema::users::year_joined.eq(&user.year_joined))
        .select(schema::users::reg_no)
        .order(schema::users::reg_no.desc())
        .first::<String>(&mut conn)
        .await
        .optional()?;

    let next_code = match max_reg_no {
        Some(reg) => {
            let last_part = reg.split('/').last().unwrap_or("0");
            last_part.parse::<i64>().unwrap_or(0) + 1
        }
        None => 1,
    };

    user.set_reg_no(next_code);
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::UserCreated, performer_id)
        .set_target_id(user.id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User registered successfully".into())
}

pub async fn find_user_by_email_or_username(
    conn: &mut crate::Connection<'_>,
    identifier: &str,
) -> Result<Option<UserDto>, ModuleError> {
    let user = schema::users::table
        .filter(
            schema::users::email
                .eq(identifier)
                .or(schema::users::username.eq(identifier)),
        )
        .select(UserDto::as_select())
        .first::<UserDto>(conn)
        .await
        .optional()?;

    Ok(user)
}

pub async fn get_user(pool: Arc<Pool>, id: Uuid) -> Result<UserDto, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    let user = schema::users::table
        .find(id)
        .select(UserDto::as_select())
        .first::<UserDto>(&mut conn)
        .await
        .optional()?;
    match user {
        Some(u) => Ok(u),
        None => Err(ModuleError::ResourceNotFound("User not found".into())),
    }
}

pub async fn get_all_users(
    pool: Arc<Pool>,
    payload: UserFilter,
) -> Result<Vec<UserDto>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    let mut query = schema::users::table.into_boxed();

    if let Some(search) = payload.search {
        query = query.filter(
            schema::users::first_name
                .ilike(format!("%{}%", search))
                .or(schema::users::last_name.ilike(format!("%{}%", search)))
                .or(schema::users::email.ilike(format!("%{}%", search)))
                .or(schema::users::username.ilike(format!("%{}%", search))),
        );
    }

    let limit = payload.limit.unwrap_or(300);
    let offset = (payload.page.unwrap_or(1) - 1) * limit;

    let users = query
        .limit(limit)
        .offset(offset)
        .select(UserDto::as_select())
        .load::<UserDto>(&mut conn)
        .await?;

    Ok(users)
}

pub async fn update_user(
    pool: Arc<Pool>,
    payload: UpdateUserRequest,
    id: Uuid,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    if payload.is_empty() {
        return Err(ModuleError::BadRequest("No fields to update".into()));
    }

    let target = schema::users::table.filter(schema::users::id.eq(id));

    let result = diesel::update(target)
        .set((
            payload.first_name.map(|v| schema::users::first_name.eq(v)),
            payload.last_name.map(|v| schema::users::last_name.eq(v)),
            payload.dob.map(|v| schema::users::dob.eq(v)),
            payload.gender.map(|v| schema::users::gender.eq(v)),
            payload.phone.map(|v| schema::users::phone.eq(v)),
            payload.address.map(|v| schema::users::address.eq(v)),
            payload.city.map(|v| schema::users::city.eq(v)),
            payload.state.map(|v| schema::users::state.eq(v)),
            payload.country.map(|v| schema::users::country.eq(v)),
        ))
        .execute(&mut conn)
        .await;
    match result {
        Ok(0) => Err(ModuleError::Error("User not found".into())),
        Ok(_) => {
            let log = ActivityLog::new(ActivityType::UserUpdated, performer_id)
                .set_target_id(id)
                .set_target_type("User".into())
                .finish();
            crate::services::activity_logs::emit_log(log, &mut conn).await?;

            Ok("User updated successfully".into())
        }
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            return Err(ModuleError::Error(
                format!(
                    "Duplicate value for {}",
                    info.constraint_name().unwrap_or("field")
                )
                .into(),
            ));
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn admin_update_user(
    pool: Arc<Pool>,
    mut payload: AdminUpdateUserRequest,
    id: Uuid,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    if payload.is_empty() {
        return Err(ModuleError::BadRequest("No fields to update".into()));
    }

    let target = schema::users::table.filter(schema::users::id.eq(id));

    if let Some(password) = payload.password.take() {
        let hashed_password = helpers::password_hasher(&password)?;
        payload.password = Some(hashed_password);
    }

    let result = diesel::update(target)
        .set((
            payload.first_name.map(|v| schema::users::first_name.eq(v)),
            payload.last_name.map(|v| schema::users::last_name.eq(v)),
            payload.dob.map(|v| schema::users::dob.eq(v)),
            payload.gender.map(|v| schema::users::gender.eq(v)),
            payload.phone.map(|v| schema::users::phone.eq(v)),
            payload.address.map(|v| schema::users::address.eq(v)),
            payload.city.map(|v| schema::users::city.eq(v)),
            payload.state.map(|v| schema::users::state.eq(v)),
            payload.country.map(|v| schema::users::country.eq(v)),
            payload.role.map(|v| schema::users::role.eq(v)),
            payload.email.map(|v| schema::users::email.eq(v)),
            payload
                .year_joined
                .map(|v| schema::users::year_joined.eq(v)),
            payload.password.map(|v| schema::users::password_hash.eq(v)),
        ))
        .execute(&mut conn)
        .await;
    match result {
        Ok(0) => Err(ModuleError::Error("User not found".into())),
        Ok(_) => {
            let log = ActivityLog::new(ActivityType::UserUpdated, performer_id)
                .set_target_id(id)
                .set_target_type("User".into())
                .finish();
            crate::services::activity_logs::emit_log(log, &mut conn).await?;

            Ok("User updated successfully".into())
        }
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            return Err(ModuleError::Error(
                format!(
                    "Duplicate value for {}",
                    info.constraint_name().unwrap_or("field")
                )
                .into(),
            ));
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_user(
    pool: Arc<Pool>,
    id: Uuid,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    diesel::delete(schema::users::table.find(id))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::UserUpdated, performer_id) // Using UserUpdated for delete as well, or should I add UserDeleted?
        .set_target_id(id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User deleted successfully".into())
}

pub async fn deactivate_user(
    pool: Arc<Pool>,
    id: Uuid,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    diesel::update(schema::users::table.find(id))
        .set(schema::users::is_active.eq(false))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::UserDeactivation, performer_id)
        .set_target_id(id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User deactivated successfully".into())
}

pub async fn activate_user(
    pool: Arc<Pool>,
    id: Uuid,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    diesel::update(schema::users::table.find(id))
        .set(schema::users::is_active.eq(true))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::UserActivation, performer_id)
        .set_target_id(id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User activated successfully".into())
}

pub async fn update_user_role(
    pool: Arc<Pool>,
    id: Uuid,
    payload: UpdateUserRoleRequest,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    diesel::update(schema::users::table.find(id))
        .set(schema::users::role.eq(payload.role))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::UserUpdated, performer_id)
        .set_target_id(id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("User role updated successfully".into())
}

pub async fn import_users(
    pool: Arc<Pool>,
    mut multipart: Multipart,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;

    let mut year_counter: HashMap<String, i64> = HashMap::new();
    let mut total_imported = 0;
    let mut users = Vec::new();

    let mut temp_years = HashSet::new();
    let mut temp_users = Vec::new();

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
            let mut user: crate::dto::user::CsvUser =
                result.map_err(|e| ModuleError::InternalError(e.to_string().into()))?;

            user.validate();
            let new_user: User = user.to_new_user()?.try_into()?;
            temp_years.insert(new_user.year_joined.to_string());
            temp_users.push(new_user);
        }
    }
    // Get starting counts for each year from database based on max existing reg_no
    for year in temp_years {
        let max_reg_no = schema::users::table
            .filter(schema::users::year_joined.eq(&year))
            .select(schema::users::reg_no)
            .order(schema::users::reg_no.desc())
            .first::<String>(&mut conn)
            .await
            .optional()?;

        let next_code = match max_reg_no {
            Some(reg) => {
                let last_part = reg.split('/').last().unwrap_or("0");
                last_part.parse::<i64>().unwrap_or(0)
            }
            None => 0,
        };
        year_counter.insert(year, next_code);
    }

    for mut user in temp_users {
        let year_key = user.year_joined.to_string();
        // Get or initialize counter for this year
        let counter = year_counter.entry(year_key.clone()).or_insert(0);
        // Increment counter and set reg_no
        *counter += 1;
        user.set_reg_no(*counter);
        users.push(user);
        total_imported += 1;
    }

    // Insert all users
    let tx: Result<(), ModuleError> = conn
        .build_transaction()
        .run(|conn| {
            Box::pin(async move {
                diesel::insert_into(schema::users::table)
                    .values(&users)
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .await;

    match tx {
        Ok(_) => {
            let log = ActivityLog::new(ActivityType::UserImported, performer_id)
                .set_details(serde_json::json!({ "total_imported": total_imported }))
                .finish();
            crate::services::activity_logs::emit_log(log, &mut conn).await?;

            Ok("User registered successfully".into())
        }
        Err(e) => Err(e),
    }
}

pub async fn export_users(
    pool: Arc<Pool>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let mut conn = pool.get().await?;

    let users = schema::users::table
        .select(UserDto::as_select())
        .load::<UserDto>(&mut conn)
        .await?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&[
        "Usher No",
        "First Name",
        "Last Name",
        "Email",
        "Phone",
        "Date of Birth",
        "Gender",
        "Year of Entry",
    ])
    .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    for user in users {
        let row = vec![
            user.reg_no,
            user.first_name,
            user.last_name,
            user.email,
            user.phone.unwrap_or("N/A".to_string()),
            user.dob.map(|d| d.to_string()).unwrap_or("N/A".to_string()),
            user.gender.unwrap_or("N/A".to_string()),
            user.year_joined,
        ];
        // write_record expects &[&str]
        let row_refs: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
        wtr.write_record(&row_refs)
            .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    }
    let data = wtr
        .into_inner()
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        "attachment; filename=\"users.csv\"".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/csv".parse().unwrap(),
    );
    Ok((headers, data))
}

pub async fn change_password(
    pool: Arc<Pool>,
    payload: ChangePasswordRequest,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;
    let password_hash = crate::helpers::password_hasher(&payload.password)?;
    let user_id = schema::users::table
        .filter(schema::users::email.eq(&payload.email))
        .select(schema::users::id)
        .first::<Uuid>(&mut conn)
        .await?;

    diesel::update(schema::users::table.find(user_id))
        .set(schema::users::password_hash.eq(password_hash))
        .execute(&mut conn)
        .await?;

    let log = ActivityLog::new(ActivityType::PasswordChanged, performer_id)
        .set_target_id(user_id)
        .set_target_type("User".into())
        .finish();
    crate::services::activity_logs::emit_log(log, &mut conn).await?;

    Ok("Password changed successfully".into())
}

pub async fn reset_user_device_id(
    user_id: Uuid,
    pool: Arc<Pool>,
    performer_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    let result = diesel::update(schema::users::table)
        .filter(schema::users::id.eq(user_id))
        .set(schema::users::device_id.eq(Option::<String>::None))
        .execute(&mut conn)
        .await;
    match result {
        Ok(0) => Err(ModuleError::Error("User not found".into())),
        Ok(_) => {
            let log = ActivityLog::new(ActivityType::DeviceReset, performer_id)
                .set_target_id(user_id)
                .set_target_type("User".into())
                .finish();
            crate::services::activity_logs::emit_log(log, &mut conn).await?;

            Ok("User device ID reset successfully".into())
        }
        Err(e) => Err(ModuleError::Error(e.to_string().into())),
    }
}
