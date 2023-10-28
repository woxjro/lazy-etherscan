pub mod types {
    use ethers_core::types::{Address, Transaction, TransactionReceipt, U256};
    use url::Url;

    #[derive(Clone, Debug, PartialEq)]
    pub struct AddressInfo {
        pub address: Address,
        pub ens_id: Option<String>,
        pub avatar_url: Option<Url>,
        pub balance: U256,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct TransactionWithReceipt {
        pub transaction: Transaction,
        pub transaction_receipt: TransactionReceipt,
    }
} /* types */
