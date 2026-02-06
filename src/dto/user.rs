use super::*;
use crate::models::users::*;

#[derive(Selectable, Serialize, Deserialize, Queryable, Clone)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    #[diesel(column_name = password_hash)]
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub dob: Option<NaiveDateTime>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_seen: Option<NaiveDateTime>,
    pub year_joined: String,
    pub reg_no: String,
    pub current_roster_hall: Option<String>,
    pub current_roster_allocation: Option<String>,
    pub role: Role,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub dob: Option<NaiveDateTime>,
    pub year_joined: String,
    pub is_active: bool,
    pub role: Role,
    pub gender: Option<String>,
    pub phone: Option<String>,
}

impl TryFrom<NewUser> for User {
    type Error = ModuleError;
    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let password_hash = crate::helpers::password_hasher(&value.password)?;
        Ok(User {
            id: Uuid::now_v7(),
            reg_no: "N/A".to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            password: password_hash,
            dob: value.dob,
            avatar_url: None,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
            year_joined: value.year_joined,
            current_roster_hall: None,
            current_roster_allocation: None,
            last_seen: Some(chrono::Local::now().naive_local()),
            is_active: value.is_active,
            role: value.role,
            device_id: None,
            username: None,
            gender: value.gender,
            phone: value.phone,
            address: None,
            city: None,
            state: None,
            country: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    pub id: Option<uuid::Uuid>,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<NaiveDateTime>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserFilter {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangePasswordRequest {
    pub email: String,
    pub password: String,
}
