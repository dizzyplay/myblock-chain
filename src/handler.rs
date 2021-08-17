use crate::templates::{BaseTemplate, BlockChainList};
use crate::{block_chain, WebResult, DB};
use askama::Template;
use warp::Reply;

pub async fn list(db: DB) -> WebResult<impl Reply> {
    let block_chain = db.read().await;

    let block_chain_list = BlockChainList {
        _parent: BaseTemplate {
            title: "Block chain List",
        },
        block_chain: format!("{}", block_chain),
    };
    Ok(warp::reply::html(block_chain_list.render().unwrap()))
}
