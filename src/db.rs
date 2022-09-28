use diesel::{PgConnection, Connection};
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
