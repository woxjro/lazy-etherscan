use ethers_core::types::{Block, Transaction, U64};

#[derive(Clone, Debug)]
pub struct Statistics {
    pub ether_price: Option<U64>,
    pub market_cap: Option<U64>,
    pub transactions: Option<U64>,
    pub med_gas_price: Option<U64>,
    pub last_safe_block: Option<Block<Transaction>>,
    pub last_finalized_block: Option<Block<Transaction>>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            ether_price: None,
            market_cap: None,
            transactions: None,
            med_gas_price: None,
            last_safe_block: None,
            last_finalized_block: None,
        }
    }

    //pub const ETHER_PRICE_INDEX: usize = 0;
    //pub const TRANSACTIONS_INDEX: usize = 1;
    pub const LAST_SAFE_BLOCK_INDEX: usize = 2;
    //pub const MARKET_CAP_INDEX: usize = 3;
    //pub const MED_GAS_PRICE_INDEX: usize = 4;
    pub const LAST_FINALIZED_BLOCK_INDEX: usize = 5;
}
