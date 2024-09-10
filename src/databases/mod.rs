use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Assuming `models` and `schema` are modules defined at the same level as this file.
pub mod models;

use crate::databases::models::{NewAccount, Account};  // Correcting the path if necessary
use crate::schema::account;  // This might need to be corrected based on your project structure

// Function to establish a connection to the PostgreSQL database.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Function to create a new account in the database.
pub fn create_account(
    conn: &mut PgConnection, 
    uid: i64, 
    mnemonic: Option<&str>, 
    address: Option<&str>, 
    token: Option<&str>, 
    feature: Option<&[u8]>) -> Account {

    let new_account = NewAccount { 
        uid, 
        mnemonic, 
        address, 
        token,
        feature  // Passing the binary data for the feature
    };

    diesel::insert_into(account::table)
        .values(&new_account)
        .get_result(conn)
        .expect("Error saving new account")
}
