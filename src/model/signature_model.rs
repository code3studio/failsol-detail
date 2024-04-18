use serde::{Deserialize, Serialize,ser::{Serialize as SerSerialize,SerializeStruct,Serializer}};
#[derive(Debug,Deserialize,Serialize)]
pub struct Signature {
  pub  _id: String,
  pub  status:bool
}


#[derive(Debug,Deserialize,Serialize)]
pub struct ImageQuery {
 pub name:String
}