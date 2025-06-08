// Refactor 4.0
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

const PRIVATE_KEY_LOCATION : &str = "./assets/private-keys.json";

#[derive(Debug, Deserialize, Clone)]
pub struct PrivateKeys {
    pub jwt_key : String
}

pub async fn get_private_keys() -> PrivateKeys {
    let json_file_path = Path::new(PRIVATE_KEY_LOCATION);
    let file = File::open(json_file_path);

    let key_data = match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader::<BufReader<File>, PrivateKeys>(reader)            
        }
        Err(error) => { panic!("file opening failed: {}", error); }
    };

    match key_data {
        Ok(keys) => keys,
        Err(error) => { panic!("parse failed: {}", error); }
    }
}