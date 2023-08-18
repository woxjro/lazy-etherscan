pub mod types {
    use ethers_core::types::{Transaction, TransactionReceipt};

    #[derive(Clone, Debug, PartialEq)]
    pub struct TransactionWithReceipt {
        pub transaction: Transaction,
        pub transaction_receipt: TransactionReceipt,
    }
} /* types */
