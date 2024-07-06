use crate::{models::client::Client, repository};

pub fn get_clients_service() -> Result<Vec<Client>, Box<dyn std::error::Error>> {
    let list_clients_from_server = repository::clients_repository::get_clients();
    Ok(list_clients_from_server)
}

pub fn get_clients_by_id_service(id: i32) -> Result<Client, Box<dyn std::error::Error>> {
    let list_clients_from_server = repository::clients_repository::get_clients();
    let client = list_clients_from_server.iter().find(|item| item.id == id);
    if let Some(item) = client {
        Ok(item.clone())
    } else {
        Err("Client not found".into())
    }
}

pub fn post_clients_service(new_client: Client) -> Result<Client, Box<dyn std::error::Error>> {
    repository::clients_repository::insert_into_bd(new_client.clone());
    Ok(new_client)
}
