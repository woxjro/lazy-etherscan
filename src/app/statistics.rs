use ethers::core::types::{Block, Transaction, U256};

#[derive(Clone, Debug)]
pub struct Statistics {
    pub ethusd: Option<f64>,
    pub node_count: Option<usize>,
    pub suggested_base_fee: Option<U256>,
    pub med_gas_price: Option<U256>,
    pub last_safe_block: Option<Block<Transaction>>,
    pub last_finalized_block: Option<Block<Transaction>>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            ethusd: None,
            node_count: None,
            suggested_base_fee: None,
            med_gas_price: None,
            last_safe_block: None,
            last_finalized_block: None,
        }
    }

    pub const ETHUSD_INDEX: usize = 0;
    pub const SUGGESTED_BASE_FEE_INDEX: usize = 1;
    pub const LAST_SAFE_BLOCK_INDEX: usize = 2;
    pub const NODE_COUNT_INDEX: usize = 3;
    pub const MED_GAS_PRICE_INDEX: usize = 4;
    pub const LAST_FINALIZED_BLOCK_INDEX: usize = 5;
}
