use serde::{Deserialize, Serialize,ser::{Serialize as SerSerialize,SerializeStruct,Serializer}};
#[derive(Debug,Deserialize,Serialize)]
pub struct Signature {
  pub  _id: String,
  pub  status:bool
}



pub struct Transaction {
  pub signatures:Vec<String>,
}

impl SerSerialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("Transaction", 1)?;
        s.serialize_field("signatures", &self.signatures)?;
        s.end()
    }
}