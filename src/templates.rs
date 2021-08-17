use crate::block_chain;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "list.html")]
pub struct BlockChainList<'a> {
    pub block_chain: String,
    pub _parent: BaseTemplate<'a>,
}
