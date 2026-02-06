use super::*;
use crate::models::user_attendance::AttendanceType;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Selectable, Serialize, Deserialize, Queryable, Clone)]
#[diesel(table_name = user_attendance)]
pub struct UserAttendanceDto {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub date: NaiveDate,
    pub week_day: String,
    pub time_in: NaiveDateTime,
    pub time_out: Option<NaiveDateTime>,
    pub marked_by: Option<uuid::Uuid>,
    pub event_id: Option<uuid::Uuid>,
    pub attendance_type: AttendanceType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoPoint {
    pub lat: f64,
    pub lng: f64,
}

impl GeoPoint {
    pub fn validate(self) -> Result<Self, &'static str> {
        if !(-90.0..=90.0).contains(&self.lat) {
            return Err("Invalid latitude");
        }
        if !(-180.0..=180.0).contains(&self.lng) {
            return Err("Invalid longitude");
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignAttendanceRequest {
    pub location: GeoPoint,
    pub device_id: String,
}
