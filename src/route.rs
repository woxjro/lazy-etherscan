use ethers_core::types::{Block, Transaction};

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
    Block(Block<Transaction>),
    Transaction(Transaction),
}
