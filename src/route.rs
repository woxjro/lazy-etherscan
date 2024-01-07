use crate::ethers::types::{AddressInfo, BlockWithTransactionReceipts, TransactionWithReceipt};
use ethers::core::types::Transaction;

#[derive(Clone)]
pub enum RouteId {
    Welcome,
    Searching(String),
    AddressInfo(Option<AddressInfo>),
    Block(Option<BlockWithTransactionReceipts<Transaction>>),
    TransactionsOfBlock(Option<BlockWithTransactionReceipts<Transaction>>),
    WithdrawalsOfBlock(Option<BlockWithTransactionReceipts<Transaction>>),
    Transaction(Option<TransactionWithReceipt>),
    InputDataOfTransaction(Option<TransactionWithReceipt>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    SearchBar,
    LatestBlocks,
    LatestTransactions,
    Main,
}

#[derive(Clone)]
pub struct Route {
    id: RouteId,
    active_block: ActiveBlock,
}

impl Route {
    pub fn new(id: RouteId, active_block: ActiveBlock) -> Self {
        Self { id, active_block }
    }

    pub fn get_active_block(&self) -> ActiveBlock {
        self.active_block
    }

    pub fn get_id(&self) -> RouteId {
        self.id.to_owned()
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            id: RouteId::Welcome,
            active_block: ActiveBlock::SearchBar,
        }
    }
}
