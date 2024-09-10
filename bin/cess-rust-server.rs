use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;
use cess_rust_server::routes::configure;
use cess_rust_sdk::chain::{ChainSdk, file::File};
use cess_rust_sdk::chain::storage_handler::StorageHandler;
use cess_rust_sdk::config;
use cess_rust_sdk::chain::deoss::DeOss;
use cess_rust_sdk::core::utils::account::parsing_public_key;
use cess_rust_sdk::chain::file_bank::FileBank;

#[tokio::main()]
async fn main() -> std::io::Result<()> {
    // load environment variables
    dotenv().ok();

    // initialize SDK
    let cess_self_node: bool = env::var("CESS_SELF")
        .map(|value| value == "true")  
        .unwrap_or(false);
    let cess_node_url: String = env::var("CESS_NODE_URL")
        .unwrap_or_else(|_| "ws://54.153.126.127:8080".to_string());
        // .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string());
    let deoss_url: String = env::var("DEOSS_URL")
        .unwrap_or_else(|_| "default_deoss_url".to_string());
    let deoss_account: String = env::var("DEOSS_ACCOUNT")
        .unwrap_or_else(|_| "default_deoss_account".to_string());
    
    if cess_self_node == true {
        config::set_custom_url(Some(cess_node_url.clone()));
    }
    
    const MNEMONIC: &str = "chicken sport cereal awake alarm degree love trophy since broom frozen minor";
    const ACCOUNT_ADDRESS: &str = "cXhcFzLAS7LBuyUaPvpJd4Honkod3SFE7Fvx1HxSa3tHjHWq2";
    const BUCKET_NAME: &str = "Face_Bucket_5";
    
    let sdk = ChainSdk::new(&MNEMONIC, "service_name");
    let deoss_url = "https://deoss-us.cess.cloud".to_string();
    
    config::set_custom_deoss_url(Some("https://deoss-us.cess.cloud".to_string()));
    config::set_custom_deoss_account(Some("cXf2xaU1RiJUhpPc471PxWknFx3mqf5opV6VTEQ4oYohWLZib".to_string()));
    
    println!(" ================ ENV VARIABLES FOR CESS ============== \n");
    println!("\t CESS_SELF: {}", cess_self_node);
    println!("\t CESS_NODE_URL: {}", cess_node_url);
    // println!("\t DEOSS_URL: {}", deoss_url);
    // println!("\t DEOSS_ACCOUNT: {}", deoss_account);
    println!("\t MNEMONIC: {}", MNEMONIC);
    println!("\n");

    // println!(" ================ BUY CESS STORAGE SPACE ============== \n");
    
    // let buy_space_result = sdk.buy_space(1).await;
    // match buy_space_result {
    //     Ok(r) => {
    //         println!("Storage Purchase successful: {:?}", r);
    //     }
    //     Err(e) => {
    //         println!("Storage Purchase failed: {:?}", e);
    //     }
    // }
    // println!("\n");

    // println!(" ================ AUTHORIZE CESS STORAGE SPACE ============== \n");
    // // if file name is not specified, the root_hash will be used as the file_name
    // let authorize_pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
    // let authorize_result = sdk.authorize(&authorize_pk_bytes).await;
    // match authorize_result {
    //     Ok(r) => {
    //         println!("Account authorize successful: {:?}", r);
    //     }
    //     Err(e) => {
    //         println!("Account authorize failed: {:?}", e);
    //     }
    // }
    // println!("\n");

    // println!(" ================ BUCKET CREATION ============== \n");
    // let create_pk_bytes = parsing_public_key(ACCOUNT_ADDRESS).unwrap();
    // let create_result = sdk.create_bucket(&create_pk_bytes, BUCKET_NAME).await;
    // match create_result {
    //     Ok(r) => {
    //         println!("Account authorize successful: {:?}", r);
    //     }
    //     Err(e) => {
    //         println!("Account authorize failed: {:?}", e);
    //     }
    // }
    // println!("\n");

    // println!(" ================ FILE UPLOAD ============== \n");
    // let upload_result = sdk.store_file("diesel.toml", BUCKET_NAME).await;
    // match upload_result {
    //     Ok(r) => {
    //         println!("File upload successful: {:?}", r);
    //     }
    //     Err(e) => {
    //         println!("File upload failed: {:?}", e);
    //     }
    // }
    // println!("\n");

    let explorer_server_port: String = env::var("EXPLORER_SERVER_PORT")
        .unwrap_or_else(|_| "8799".to_string());
    let explorer_server_host: String = env::var("EXPLORER_SERVER_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    println!("Welcome Face Wallet!");
    let _ = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .configure(configure)
    })
    .bind(format!("{explorer_server_host}:{explorer_server_port}"))? // Bind server to localhost:8080
    .run()
    .await;

    Ok(())
}