pub mod types {
    use ethers::{
        core::types::{Address, Transaction, TransactionReceipt, U256},
        etherscan::contract::ContractMetadata,
    };
    use std::cmp::PartialEq;
    use url::Url;

    #[derive(Clone, Debug)]
    pub struct AddressInfo {
        pub address: Address,
        pub ens_id: Option<String>,
        pub avatar_url: Option<Url>,
        pub contract_metadata: Option<ContractMetadata>,
        pub balance: U256,
    }

    //impl PartialEq for ContractMetadata {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct TransactionWithReceipt {
        pub transaction: Transaction,
        pub transaction_receipt: TransactionReceipt,
    }
} /* types */
