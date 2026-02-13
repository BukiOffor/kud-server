use crate::models::roster::{Hall, Roster};

use super::*;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct NewRoster {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub num_for_hall_one: i32,
    pub num_for_main_hall: i32,
    pub num_for_gallery: i32,
    pub num_for_basement: i32,
    pub num_for_outside: i32,
    pub year: String,
}
impl From<NewRoster> for Roster {
    fn from(roster: NewRoster) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: roster.name,
            is_active: false,
            num_for_hall_one: roster.num_for_hall_one,
            num_for_main_hall: roster.num_for_main_hall,
            num_for_gallery: roster.num_for_gallery,
            num_for_basement: roster.num_for_basement,
            num_for_outside: roster.num_for_outside,
            start_date: roster.start_date,
            end_date: roster.end_date,
            year: roster.year,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::rosters)]
pub struct RosterDto {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub start_date: chrono::NaiveDate,
    pub num_for_hall_one: i32,
    pub num_for_main_hall: i32,
    pub num_for_gallery: i32,
    pub num_for_basement: i32,
    pub num_for_outside: i32,
    pub end_date: chrono::NaiveDate,
    pub year: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, utoipa::ToSchema)]
#[diesel(table_name = crate::schema::rosters)]
pub struct UpdateRosterRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub num_for_hall_one: Option<i32>,
    pub num_for_main_hall: Option<i32>,
    pub num_for_gallery: Option<i32>,
    pub num_for_basement: Option<i32>,
    pub num_for_outside: Option<i32>,
    pub year: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, utoipa::ToSchema)]
pub struct RosterAssignmentDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub reg_no: String,
    pub hall: Hall,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CsvUserRoster {
    pub first_name: String,
    pub last_name: String,
    pub hall: Option<Hall>,
}
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateUserHallRequest {
    pub user_id: Uuid,
    pub user_roster_id: Uuid,
    pub hall: Hall,
}
