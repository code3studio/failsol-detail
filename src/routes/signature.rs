use std::str::FromStr;

use actix_web::{get, web::{self, Data}, HttpResponse};
use mongodb::bson::doc;
use solana_client::{rpc_client::RpcClient, rpc_config::{self, RpcTransactionConfig}};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;

use crate::services::db::Database;

#[get("/signature")]
pub async fn get_signature(db:Data<Database>) -> HttpResponse {
    if let Some(result) = db.signature.find_one_and_update(doc! { "status": false }, doc! { "$set": { "status": true } },None).await.ok().expect("get error") {
        // let client = RpcClient::new("https://api.mainnet-beta.solana.com");
        // let signature = Signature::from_str(&result._id).unwrap();
        // match client.get_transaction(&signature, UiTransactionEncoding::Json) {
        //     Ok(transaction) => println!("{:#?}", transaction),
        // Err(e) => eprintln!("Error fetching transaction: {}", e),
        // }
        
        //  HttpResponse::Ok().json(result)
        let fetched_transaction = web::block(move || {
            let client = RpcClient::new("https://api.mainnet-beta.solana.com");
            let signature = Signature::from_str(&result._id.clone()).unwrap();
            let config = RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            };
            client.get_transaction_with_config(&signature, config)
        })
        .await;

        match fetched_transaction {
            Ok(transaction) => {
                match transaction {
                    Ok(t) => {
                    HttpResponse::Ok().json(t)}
                    Err(e) => {eprintln!("Error{:?}",e);
                    HttpResponse::InternalServerError().finish()}
                }
                
            }
            Err(e) => {
                eprintln!("Error fetching transaction: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }else {
        HttpResponse::InternalServerError().body("getError".to_string())
    }
}