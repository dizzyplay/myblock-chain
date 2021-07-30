use crate::block::Block;
use sha256::{digest};

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: Vec::new()
        }
    }

    pub fn add_block(&mut self, data:String) {
        match self.blocks.last() {
            Some(block)=>{
                let last_hash = block.hash.clone();
                let new_block = Block::new(data, last_hash);
                self.blocks.push(new_block);
            },
            None=>{
                let new_block = Block::new(data, String::new());
                self.blocks.push(new_block);
            }
        }
    }
}