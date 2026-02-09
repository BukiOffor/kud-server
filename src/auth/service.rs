use crate::dto::user::UserDto;
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use diesel_async::RunQueryDsl;

use super::jwt::*;
use super::*;
use crate::models::activity_logs::{ActivityLog, ActivityType};

pub async fn login(
    jar: CookieJar,
    pool: Arc<Pool>,
    payload: LoginPayload,
) -> Result<(CookieJar, UserDto), ModuleError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ModuleError::InternalError("could not get pool connection".into()))?;
    let user =
        crate::services::users::find_user_by_email_or_username(&mut conn, &payload.user).await?;

    if let Some(user) = user {
        let is_valid = crate::helpers::password_verfier(&payload.password, &user.password);
        if !is_valid {
            return Err(ModuleError::AuthError);
        }

        let token = create_session_token(user.id, user.role.clone())?;

        let cookie = Cookie::build(("access_token", token.access_token))
            .http_only(true)
            .secure(true) // ❌❌❌ change this to true for production
            .path("/")
            .max_age(cookie::time::Duration::days(7))
            .same_site(cookie::SameSite::None)
            .build();

        let refresh_cookie = Cookie::build(("refresh_token", token.refresh_token))
            .path("/")
            .http_only(true)
            .secure(true) // ❌❌❌ change this to true for production
            .same_site(cookie::SameSite::None)
            .max_age(cookie::time::Duration::days(8))
            .build();

        let updated_jar = jar.add(cookie).add(refresh_cookie);

        diesel::update(schema::users::table.filter(schema::users::id.eq(user.id)))
            .set(schema::users::last_seen.eq(chrono::Utc::now().naive_utc()))
            .execute(&mut conn)
            .await?;

        let log = ActivityLog::new(ActivityType::UserLogin, user.id)
            .set_target_id(user.id)
            .set_target_type("User".into())
            .finish();
        crate::services::activity_logs::emit_log(log, &mut conn).await?;

        Ok((updated_jar, user))
    } else {
        Err(ModuleError::AuthError)
    }
}
