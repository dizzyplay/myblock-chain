mod block_chain;
mod handler;
mod routes;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Rejection;

pub type WebResult<T> = Result<T, Rejection>;
pub type DB = Arc<RwLock<block_chain::BlockChain>>;

#[tokio::main]
async fn main() {
    let chain: DB = Arc::new(RwLock::new(block_chain::BlockChain::new()));
    chain.write().await.add_block(format!("hello"));
    chain.write().await.add_block(format!("hello"));
    chain.write().await.add_block(format!("hello"));
    println!("{}", chain.read().await);

    let api = routes::api(chain.clone());

    warp::serve(api).run(([0, 0, 0, 0], 3000)).await;
}
