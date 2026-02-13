use chrono::{NaiveDate, NaiveTime};

use crate::dto::user::UserDto;

use super::*;

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserPresentStats {
    pub absentees: Vec<UserDto>,
    pub date: NaiveDate,
    pub presentees: Vec<UserDto>,
}

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AttendanceStats {
    pub admin_rate: f64,
    pub user_rate: f64,
    pub technical_rate: f64,
    pub total_users: i64,
    pub active_users: i64,
    pub suspended_users: i64,
}

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AttendanceSummary {
    pub total_days: i64,
    pub days_present: i64,
    pub rate: f64,
}

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserAttendanceHistory {
    pub user: UserDto,
    pub history: Vec<crate::dto::attendance::UserAttendanceDto>,
    pub summary: AttendanceSummary,
}

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EventAttendee {
    pub user_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub time_in: NaiveTime,
}

#[derive(Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EventStatsReport {
    pub total_attendees: i64,
    pub eligible_attendees_count: i64,
    pub attendees: Vec<EventAttendee>,
    pub absentees: Vec<UserDto>,
}
