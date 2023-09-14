use crate::ethers::types::{AddressInfo, TransactionWithReceipt};
use ethers_core::types::{Block, Transaction};

#[derive(PartialEq, Clone)]
pub enum RouteId {
    Welcome,
    AddressInfo(Option<AddressInfo>),
    Block(Option<Block<Transaction>>),
    TransactionsOfBlock(Option<Block<Transaction>>),
    WithdrawalsOfBlock(Option<Block<Transaction>>),
    Transaction(Option<TransactionWithReceipt>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    SearchBar,
    LatestBlocks,
    LatestTransactions,
    Main,
}

#[derive(PartialEq, Clone)]
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
