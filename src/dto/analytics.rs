use chrono::NaiveDate;

use crate::dto::user::UserDto;

use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserPresentStats {
    pub absentees: Vec<UserDto>,
    pub date: NaiveDate,
    pub presentees: Vec<UserDto>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AttendanceStats {
    pub admin_rate: f64,
    pub user_rate: f64,
    pub technical_rate: f64,
    pub total_users: i64
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AttendanceSummary {
    pub total_days: i64,
    pub days_present: i64,
    pub rate: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAttendanceHistory {
    pub user: UserDto,
    pub history: Vec<crate::dto::attendance::UserAttendanceDto>,
    pub summary: AttendanceSummary,
}
