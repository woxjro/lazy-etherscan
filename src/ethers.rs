pub mod types {
    use ethers::{
        core::types::{Address, Block, Transaction, TransactionReceipt, U256},
        etherscan::contract::ContractMetadata,
    };
    use serde::{Deserialize, Deserializer};
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

    #[derive(Clone, Debug, PartialEq)]
    pub struct TransactionWithReceipt {
        pub transaction: Transaction,
        pub transaction_receipt: TransactionReceipt,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct BlockWithTransactionReceipts<T> {
        pub block: Block<T>,
        pub transaction_receipts: Option<Vec<TransactionReceipt>>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct ERC20Token {
        pub name: String,
        pub ticker: String,
        #[serde(deserialize_with = "deserialize_address_from_string")]
        pub contract_address: Address,
    }

    fn deserialize_address_from_string<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        s.parse::<Address>().map_err(serde::de::Error::custom)
    }

    impl ERC20Token {
        pub fn find_by_address(erc20_tokens: &[Self], address: Address) -> Option<Self> {
            erc20_tokens
                .iter()
                .find(|erc20_token| erc20_token.contract_address == address)
                .map(|token| token.to_owned())
        }

        pub fn find_by_ticker(erc20_tokens: &[Self], ticker: &str) -> Option<Self> {
            erc20_tokens
                .iter()
                .find(|erc20_token| erc20_token.ticker == ticker)
                .map(|token| token.to_owned())
        }
    }
} /* types */
