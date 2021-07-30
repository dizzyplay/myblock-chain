use sha256::{digest};
use std::ops::Add;

#[derive(Debug)]
pub struct Block {
    pub data: String,
    pub prev_hash: String,
    pub hash: String
}

impl Block {
    pub fn new(data:String, prev_hash: String)->Self{
        Block{
            data: data.clone(),
            prev_hash: prev_hash.clone(),
            hash: digest(data.add(prev_hash.as_str()))
        }
    }
}