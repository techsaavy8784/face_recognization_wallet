use diesel::prelude::*;
use crate::schema::account;
use diesel::sql_types::Bytea; // Include Bytea type for handling binary data

#[derive(Clone, Debug, Queryable, Selectable)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i64,
    pub uid: i64,
    pub mnemonic: Option<String>,
    pub address: Option<String>,
    pub token: Option<String>,
    pub feature: Option<Vec<u8>>,  // Include the feature field for binary data
}

#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub uid: i64,
    pub mnemonic: Option<&'a str>,
    pub address: Option<&'a str>,
    pub token: Option<&'a str>,
    pub feature: Option<&'a [u8]>  // Include the feature field to be able to insert binary data
}
