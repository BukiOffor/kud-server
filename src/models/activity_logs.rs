use chrono::NaiveDateTime;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{self, Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq, Eq)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum ActivityType {
    UserLogin,
    UserLogout,
    UserCreated,
    UserUpdated,
    UserActivation,
    UserDeactivation,
    UserMarkedAttendance,
    AdminMarkedAttendanceForUser,
    UserImported,
    PasswordChanged,
    DeviceReset,
    EventCreated,
    EventUpdated,
    EventDeleted,
    EventCheckIn,
    RosterCreated,
    RosterUpdated,
    RosterDeleted,
    RosterActivated,
    AttendanceRevoked,
    RosterImported,
    UserHallUpdated,
}

impl ActivityType {
    pub fn message(&self) -> String {
        match self {
            ActivityType::UserLogin => "Logged into the system.".into(),
            ActivityType::UserLogout => "Logged out of the system.".into(),
            ActivityType::UserCreated => "Created a new user.".into(),
            ActivityType::UserUpdated => "Updated a user's information.".into(),
            ActivityType::UserActivation => "Activated a user account.".into(),
            ActivityType::UserDeactivation => "Deactivated a user account.".into(),
            ActivityType::UserMarkedAttendance => "Marked a user's attendance.".into(),
            ActivityType::AdminMarkedAttendanceForUser => "Marked a user's attendance.".into(),
            ActivityType::UserImported => "Imported users from CSV.".into(),
            ActivityType::PasswordChanged => "Changed user password.".into(),
            ActivityType::DeviceReset => "Changed user device ID.".into(),
            ActivityType::EventCreated => "Created a new event.".into(),
            ActivityType::EventUpdated => "Updated an event.".into(),
            ActivityType::EventDeleted => "Deleted an event.".into(),
            ActivityType::EventCheckIn => "Checked into an event.".into(),
            ActivityType::RosterCreated => "Created a new roster.".into(),
            ActivityType::RosterUpdated => "Updated a roster.".into(),
            ActivityType::RosterDeleted => "Deleted a roster.".into(),
            ActivityType::RosterActivated => "Activated a roster.".into(),
            ActivityType::AttendanceRevoked => "Revoked a user's attendance.".into(),
            ActivityType::RosterImported => "Imported a new roster.".into(),
            ActivityType::UserHallUpdated => "Updated a user's hall assignment.".into(),
        }
    }
}

impl FromSql<Text, diesel::pg::Pg> for ActivityType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        serde_json::from_str(s).map_err(Into::into)
    }
}

impl ToSql<Text, diesel::pg::Pg> for ActivityType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = serde_json::to_string(self)?;
        out.write(s.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize, Insertable, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::activity_logs)]
pub struct ActivityLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
    pub target_id: Option<Uuid>, // receiptent of the operation <topic id or subject id or user id>
    pub target_type: Option<String>, // change to enum ??
    pub details: Value,
    pub created_at: NaiveDateTime,
}

impl ActivityLog {
    pub fn new(activity_type: ActivityType, user_id: Uuid) -> Self {
        ActivityLog {
            id: Uuid::now_v7(),
            user_id,
            activity_type,
            target_id: None,
            target_type: None,
            details: Value::Null,
            created_at: chrono::Local::now().naive_local(),
        }
    }
    pub fn set_target_id(&mut self, target_id: Uuid) -> &mut Self {
        self.target_id = Some(target_id);
        self
    }
    pub fn set_target_type(&mut self, target_type: String) -> &mut Self {
        self.target_type = Some(target_type);
        self
    }
    pub fn set_details(&mut self, details: Value) -> &mut Self {
        self.details = details;
        self
    }
    /// resets the ID to a new UUID
    /// this is useful when you want to reuse the same log structure for a new log entry
    pub fn reset_id(&mut self) -> &mut Self {
        self.id = Uuid::now_v7();
        self
    }
    pub fn finish(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: Option<String>,
    pub user_role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub activity_type: String,
    pub created_at: NaiveDateTime,
}
