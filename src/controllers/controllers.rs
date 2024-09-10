use actix_web::{web, HttpResponse, Responder};
use cess_rust_sdk::core::utils::account::get_pair_address_as_ss58_address;
use serde::{Deserialize, Serialize};
use sp_keyring::sr25519::sr25519::Pair;
use diesel::prelude::*;

use crate::{
    controllers::accounts::{generate_mnemonic, get_pair},
    databases::*,
    databases::models::Account,
    schema::account::dsl::*,
    jwt::generate_token
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetWalletInfo {
    uid: i64,
    address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateWalletInfo {
    uid: i64,
    feature: Vec<u8>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RecoverWalletInfo {
    uid: i64,
    feature: Vec<u8>,
    recover_key: String
}

#[derive(Serialize, Debug)]
pub struct WalletResponse {
    result: String,
    msg: String,
    wallet_address: String,
    mnemonic: String,
    token: String,
    feature: Vec<u8>
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the face-recognization rust server!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Status: Running")
}

pub async fn get_wallet_post(info: web::Json<GetWalletInfo>) -> impl Responder {
    let connection = &mut establish_connection();

    let results = account
        .filter(address.eq(&info.address)) // Ensure address is referenced correctly
        .limit(1)
        .load::<Account>(connection)
        .expect("Error loading account");

    if results.is_empty() {
        let response_message = WalletResponse {
            result: "Error".to_string(),
            msg: "Can not find the account".to_string(),
            wallet_address: "".to_string(),
            mnemonic: "".to_string(),
            token: "".to_string(),
            feature: Vec::new()
        };
        return HttpResponse::Ok().content_type("application/json").json(response_message);
    }

    let account_data = &results[0];
    match generate_token(info.address.clone(), info.uid) {
        Ok(jtoken) => {
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Got wallet successfully".to_string(),
                wallet_address: info.address.clone(),
                mnemonic: account_data.mnemonic.clone().unwrap_or_default(),
                token: jtoken,
                feature: account_data.feature.clone().unwrap_or_else(Vec::new),
            };
            HttpResponse::Ok().json(response_message)
        },
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `generate_token`".to_string(),
                wallet_address: "".to_string(),
                mnemonic: "".to_string(),
                token: "".to_string(),
                feature: Vec::new()
            };
            HttpResponse::Ok().json(response_message)
        }
    }
}

pub async fn create_wallet_post(info: web::Json<CreateWalletInfo>) -> impl Responder {
    let mnem: Option<String>;
    println!("======================  create wallet 1 ");
    match generate_mnemonic() {
        Ok(t) => mnem = Some(t),
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `generate_mnemonic`".to_string(),
                wallet_address: "".to_string(),
                mnemonic: "".to_string(),
                token: "".to_string(),
                feature: Vec::new()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    };
    println!("======================  create wallet 2 ");
    let pair: Pair;
    match get_pair(&mnem.clone().unwrap(), None) {
        Ok(t) => pair = t,
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `get_pair`".to_string(),
                wallet_address: "".to_string(),
                mnemonic: "".to_string(),
                token: "".to_string(),
                feature: Vec::new()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    };
    println!("======================  create wallet 3 ");
    let address_to_fund: String;
    match get_pair_address_as_ss58_address(pair) {
        Ok(t) => address_to_fund = t,
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `get_pair_address_as_ss58_address`".to_string(),
                wallet_address: "".to_string(),
                mnemonic: "".to_string(),
                token: "".to_string(),
                feature: Vec::new()
            };
            return HttpResponse::Ok().content_type("application/json").json(response_message);
        }
    }
    println!("======================  create wallet 4 ");
    match generate_token(address_to_fund.clone(), info.uid.clone()) {
        Ok(jtoken) => {
            let connection = &mut establish_connection();

            let myaccount = create_account(connection, info.uid.clone(), Some(&mnem.unwrap()), Some(&address_to_fund.clone()), Some(&jtoken.clone()), Some(&info.feature));
            
            println!("test account: {:?}", myaccount.clone());
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Created wallet successfully".to_string(),
                wallet_address: address_to_fund,
                mnemonic: "".to_string(),
                token: jtoken,
                feature: Vec::new()
            };
        
            println!("test response_message: {:?}", response_message);
            HttpResponse::Ok().json(response_message)
        },
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `generate_token`".to_string(),
                wallet_address: "".to_string(),
                mnemonic: "".to_string(),
                token: "".to_string(),
                feature: Vec::new()
            };
        
            HttpResponse::Ok().json(response_message)
        }
    }
}


pub async fn recover_wallet_post(info: web::Json<RecoverWalletInfo>) -> impl Responder {
    let connection = &mut establish_connection();

    let results = account
        .filter(address.eq(&info.recover_key)) // Ensure address is referenced correctly
        .limit(1)
        .load::<Account>(connection)
        .expect("Error loading account");

    if results.is_empty() {
        let response_message = WalletResponse {
            result: "Error".to_string(),
            msg: "Can not find the account".to_string(),
            wallet_address: "".to_string(),
            mnemonic: "".to_string(),
            token: "".to_string(),
            feature: Vec::new()
        };
        return HttpResponse::Ok().content_type("application/json").json(response_message);
    }

    let account_data = &results[0];
    match generate_token(info.recover_key.clone(), info.uid) {
        Ok(jtoken) => {
            let response_message = WalletResponse {
                result: "Success".to_string(),
                msg: "Got wallet successfully".to_string(),
                wallet_address: account_data.address.clone().unwrap_or_default(),
                mnemonic: "".to_string(),
                token: jtoken,
                feature: account_data.feature.clone().unwrap_or_else(Vec::new),
            };
            HttpResponse::Ok().json(response_message)
        },
        Err(_) => {
            let response_message = WalletResponse {
                result: "Error".to_string(),
                msg: "Internal error on `generate_token`".to_string(),
                wallet_address: "".to_string(),
                token: "".to_string(),
                mnemonic: "".to_string(),
                feature: Vec::new()
            };
            HttpResponse::Ok().json(response_message)
        }
    }
}