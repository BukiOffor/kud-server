use crate::dto::{analytics::*, user::UserDto};
use chrono::NaiveDate;
use diesel::dsl::count_distinct;
use diesel_async::AsyncConnection;

use super::*;

// total number of ushers
// total number of users present on a specfic day and the absentees
// list of all ushers and their respective information
// overall attendance rate for admin and regular ushers
// list of upcoming birtdays

// get attendance for a user
pub async fn fetch_user_attendance(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
    user_id: uuid::Uuid,
) -> Result<Message<UserAttendanceHistory>, ModuleError> {
    use crate::dto::attendance::UserAttendanceDto;
    use crate::schema::{user_attendance, users};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    // 1. Fetch user details
    let user = users::table
        .find(user_id)
        .select(UserDto::as_select())
        .first::<UserDto>(conn)
        .await
        .optional()?;

    let user = match user {
        Some(u) => u,
        None => return Err(ModuleError::ResourceNotFound("User not found".into())),
    };

    // 2. Fetch all attendance records for the user
    let history = user_attendance::table
        .filter(user_attendance::user_id.eq(user_id))
        .order(user_attendance::date.desc())
        .select(UserAttendanceDto::as_select())
        .load::<UserAttendanceDto>(conn)
        .await?;

    // 3. Calculate statistics
    // We define total days as unique dates in user_attendance table (global events)
   let total_days: i64 = user_attendance::table
    .select(count_distinct(user_attendance::date))
    .get_result(conn)
    .await?;

    tracing::debug!("Total days: {}", total_days);

    let days_present = history.len() as i64;
    let rate = if total_days == 0 {
        0.0
    } else {
        (days_present as f64 / total_days as f64) * 100.0
    };

    let summary = AttendanceSummary {
        total_days,
        days_present,
        rate,
    };

    Ok(Message::new(
        "User attendance history retrieved successfully",
        Some(UserAttendanceHistory {
            user,
            history,
            summary,
        }),
    ))
}

pub async fn fetch_total_users(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
) -> Result<Message<Vec<UserDto>>, ModuleError> {
    let total_users = schema::users::table
        .select(UserDto::as_select())
        .load::<UserDto>(conn)
        .await?;
    Ok(Message::new(
        "Record retrieved successfully",
        Some(total_users),
    ))
}

pub async fn fetch_users_present_on_a_specific_day(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
    date: NaiveDate,
) -> Result<Message<UserPresentStats>, ModuleError> {
    use crate::schema::{user_attendance, users};
    use diesel_async::RunQueryDsl;

    let does_day_exist: Vec<NaiveDate> = RunQueryDsl::load(
        user_attendance::table
            .filter(user_attendance::date.eq(date))
            .select(user_attendance::date)
            .distinct(),
        conn,
    )
    .await?;

    if does_day_exist.is_empty() {
        return Err(ModuleError::ResourceNotFound(
            "No meeting or event was held on this date".into(),
        ));
    }

    // 1. Fetch all active users
    let active_users: Vec<UserDto> = RunQueryDsl::load(
        users::table
            //.filter(users::is_active.eq(true))
            .select(UserDto::as_select()),
        conn,
    )
    .await?;

    // 2. Fetch all unique user IDs present on the specific day
    let present_user_ids: Vec<uuid::Uuid> = RunQueryDsl::load(
        user_attendance::table
            .filter(user_attendance::date.eq(date))
            .select(user_attendance::user_id)
            .distinct(),
        conn,
    )
    .await?;
    // 3. Partition users into presentees and absentees
    let (presentees, absentees): (Vec<UserDto>, Vec<UserDto>) = active_users
        .into_iter()
        .partition(|u| present_user_ids.contains(&u.id));

    let stats = UserPresentStats {
        presentees,
        absentees,
        date,
    };

    Ok(Message::new("Record retrieved successfully", Some(stats)))
}

pub async fn fetch_upcoming_birthdays(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
) -> Result<Message<Vec<UserDto>>, ModuleError> {
    use crate::schema::users;
    use chrono::{Datelike, Local};
    use diesel_async::RunQueryDsl;

    // 1. Fetch all active users with a date of birth
    let active_users: Vec<UserDto> = RunQueryDsl::load(
        users::table
            .filter(users::is_active.eq(true))
            .filter(users::dob.is_not_null())
            .select(UserDto::as_select()),
        conn,
    )
    .await?;

    let today = Local::now().naive_local().date();
    let current_month = today.month();

    let mut upcoming_birthdays: Vec<UserDto> = active_users
        .into_iter()
        .filter(|u| {
            if let Some(dob) = u.dob {
                dob.date().month() == current_month
            } else {
                false
            }
        })
        .collect();

    // Sort by day of month
    upcoming_birthdays.sort_by_key(|u| u.dob.unwrap().date().day());

    Ok(Message::new(
        "Upcoming birthdays retrieved successfully",
        Some(upcoming_birthdays),
    ))
}

pub async fn fetch_attendance_rates(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
) -> Result<Message<AttendanceStats>, ModuleError> {
    use crate::models::users::Role;
    use crate::schema::{user_attendance, users};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    // 1. Get total events count
    let total_events: i64 = user_attendance::table
        .select(count_distinct(user_attendance::date))
        .get_result(conn)
        .await?;

    // 2. Fetch all users with their roles and status
    let all_users: Vec<(uuid::Uuid, Role, bool)> = users::table
        .select((users::id, users::role, users::is_active))
        .load::<(uuid::Uuid, Role, bool)>(conn)
        .await?;

    let total_users_count = all_users.len() as i64;
    let active_users_list: Vec<&(uuid::Uuid, Role, bool)> =
        all_users.iter().filter(|(_, _, active)| *active).collect();
    let suspended_users_count = total_users_count - active_users_list.len() as i64;

    if total_events == 0 {
        return Ok(Message::new(
            "Attendance rates retrieved successfully (no events)",
            Some(AttendanceStats {
                admin_rate: 0.0,
                user_rate: 0.0,
                technical_rate: 0.0,
                total_users: total_users_count,
                active_users: active_users_list.len() as i64,
                suspended_users: suspended_users_count,
            }),
        ));
    }

    let admin_count = active_users_list
        .iter()
        .filter(|(_, r, _)| matches!(r, Role::Admin))
        .count() as f64;
    let user_count = active_users_list
        .iter()
        .filter(|(_, r, _)| matches!(r, Role::User))
        .count() as f64;
    let technical_count = active_users_list
        .iter()
        .filter(|(_, r, _)| matches!(r, Role::Technical))
        .count() as f64;

    // 3. Fetch all attendance records count grouped by role
    let attendances: Vec<(uuid::Uuid, Role)> = user_attendance::table
        .inner_join(users::table.on(user_attendance::user_id.eq(users::id)))
        .filter(users::is_active.eq(true))
        .select((user_attendance::id, users::role))
        .load::<(uuid::Uuid, Role)>(conn)
        .await?;

    let admin_attendances = attendances
        .iter()
        .filter(|(_, r)| matches!(r, Role::Admin))
        .count() as f64;
    let user_attendances = attendances
        .iter()
        .filter(|(_, r)| matches!(r, Role::User))
        .count() as f64;
    let technical_attendances = attendances
        .iter()
        .filter(|(_, r)| matches!(r, Role::Technical))
        .count() as f64;

    let calculate_rate = |attendances: f64, user_count: f64, total_events: i64| {
        if user_count == 0.0 {
            0.0
        } else {
            (attendances / (user_count * total_events as f64)) * 100.0
        }
    };

    let stats = AttendanceStats {
        admin_rate: calculate_rate(admin_attendances, admin_count, total_events),
        user_rate: calculate_rate(user_attendances, user_count, total_events),
        technical_rate: calculate_rate(technical_attendances, technical_count, total_events),
        total_users: total_users_count,
        active_users: active_users_list.len() as i64,
        suspended_users: suspended_users_count,
    };

    Ok(Message::new(
        "Attendance rates retrieved successfully",
        Some(stats),
    ))
}

pub async fn fetch_event_stats_report(
    conn: &mut impl AsyncConnection<Backend = diesel::pg::Pg>,
    event_id: uuid::Uuid,
) -> Result<Message<EventStatsReport>, ModuleError> {
    use crate::schema::{user_attendance, users};
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let attendees = user_attendance::table
        .inner_join(users::table.on(user_attendance::user_id.eq(users::id)))
        .filter(user_attendance::event_id.eq(event_id))
        .select((
            users::id,
            users::first_name,
            users::last_name,
            users::email,
            user_attendance::time_in,
        ))
        .load::<(uuid::Uuid, String, String, String, chrono::NaiveDateTime)>(conn)
        .await?;

    let event_attendees = attendees
        .into_iter()
        .map(|(id, first, last, email, time_in)| EventAttendee {
            user_id: id,
            first_name: first,
            last_name: last,
            email,
            time_in: time_in.time(),
        })
        .collect::<Vec<_>>();

    let total_attendees = event_attendees.len() as i64;

    let absentees = users::table
        .left_outer_join(
            user_attendance::table.on(user_attendance::user_id
                .eq(users::id)
                .and(user_attendance::event_id.eq(Some(event_id)))),
        )
        .filter(users::is_active.eq(true))
        .filter(user_attendance::id.is_null())
        .select(UserDto::as_select())
        .load::<UserDto>(conn)
        .await?;

    let eligible_attendees_count = total_attendees + absentees.len() as i64;

    Ok(Message::new(
        "Event stats report retrieved successfully",
        Some(EventStatsReport {
            total_attendees,
            eligible_attendees_count,
            attendees: event_attendees,
            absentees,
        }),
    ))
}
