use crate::models::roster::{Hall};

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
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub reg_no: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[diesel(column_name = password_hash)]
    pub password: String,
    pub dob: Option<NaiveDateTime>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub year_joined: String,
    pub current_roster_hall: Option<Hall>,
    pub current_roster_allocation: Option<String>,
    pub role: Role,
    pub last_seen: Option<NaiveDateTime>,
    pub device_id: Option<String>,
    pub is_active: bool,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub hall_derivation: i32,
}

impl User {
    pub fn set_reg_no(&mut self, code: i64) {
        self.reg_no = format!("{}/KUD/{:03}", self.year_joined, code);
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq, Eq, Hash, Default,
)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    #[default]
    User,
    Technical,
}

impl FromSql<Text, diesel::pg::Pg> for Role {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl ToSql<Text, diesel::pg::Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        out.write(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
