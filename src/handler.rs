use crate::{block_chain, WebResult, DB};
use warp::Reply;

pub async fn list(db: DB) -> WebResult<impl Reply> {
    let block_chain = db.read().await;

    Ok(warp::reply::json(&block_chain.blocks))
}

pub async fn add(db:DB) -> WebResult<impl Reply>{
    db.write().await.add_block(format!("new"));
    let block_chain = db.read().await;
    Ok(warp::reply::json(&block_chain.blocks))
}