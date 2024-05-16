use serde::{Deserialize, Serialize};
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
#[derive(Debug,Deserialize,Serialize)]
pub struct Signature {
  pub  _id: String,
  pub  status:bool
}


#[derive(Debug,Deserialize,Serialize)]
pub struct ImageQuery {
 pub name:String
}

#[derive(Debug,Deserialize,Serialize)]

pub struct ResponseModel {
  pub transaction:EncodedConfirmedTransactionWithStatusMeta,
  pub hash:String
}