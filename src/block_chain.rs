use crate::block::Block;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.blocks.iter() {
            write!(f,"data: {} hash: {} prev_hash: {} \n",b.data, b.hash, b.prev_hash)?;
        }
        Ok(())
    }
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