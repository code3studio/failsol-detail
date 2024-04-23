use std::{collections::HashMap, path::{Path, PathBuf}, str::FromStr};

use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, Bytes, Data, Payload, Query},
    HttpResponse, Result,
};
use futures_util::StreamExt;
use mongodb::bson::doc;
use serde::Serialize;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{self, RpcTransactionConfig},
};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};

use crate::{
    model::signature_model::ImageQuery, services::db::Database, utils::generate_image::{generate::{self, generate}, upload_pinata::upload_pinata}
};

#[get("/signature")]
pub async fn get_signature(db: Data<Database>) -> HttpResponse {
    upload_pinata().await;
    if let Some(result) = db
        .signature
        .find_one_and_update(
            doc! { "status": false },
            doc! { "$set": { "status": true } },
            None,
        )
        .await
        .ok()
        .expect("get error")
    {
        
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
                        let s = &t.transaction.transaction;
                        let (signature, signer) = match s {
                            EncodedTransaction::Json(ui_transaction) => {
                                let signer = match &ui_transaction.message {
                                    UiMessage::Raw(ui_message) => &ui_message.account_keys[0],
                                    _ => {
                                        todo!()
                                    }
                                };
                                (ui_transaction.signatures[0].clone(), signer)
                            }
                            _ => {
                                todo!()
                            }
                        };
                        let fee = t.transaction.meta.clone().unwrap().fee;
                        let log_messages = match t.transaction.meta.clone().unwrap().log_messages {
                            solana_transaction_status::option_serializer::OptionSerializer::Some(e) => {e},
                            solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                            solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(), };
                        generate(&signature, t.slot, t.block_time, &signer, fee,log_messages).unwrap();
                        HttpResponse::Ok().json(t)
                    }
                    Err(e) => {
                        eprintln!("Error{:?}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching transaction: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::InternalServerError().body("getError".to_string())
    }
}

#[get("/image")]
pub async fn get_image( db:Data<Database>)->Result<NamedFile> {
    // let file_name = format!(".output/result.png",query.name );
    let path = PathBuf::from("./output/result.png");


    Ok(NamedFile::open(path)?)
}
