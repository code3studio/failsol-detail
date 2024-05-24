use std::error::Error;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature as SolSignature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub _id: String,
    pub status: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct History {
    pub _id: ObjectId,
    pub status: bool,
    pub signature:String
}

impl TryFrom<SolSignature> for History {
    type Error = Box<dyn Error>;

    fn try_from(value: SolSignature) -> Result<Self, Self::Error> {
        Ok(Self {
            _id:ObjectId::new(),
            status:false,
            signature:value.to_string()
        })
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryRequest {
    pub signature:String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageQuery {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseModel {
    // pub transaction: EncodedConfirmedTransactionWithStatusMeta,
    pub hash: String,
    pub number:u64
}




