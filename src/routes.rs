use crate::{block_chain, handler, DB};
use warp::{self, Filter, Rejection, Reply};

pub fn api(db: DB) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path::end()
        .map(|| "block chain")
        .or(add(db.clone()))
        .or(list(db.clone()))
}

pub fn list(db: DB) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path("list")
        .and(block_chain::with_db(db.clone()))
        .and_then(handler::list)
}

pub fn add(db: DB) -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path("add")
        .and(block_chain::with_db(db.clone()))
        .and_then(handler::add)
}
