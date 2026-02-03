pub mod attendance;
pub mod user;
use super::{schema::*, *};
use diesel::sql_types::{Bool, Int8, Nullable, Text, Timestamp, Uuid as SqlUuid};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Message {
            message: message.into(),
        }
    }
}

impl From<&str> for Message {
    fn from(message: &str) -> Self {
        Message::new(message)
    }
}
