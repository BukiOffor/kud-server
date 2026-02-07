use super::*;
use crate::models::users::User;
use crate::{dto::attendance::*, models::user_attendance::*};
use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::Africa::Lagos;
use uuid::Uuid;

pub async fn admin_sign_attendance(
    pool: Arc<Pool>,
    admin_id: Uuid,
    worker_id: Uuid,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;
    let now = now_in_nigeria();
    let is_valid = is_within_attendance_window(now);
    if !is_valid {
        return Err(ModuleError::Error("Attendance window is closed".into()));
    }
    let today = Lagos
        .from_utc_datetime(&chrono::Utc::now().naive_utc())
        .date_naive();
    let mut user_attendance = UserAttendance::new(worker_id, today);
    user_attendance.set_marked_by(admin_id);
    diesel::insert_into(schema::user_attendance::table)
        .values(&user_attendance)
        .execute(&mut conn)
        .await?;
    Ok(Message::new("Attendance signed successfully", None))
}

pub async fn sign_attendance(
    pool: Arc<Pool>,
    user_id: Uuid,
    payload: SignAttendanceRequest,
) -> Result<Message<()>, ModuleError> {
    let mut conn = pool.get().await?;
    let user: Option<User> = fetch!(
        schema::users::table,
        schema::users::id,
        user_id,
        User,
        &mut conn
    );
    let user = match user {
        Some(user) => user,
        None => return Err(ModuleError::Error("User not found".into())),
    };
    if let Some(device_id) = user.device_id {
        if device_id != payload.device_id {
            return Err(ModuleError::Error("Device ID does not match".into()));
        }
    } else {
        diesel::update(schema::users::table)
            .filter(schema::users::id.eq(user_id))
            .set(schema::users::device_id.eq(&payload.device_id))
            .execute(&mut conn)
            .await?;
    };
    let now = now_in_nigeria();
    is_valid_attempt(now, payload)?;

    let today = Lagos
        .from_utc_datetime(&chrono::Utc::now().naive_utc())
        .date_naive();

    let user_attendance = UserAttendance::new(user_id, today);
    diesel::insert_into(schema::user_attendance::table)
        .values(&user_attendance)
        .execute(&mut conn)
        .await?;
    Ok(Message::new(
        "Attendance signed successfully, Welcome to church",
        None,
    ))
}

fn now_in_nigeria() -> chrono::DateTime<chrono_tz::Tz> {
    Lagos.from_utc_datetime(&chrono::Utc::now().naive_utc())
}

pub fn is_valid_attempt(
    now: chrono::DateTime<chrono_tz::Tz>,
    payload: SignAttendanceRequest,
) -> Result<(), ModuleError> {
    let weekday = now.weekday();
    let hour = now.hour();
    let minute = now.minute();

    match weekday {
        chrono::Weekday::Sun => {
            // Sunday: anytime
            let church_location = crate::CHIDA_LOCATION
                .get()
                .ok_or(ModuleError::Error("Church location not set".into()))?;
            if !is_within_radius(payload.location, church_location.clone(), 150.0) {
                tracing::warn!("User is not within radius");
                return Err(ModuleError::Error(
                    "User is not within the church radius".into(),
                ));
            }
            Ok(())
        }
        chrono::Weekday::Tue => {
            // Wednesday: 16:30 → 18:00
            let minutes_since_midnight = hour * 60 + minute;
            let start = 18 * 60 + 30; // 6:30 PM
            let end = 24 * 60; // 8:00 PM
            let is_meeting_time = minutes_since_midnight >= start && minutes_since_midnight <= end;
            if !is_meeting_time {
                return Err(ModuleError::Error("Attendance window is closed".into()));
            }
            let church_location = crate::HOME_LOCATION_CHECKIN_RADIUS
                .get()
                .ok_or(ModuleError::Error("Church location not set".into()))?;
            if !is_within_radius(payload.location, church_location.clone(), 100.0) {
                tracing::warn!("User is not within radius");
                return Err(ModuleError::Error(
                    "User is not within checkin radius".into(),
                ));
            }
            Ok(())
        }
        chrono::Weekday::Wed => {
            // Wednesday: 16:30 → 18:00
            let minutes_since_midnight = hour * 60 + minute;
            let start = 16 * 60 + 30; // 4:30 PM
            let end = 18 * 60; // 6:00 PM
            let is_meeting_time = minutes_since_midnight >= start && minutes_since_midnight <= end;
            if !is_meeting_time {
                return Err(ModuleError::Error("Attendance window is closed".into()));
            }
            let church_location = crate::DOA_LOCATION
                .get()
                .ok_or(ModuleError::Error("Church location not set".into()))?;
            if !is_within_radius(payload.location, church_location.clone(), 150.0) {
                tracing::warn!("User is not within radius");
                return Err(ModuleError::Error(
                    "User is not within checkin radius".into(),
                ));
            }
            Ok(())
        }

        _ => Err(ModuleError::Error(
            "You can only sign attendance on Sundays and Wednesdays".into(),
        )), // All other days
    }
}

pub fn is_within_radius(point1: GeoPoint, point2: GeoPoint, radius: f64) -> bool {
    let distance = helpers::haversine_meters(point1, point2);
    distance <= radius
}

fn is_within_attendance_window(now: chrono::DateTime<chrono_tz::Tz>) -> bool {
    let weekday = now.weekday();
    let hour = now.hour();
    let minute = now.minute();
    match weekday {
        chrono::Weekday::Sun => true,
        chrono::Weekday::Wed => {
            // Wednesday: 16:30 → 19:45
            let minutes_since_midnight = hour * 60 + minute;
            let start = 16 * 60 + 30; // 4:30 PM
            let end = 19 * 60 + 45; // 7:45 PM
            minutes_since_midnight >= start && minutes_since_midnight <= end
        }
        _ => false,
    }
}
