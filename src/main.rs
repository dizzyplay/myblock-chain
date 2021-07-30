mod block;
mod block_chain;

fn main() {
    let mut chain = block_chain::BlockChain::new();
    chain.add_block(format!("hello"));
    chain.add_block(format!("hello"));
    chain.add_block(format!("hello"));
    println!("{:?}",chain);
}
