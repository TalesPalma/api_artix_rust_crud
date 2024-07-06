use crate::models::client::Client;

pub fn get_clients() -> Vec<Client> {
    vec![
        Client {
            id: 1,
            name: "Tales".to_string(),
            email: "HkxhB@example.com".to_string(),
        },
        Client {
            id: 2,
            name: "Carlos".to_string(),
            email: "HkxhB@example.com".to_string(),
        },
        Client {
            id: 3,
            name: "Maria".to_string(),
            email: "HkxhB@example.com".to_string(),
        },
        Client {
            id: 4,
            name: "Maria".to_string(),
            email: "HkxhB@example.com".to_string(),
        },
    ]
}
