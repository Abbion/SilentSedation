// Refactor 4.0
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use crate::constants::DEVICE_CODE_LENGTH;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Code {
    code : [u8; DEVICE_CODE_LENGTH]
}

impl Code {
    pub fn new() -> Code {
        let mut rng = rand::rng();
        let mut code_digits : [u8; DEVICE_CODE_LENGTH] = [0; DEVICE_CODE_LENGTH];
        for i in 0..DEVICE_CODE_LENGTH {
          code_digits[i] = rng.random_range(0..10);  
        }

        Code{ code : code_digits }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.code.shuffle(&mut rng);
    }

    pub fn as_string(&self) -> String {
        self.code.iter().map(|d| d.to_string()).collect()
    }

    pub fn from_string(string : String) -> Option<Code> {
        if string.len() != DEVICE_CODE_LENGTH || !string.chars().all(|s| s.is_ascii_digit()) {
            return None;
        }

        let mut code_digits : [u8; DEVICE_CODE_LENGTH] = [0; DEVICE_CODE_LENGTH];
        for (i, char) in string.chars().enumerate() {
            let digit = char.to_digit(10);
            match digit {
                Some(digit) => code_digits[i] = digit as u8,
                None => { 
                    eprintln!("Error: Code.from_string failed!");
                    return None;
                }
            }
        }
        Some(Code { code : code_digits })
    }
}