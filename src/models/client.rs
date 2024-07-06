use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name= crate::schema::clients)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
}
