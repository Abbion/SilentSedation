// Refactor 4.0
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::private::PrivateKeys;

pub struct JsonWebTokenData {
    encode_key : EncodingKey,
    decode_key : DecodingKey,
    header : Header,
    validator : Validation
}

impl JsonWebTokenData {
    pub fn new(private_keys : &PrivateKeys) -> JsonWebTokenData {
        JsonWebTokenData {
            encode_key : EncodingKey::from_secret(private_keys.jwt_key.as_ref()),
            decode_key : DecodingKey::from_secret(private_keys.jwt_key.as_ref()),
            header : Header::new(Algorithm::HS512),
            validator : Validation::new(Algorithm::HS512)
        }
    }

    pub fn encode<T: Serialize>(&self, data : &T) -> jsonwebtoken::errors::Result<String> {
         jsonwebtoken::encode(&self.header, data, &self.encode_key)
     }

     pub fn decode<T: DeserializeOwned>(&self, token : String) -> jsonwebtoken::errors::Result<TokenData<T>> {
        jsonwebtoken::decode::<T>(&token, &self.decode_key, &self.validator)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    pub sub : String,
    pub id : usize,
    exp : usize
}

impl UserToken {
    pub fn new(sub : String, id : usize) -> UserToken {
        UserToken { sub, id, exp : 0 }
    }

    pub fn set_exp_in_days(&mut self, days_till_expire : usize) {
        let start = SystemTime::now().duration_since(UNIX_EPOCH);

        match start {
            Ok(duration) => {
                let seconds_in_days = days_till_expire * 24 * 60 * 60;
                self.exp = duration.as_secs() as usize + seconds_in_days;
            },
            Err(error) => {
                eprintln!("{}", error);
                self.exp = 0;
            }
        }
    }
}