use crate::DB;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Formatter;
use warp::{self, Filter};
use serde::{Serialize, Deserialize};
use sha256::digest;
use std::ops::Add;


pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.blocks.iter() {
            write!(
                f,
                "data: {} hash: {} prev_hash: {} ",
                b.data, b.hash, b.prev_hash
            )?;
        }
        Ok(())
    }
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, data: String) {
        match self.blocks.last() {
            Some(block) => {
                let last_hash = block.hash.clone();
                let new_block = Block::new(data, last_hash);
                self.blocks.push(new_block);
            }
            None => {
                let new_block = Block::new(data, String::new());
                self.blocks.push(new_block);
            }
        }
    }
}

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

