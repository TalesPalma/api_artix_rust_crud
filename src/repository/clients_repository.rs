use diesel::{RunQueryDsl, SelectableHelper};

use crate::{
    database,
    models::client::{self, Client},
    schema::clients,
};
pub fn get_clients() -> Vec<Client> {
    let mut conn = database::configs::get_connection_mylsql();
    let results = clients::table
        .load::<Client>(&mut conn)
        .expect("Error loading clients");
    results
}

pub fn insert_into_bd(new_client: Client) {
    let mut conn = database::configs::get_connection_mylsql();
    diesel::insert_into(clients::table)
        .values(&new_client)
        .execute(&mut conn)
        .expect("Error insert in bd");
}
