use std::env;

use pinata_sdk::{PinByFile, PinByJson, PinataApi};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attributes {
    attributes: Vec<Attribute>,
}

pub async fn upload_pinata(transaction:String,block:String,fee:f64) ->Result<String,String> {
    let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
    let pinata_secret_key = env::var("PINATA_API_SECRET_KEY").unwrap();
    let nft_name = env::var("NFT_NAME").unwrap_or("FailSol".to_string());
    let description = env::var("DESCRIPTION").unwrap_or("This is an NFT on Solana".to_string());
    let api = PinataApi::new(&pinata_api_key, &pinata_secret_key).unwrap();

    // test that you can connect to the API:
    let result = api.pin_file(PinByFile::new("output/result.png")).await;
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        // let mut json_data = HashMap::new();
        // let data = r#"
        // {
        //     "attributes": [
        //         {"trait_type": "Speed", "value": "Quick"},
        //         {"trait_type": "Type", "value": "Pixelated"},
        //         {"trait_type": "Background", "value": "QuickNode Blue"}
        //     ]
        // }
        // "#;
        // let parsed_attributes: Value = serde_json::from_str(data).unwrap();
   
        // json_data.insert("name", nft_name);
        // json_data.insert("description", description);
        // json_data.insert("image", format!("https://white-giant-bird-563.mypinata.cloud/ipfs/{}",hash));
        // json_data.insert("attributes", data.to_string());

        let json = json!({
            "name":nft_name,
            "description": description,
            "image": format!("https://white-giant-bird-563.mypinata.cloud/ipfs/{}",hash),
            "attributes" : [
                {"trait_type" : "link" ,"value":transaction},
                {"trait_type" : "block" ,"value":block},
                {"trait_type" : "fee" ,"value":fee},
            ] ,
            "properties" : {
                "files" : [
                    {
                        "type":"image/png",
                        "uri" :format!("https://white-giant-bird-563.mypinata.cloud/ipfs/{}",hash)
                    }
                ]
            }
        });
        let result = api.pin_json(PinByJson::new(json)).await;

        if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        return Ok(hash);
        }
        
      }
      Err("upload failed".to_string())
      
}
