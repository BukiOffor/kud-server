use super::*;
use crate::models::roster::Hall;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    AsChangeset,
    Insertable,
    QueryableByName,
)]
#[diesel(table_name = crate::schema::users_rosters)]
pub struct UsersRoster {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub roster_id: uuid::Uuid,
    pub hall: Hall,
    pub year: String,
    pub created_at: chrono::NaiveDateTime,
}

impl UsersRoster {
    pub fn new(user_id: uuid::Uuid, roster_id: uuid::Uuid, hall: Hall, year: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            user_id,
            roster_id,
            hall,
            year,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
