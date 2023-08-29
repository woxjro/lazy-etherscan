pub mod types {
    use ethers_core::types::{Address, Transaction, TransactionReceipt, U256};

    #[derive(Clone, Debug, PartialEq)]
    pub struct AddressInfo {
        pub address: Address,
        pub ens_id: String,
        pub balance: U256,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct TransactionWithReceipt {
        pub transaction: Transaction,
        pub transaction_receipt: TransactionReceipt,
    }
} /* types */
