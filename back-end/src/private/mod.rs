use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct PrivateKeys {
    pub jwt_key : String
}

pub fn get_private_keys() -> Result<PrivateKeys, Box<dyn std::error::Error>> {
    let json_file_path = Path::new("./assets/private-keys.json");
    let file = File::open(json_file_path);

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let key_data : Result<PrivateKeys, serde_json::Error> = serde_json::from_reader(reader);

            match key_data {
                Ok(keys) => { 
                    return Ok(keys);
                 },
                Err(e) => { //Parse error
                    return Err(e.into());
                }
            }
        }
        Err(e) => { //File error
            return  Err(e.into());
        }
    }
}