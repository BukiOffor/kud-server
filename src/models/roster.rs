use super::*;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Identifiable,
    AsChangeset,
    Insertable,
    QueryableByName,
    utoipa::ToSchema,
)]
#[diesel(table_name = crate::schema::rosters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Roster {
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
    pub num_male_for_hall_one: Option<i32>,
    pub num_female_for_hall_one: Option<i32>,
    pub num_male_for_main_hall: Option<i32>,
    pub num_female_for_main_hall: Option<i32>,
    pub num_male_for_gallery: Option<i32>,
    pub num_female_for_gallery: Option<i32>,
    pub num_male_for_basement: Option<i32>,
    pub num_female_for_basement: Option<i32>,
    pub num_male_for_outside: Option<i32>,
    pub num_female_for_outside: Option<i32>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    AsExpression,
    FromSqlRow,
    PartialEq,
    Eq,
    Hash,
    utoipa::ToSchema,
)]
#[diesel(sql_type = Text)]
pub enum Hall {
    MainHall,
    HallOne,
    Gallery,
    Basement,
    Outside,
}
impl FromSql<Text, diesel::pg::Pg> for Hall {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl ToSql<Text, diesel::pg::Pg> for Hall {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        out.write(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl Hall {
    // Get all possible halls
    pub fn all() -> Vec<Hall> {
        vec![
            Hall::MainHall,
            Hall::HallOne,
            Hall::Gallery,
            Hall::Basement,
            Hall::Outside,
        ]
    }

    fn get_available_halls(served_halls: &[Hall], available: Vec<Hall>) -> Vec<Hall> {
        available
            .into_iter()
            .filter(|hall| !served_halls.contains(hall))
            .collect()
    }

    /// Assign a random hall from available ones
    pub fn assign_hall(served_halls: &[Hall], available: Vec<Hall>) -> Option<Hall> {
        use rand::prelude::IndexedRandom;
        let available = Self::get_available_halls(served_halls, available);
        if available.is_empty() {
            None // All halls have been served
        } else {
            #[allow(deprecated)]
            let mut rng = rand::thread_rng();
            available.choose(&mut rng).cloned()
        }
    }
}
