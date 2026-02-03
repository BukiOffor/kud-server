use super::*;
use crate::models::users::*;
use diesel::sql_types::{Bool, Int8, Nullable, Text, Timestamp, Uuid as SqlUuid};

// #[derive(Selectable, Serialize, Deserialize, Queryable)]
// #[diesel(table_name = user_attendance)]
// pub struct UserAttendanceDto {
//     pub id: uuid::Uuid,
//     pub user_id: uuid::Uuid,
//     pub date: NaiveDateTime,
//     pub time: NaiveDateTime,
//     pub location: Option<String>,
//     pub device_id: Option<String>,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }

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

// chida event centre = 9.070818996337124, 7.434377769114212

// doa = 9.076560214946829, 7.431590122491971

// house = 9.110356556451427, 7.380244362342951
