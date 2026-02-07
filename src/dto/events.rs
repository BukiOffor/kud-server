use chrono::{NaiveDate, NaiveTime};

use crate::models::user_attendance::AttendanceType;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub location: models::events::Location,
    pub attendance_type: AttendanceType,
    pub grace_period_in_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInWithIdentifierRequest {
    pub event_id: Uuid,
    pub identifier: String,
    pub attendance_type: AttendanceType,
    pub location: Option<GeoPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckIntoEventRequest {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub attendance_type: AttendanceType,
    pub location: Option<GeoPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEventRequest {
    pub event_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub location: Option<models::events::Location>,
    pub attendance_type: Option<AttendanceType>,
    pub grace_period_in_minutes: Option<i32>,
}
