use ethers_core::types::{Block, H256};

#[derive(PartialEq, Clone)]
pub enum Route {
    Home(HomeRoute),
}

#[derive(PartialEq, Clone)]
pub enum HomeRoute {
    Root,
    Search,
    LatestBlocks,
    LatestTransactions,
    Block(Block<H256>),
    //Transaction(usize),
}
