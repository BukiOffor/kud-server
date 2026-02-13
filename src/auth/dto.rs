use super::*;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginPayload {
    pub user: String,
    pub password: String,
}
