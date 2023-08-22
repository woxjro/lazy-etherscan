use crate::ethers::types::TransactionWithReceipt;
use ethers_core::types::{Block, Transaction};

#[derive(PartialEq, Clone)]
pub enum Route {
    Home(HomeRoute),
}

#[derive(PartialEq, Clone)]
pub enum HomeRoute {
    Search,
    LatestBlocks,
    LatestTransactions,
    Block(Block<Transaction>),
    Transaction(TransactionWithReceipt),
}
