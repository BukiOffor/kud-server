pub mod attendance;
pub mod events;
pub mod user;
pub mod analytics;
use super::{schema::*, *};
//use diesel::sql_types::{Bool, Int8, Nullable, Text, Timestamp, Uuid as SqlUuid};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message<T> {
    pub message: String,
    pub data: Option<T>,
}

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
