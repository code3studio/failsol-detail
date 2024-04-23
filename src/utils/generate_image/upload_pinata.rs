use std::{collections::HashMap, env};

use pinata_sdk::{PinByFile, PinByJson, PinataApi};

pub async fn upload_pinata() {
    let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
    let pinata_secret_key = env::var("PINATA_API_SECRET_KEY").unwrap();
    let api = PinataApi::new(&pinata_api_key, &pinata_secret_key).unwrap();

    // test that you can connect to the API:
    let result = api.pin_file(PinByFile::new("output/result.png")).await;
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        println!("hash={}",hash);
        let mut json_data = HashMap::new();
        json_data.insert("url", hash);
        let result = api.pin_json(PinByJson::new(json_data)).await;

        if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        println!("jsonhash=={}",hash)
        }
        
      }
}
