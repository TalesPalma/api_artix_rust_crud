use crate::{models::client::Client, repository};

pub fn get_clients_service() -> Result<Vec<Client>, Box<dyn std::error::Error>> {
    let list_clients_from_server = repository::clients_repository::get_clients();
    Ok(list_clients_from_server)
}
