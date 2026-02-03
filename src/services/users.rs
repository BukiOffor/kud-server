use std::collections::{HashMap, HashSet};

use axum::extract::Multipart;

use super::*;
use crate::{dto::user::*, models::users::*};

pub async fn register_user(pool: Arc<Pool>, payload: NewUser) -> Result<Message, ModuleError> {
    let mut conn = &mut pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    let mut user: User = payload.try_into()?;

    let count = schema::users::table
        .filter(schema::users::year_joined.eq(&user.year_joined))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    user.set_reg_no(count + 1);
    diesel::insert_into(schema::users::table)
        .values(&user)
        .execute(&mut conn)
        .await?;

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

    let limit = payload.limit.unwrap_or(10);
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
) -> Result<Message, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    let target = schema::users::table.filter(schema::users::id.eq(payload.id));

    diesel::update(target)
        .set((
            payload.first_name.map(|v| schema::users::first_name.eq(v)),
            payload.last_name.map(|v| schema::users::last_name.eq(v)),
            payload.dob.map(|v| schema::users::dob.eq(v)),
        ))
        .execute(&mut conn)
        .await?;

    Ok("User updated successfully".into())
}

pub async fn delete_user(pool: Arc<Pool>, id: Uuid) -> Result<Message, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;
    diesel::delete(schema::users::table.find(id))
        .execute(&mut conn)
        .await?;

    Ok("User deleted successfully".into())
}

pub async fn deactive_user(pool: Arc<Pool>, id: Uuid) -> Result<Message, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    diesel::update(schema::users::table.find(id))
        .set(schema::users::is_active.eq(false))
        .execute(&mut conn)
        .await?;

    Ok("User deactivated successfully".into())
}

pub async fn active_user(pool: Arc<Pool>, id: Uuid) -> Result<Message, ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError(POOL_ERROR_MSG.into()))?;

    diesel::update(schema::users::table.find(id))
        .set(schema::users::is_active.eq(true))
        .execute(&mut conn)
        .await?;

    Ok("User activated successfully".into())
}

pub async fn import_users(
    pool: Arc<Pool>,
    mut multipart: Multipart,
) -> Result<Message, ModuleError> {
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
            let user: crate::dto::user::NewUser =
                result.map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
            let new_user: User = user.try_into()?;
            temp_years.insert(new_user.year_joined.to_string());
            temp_users.push(new_user);
        }
    }

    // Get existing counts for each year from database
    for year in temp_years {
        let count = schema::users::table
            .filter(schema::users::year_joined.eq(&year))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        year_counter.insert(year, count);
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

    tracing::info!("Imported {} users", total_imported);

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
        Ok(_) => Ok("User registered successfully".into()),
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
) -> Result<Message, ModuleError> {
    let mut conn = pool.get().await?;
    let password_hash = crate::helpers::password_hasher(&payload.password)?;
    diesel::update(schema::users::table)
        .filter(schema::users::email.eq(payload.email))
        .set(schema::users::password_hash.eq(password_hash))
        .execute(&mut conn)
        .await?;
    Ok("Password changed successfully".into())
}
