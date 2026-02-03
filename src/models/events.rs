use crate::models::user_attendance::AttendanceType;

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
#[diesel(table_name = crate::schema::events)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub date: NaiveDateTime,
    pub time: NaiveDateTime,
    pub grace_period_in_minutes: i32,
    pub attendance_type: AttendanceType,
    pub location: Location,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq, Eq, Hash, Default,
)]
#[diesel(sql_type = Text)]
pub enum Location {
    #[default]
    DOA,
    CHIDA,
    OTHER,
}

impl FromSql<Text, diesel::pg::Pg> for Location {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl ToSql<Text, diesel::pg::Pg> for Location {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        out.write(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
