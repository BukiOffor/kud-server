pub mod analytics;
pub mod attendance;
pub mod events;
pub mod logs;
pub mod pagination;
pub mod roster;
pub mod user;
use super::{schema::*, *};
//use diesel::sql_types::{Bool, Int8, Nullable, Text, Timestamp, Uuid as SqlUuid};

#[derive(Debug, Clone, Serialize, Deserialize, Default, utoipa::ToSchema)]
pub struct Message<T> {
    pub message: String,
    pub data: Option<T>,
}

pub type MessageEmpty = Message<()>;
pub type MessageString = Message<String>;
pub type MessageAttendanceVec = Message<Vec<crate::dto::attendance::AttendanceWithUser>>;
pub type MessageUserDtoVec = Message<Vec<crate::dto::user::UserDto>>;
pub type MessageUserPresentStats = Message<crate::dto::analytics::UserPresentStats>;
pub type MessageAttendanceStats = Message<crate::dto::analytics::AttendanceStats>;
pub type MessageUserAttendanceHistory = Message<crate::dto::analytics::UserAttendanceHistory>;
pub type MessageEventStatsReport = Message<crate::dto::analytics::EventStatsReport>;
pub type MessageRosterDto = Message<crate::dto::roster::RosterDto>;
pub type MessageRosterAssignmentDtoVec = Message<Vec<crate::dto::roster::RosterAssignmentDto>>;

impl<T> Message<T> {
    pub fn new<S: Into<String>>(message: S, data: Option<T>) -> Self {
        Message {
            message: message.into(),
            data,
        }
    }
}

impl<T> From<&str> for Message<T> {
    fn from(message: &str) -> Self {
        Message::new(message, None)
    }
}
