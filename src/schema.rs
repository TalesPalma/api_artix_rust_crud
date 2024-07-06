diesel::table! {
    clients (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        email -> Varchar,
    }
}
