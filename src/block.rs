use std::fmt::Formatter;
use sha256::digest;
use std::ops::Add;
use std::fmt;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Block {
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data);
        Ok(())
    }
}

impl Block {
    pub fn new(data: String, prev_hash: String) -> Self {
        Block {
            data: data.clone(),
            prev_hash: prev_hash.clone(),
            hash: digest(data.add(prev_hash.as_str())),
        }
    }
}
