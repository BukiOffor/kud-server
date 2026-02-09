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
}

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq, Eq, Hash)]
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

pub struct HallDerivation {
    pub main_hall: i32,
    pub gallery: i32,
    pub hall_one: i32,
    pub basement: i32,
    pub outside: i32,
}

impl HallDerivation {
    pub fn new() -> Self {
        Self {
            main_hall: 10,
            gallery: 8,
            hall_one: 6,
            basement: 4,
            outside: 2,
        }
    }

    pub fn get_derivation(&self, hall: Hall) -> i32 {
        match hall {
            Hall::MainHall => self.main_hall,
            Hall::Gallery => self.gallery,
            Hall::HallOne => self.hall_one,
            Hall::Basement => self.basement,
            Hall::Outside => self.outside,
        }
    }

    pub fn get_hall(&self, derivation: i32) -> Hall {
        match derivation {
            10 => Hall::MainHall,
            8 => Hall::Gallery,
            6 => Hall::HallOne,
            4 => Hall::Basement,
            2 => Hall::Outside,
            _ => panic!("Invalid derivation"),
        }
    }
    pub fn get_sum_of_derivation(&self) -> i32 {
        self.main_hall + self.gallery + self.hall_one + self.basement + self.outside
    }
}
