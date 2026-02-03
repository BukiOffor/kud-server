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
    pub location: String,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
