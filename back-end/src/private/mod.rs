use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct PrivateKeys {
    pub jwt_key : String
}

pub async fn get_private_keys() -> PrivateKeys {
    let json_file_path = Path::new("./assets/private-keys.json");
    let file = File::open(json_file_path);

    let key_data = match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader::<BufReader<File>, PrivateKeys>(reader)            
        }
        Err(e) => { panic!("file opening failed: {}", e); }
    };

    match key_data {
        Ok(keys) => keys,
        Err(e) => { panic!("parse failed: {}", e); }
    }
}