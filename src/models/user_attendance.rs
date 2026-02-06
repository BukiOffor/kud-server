use super::*;
use chrono::NaiveDate;

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
#[diesel(table_name = crate::schema::user_attendance)]
pub struct UserAttendance {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub week_day: String,
    pub time_in: NaiveDateTime,
    pub time_out: Option<NaiveDateTime>,
    pub event_id: Option<Uuid>,
    pub marked_by: Option<Uuid>,
    pub attendance_type: AttendanceType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, AsExpression, Default, FromSqlRow, PartialEq, Eq,
)]
#[diesel(sql_type = Text)]
pub enum AttendanceType {
    Remote,
    #[default]
    Onsite,
    Mandatory,
    Optional,
    Standard,
    Late,
    Excused,
}

impl FromSql<Text, diesel::pg::Pg> for AttendanceType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl ToSql<Text, diesel::pg::Pg> for AttendanceType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        out.write(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl UserAttendance {
    pub fn new(user_id: Uuid, date: NaiveDate) -> Self {
        use chrono::Datelike;
        Self {
            id: Uuid::now_v7(),
            user_id,
            date,
            week_day: date.weekday().to_string(),
            time_in: chrono::Local::now().naive_local(),
            time_out: None,
            event_id: None,
            marked_by: None,
            attendance_type: AttendanceType::Onsite,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn sign_out(&mut self) {
        self.time_out = Some(chrono::Local::now().naive_local());
    }

    pub fn set_marked_by(&mut self, marked_by: Uuid) {
        self.marked_by = Some(marked_by);
    }

    pub fn set_event_id(&mut self, event_id: Uuid) {
        self.event_id = Some(event_id);
    }

    pub fn set_attendance_type(&mut self, attendance_type: AttendanceType) {
        self.attendance_type = attendance_type;
    }
}
