use crate::app::{statistics::Statistics, App};
use crate::ethers::types::{AddressInfo, TransactionWithReceipt};
use crate::route::{ActiveBlock, Route, RouteId};
use crate::widget::StatefulList;
use crate::Etherscan;
use ethers::{
    core::types::{
        Address, Block, BlockId, BlockNumber, Chain, NameOrAddress, Transaction, TxHash, H256, U64,
    },
    etherscan::Client,
    providers::{Http, Middleware, Provider},
};
use futures::future::{join_all, try_join, try_join3};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

pub enum IoEvent {
    GetStatistics,
    GetNameOrAddressInfo { name_or_address: NameOrAddress },
    GetBlock { number: U64 },
    GetBlockByHash { hash: H256 },
    GetTransactionWithReceipt { transaction_hash: TxHash },
    GetLatestBlocksAndTransactions { n: usize },
    GetLatestBlocks { n: usize },
    GetLatestTransactions { n: usize },
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
    endpoint: &'a str,
    etherscan: &'a Option<Etherscan>,
}

impl<'a> Network<'a> {
    pub fn new(
        app: &'a Arc<Mutex<App>>,
        endpoint: &'a str,
        etherscan: &'a Option<Etherscan>,
    ) -> Self {
        Self {
            app,
            endpoint,
            etherscan,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetStatistics => {
                let res = Self::get_statistics(
                    self.endpoint,
                    self.etherscan
                        .as_ref()
                        .and_then(|etherscan| etherscan.api_key.to_owned()),
                )
                .await;
                let mut app = self.app.lock().await;
                if let Ok(statistics) = res {
                    app.statistics = statistics;
                }
                app.is_loading = false;
            }
            IoEvent::GetNameOrAddressInfo { name_or_address } => {
                let res = match name_or_address {
                    NameOrAddress::Name(name) => Self::get_name_info(self.endpoint, &name).await,
                    NameOrAddress::Address(address) => {
                        Self::get_address_info(
                            self.endpoint,
                            self.etherscan
                                .as_ref()
                                .and_then(|etherscan| etherscan.api_key.to_owned()),
                            address,
                        )
                        .await
                    }
                };
                let mut app = self.app.lock().await;
                app.set_route(Route::new(
                    RouteId::AddressInfo(if let Ok(some) = res { some } else { None }),
                    ActiveBlock::Main,
                ));

                app.is_loading = false;
            }
            IoEvent::GetBlock { number } => {
                let res = Self::get_block(self.endpoint, number).await;
                let mut app = self.app.lock().await;
                if let Ok(some) = res {
                    app.set_route(Route::new(RouteId::Block(some), ActiveBlock::Main));
                }
                app.is_loading = false;
            }
            IoEvent::GetBlockByHash { hash } => {
                let res = Self::get_block(self.endpoint, hash).await;
                let mut app = self.app.lock().await;
                if let Ok(some) = res {
                    app.set_route(Route::new(RouteId::Block(some), ActiveBlock::Main));
                }
                app.is_loading = false;
            }
            IoEvent::GetTransactionWithReceipt { transaction_hash } => {
                let res = Self::get_transaction_with_receipt(self.endpoint, transaction_hash).await;
                let mut app = self.app.lock().await;
                if let Ok(some) = res {
                    app.set_route(Route::new(RouteId::Transaction(some), ActiveBlock::Main));
                }
                app.is_loading = false;
            }
            IoEvent::GetLatestBlocksAndTransactions { n } => {
                let (blocks, transactions) = try_join(
                    Self::get_latest_blocks(self.endpoint, n),
                    Self::get_latest_transactions(self.endpoint, n),
                )
                .await
                .unwrap();
                let mut app = self.app.lock().await;
                app.latest_blocks = Some(StatefulList::with_items(blocks));
                app.latest_transactions = Some(StatefulList::with_items(transactions));
                app.is_loading = false;
            }
            IoEvent::GetLatestBlocks { n } => {
                let blocks = Self::get_latest_blocks(self.endpoint, n).await.unwrap();
                let mut app = self.app.lock().await;
                app.latest_blocks = Some(StatefulList::with_items(blocks));
                app.is_loading = false;
            }
            IoEvent::GetLatestTransactions { n } => {
                let transactions = Self::get_latest_transactions(self.endpoint, n)
                    .await
                    .unwrap();
                let mut app = self.app.lock().await;
                app.latest_transactions = Some(StatefulList::with_items(transactions));
                app.is_loading = false;
            }
        }
    }

    async fn get_block<T: Into<BlockId> + Send + Sync>(
        endpoint: &'a str,
        block_hash_or_number: T,
    ) -> Result<Option<Block<Transaction>>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let block = provider.get_block_with_txs(block_hash_or_number).await?;
        Ok(block)
    }

    //TODO: use `join`
    async fn get_name_info(
        endpoint: &'a str,
        ens_id: &str,
    ) -> Result<Option<AddressInfo>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let address = provider.resolve_name(ens_id).await?;

        let avatar_url = provider.resolve_avatar(ens_id).await.ok();
        let balance = provider.get_balance(address, None /* TODO */).await?;
        //TODO: Not Found (impl LazyEtherscanError)
        Ok(Some(AddressInfo {
            address,
            balance,
            avatar_url,
            contract_metadata: None,
            ens_id: Some(ens_id.to_owned()),
        }))
    }

    //TODO: use `join`
    async fn get_address_info(
        endpoint: &'a str,
        etherscan_api_key: Option<String>,
        address: Address,
    ) -> Result<Option<AddressInfo>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let ens_id = provider.lookup_address(address).await.ok();

        let avatar_url = if let Some(ens_id) = ens_id.as_ref() {
            provider.resolve_avatar(ens_id).await.ok()
        } else {
            None
        };

        let contract_metadata = if let Some(api_key) = etherscan_api_key {
            let client = Client::builder()
                .with_api_key(api_key)
                .chain(Chain::Mainnet)?
                .build()?;

            client.contract_source_code(address).await.ok()
        } else {
            None
        };

        let balance = provider.get_balance(address, None /* TODO */).await?;

        //TODO: Not Found
        Ok(Some(AddressInfo {
            address,
            balance,
            avatar_url,
            contract_metadata,
            ens_id,
        }))
    }

    async fn get_transaction_with_receipt(
        endpoint: &'a str,
        transaction_hash: TxHash,
    ) -> Result<Option<TransactionWithReceipt>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let transaction = provider.get_transaction(transaction_hash).await?;
        let transaction_receipt = provider.get_transaction_receipt(transaction_hash).await?;
        if let Some(transaction) = transaction {
            if let Some(transaction_receipt) = transaction_receipt {
                Ok(Some(TransactionWithReceipt {
                    transaction,
                    transaction_receipt,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn get_latest_blocks(
        endpoint: &'a str,
        n: usize,
    ) -> Result<Vec<Block<Transaction>>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let block_number = provider.get_block_number().await?;

        let mut blocks = vec![];
        for i in 0..n {
            let block = provider.get_block_with_txs(block_number - i);
            blocks.push(block);
        }

        let blocks = join_all(blocks).await;

        let mut res = vec![];
        for block in blocks {
            //TODO
            res.push(block.unwrap().unwrap());
        }
        Ok(res)
    }

    async fn get_latest_transactions(
        endpoint: &'a str,
        n: usize,
    ) -> Result<Vec<TransactionWithReceipt>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;

        let block = provider.get_block(BlockNumber::Latest).await?;

        let transaction_futures = if let Some(block) = block {
            block
                .transactions
                .iter()
                .take(n)
                .map(|&tx| provider.get_transaction(tx))
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        let transactions = join_all(transaction_futures).await;
        let transactions = transactions
            .iter()
            .filter_map(|tx| tx.as_ref().ok().and_then(|tx| tx.clone()))
            .collect::<Vec<_>>();

        let receipt_futures = transactions
            .iter()
            .map(|tx| provider.get_transaction_receipt(tx.hash))
            .collect::<Vec<_>>();
        let receipts = join_all(receipt_futures).await;
        let receipts = receipts
            .iter()
            .filter_map(|tx| tx.as_ref().ok().and_then(|receipt| receipt.clone()))
            .collect::<Vec<_>>();

        let mut result = vec![];
        for i in 0..receipts.len() {
            result.push(TransactionWithReceipt {
                transaction: transactions[i].to_owned(),
                transaction_receipt: receipts[i].to_owned(),
            });
        }

        Ok(result)
    }

    async fn get_statistics(
        endpoint: &'a str,
        etherscan_api_key: Option<String>,
    ) -> Result<Statistics, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;

        let mut ethusd = None;
        let mut node_count = None;
        let mut suggested_base_fee = None;
        let mut med_gas_price = None;
        if let Some(api_key) = etherscan_api_key {
            let client = Client::builder()
                .with_api_key(api_key)
                .chain(Chain::Mainnet)?
                .build()?;

            let (eth_price, total_node_count, gas_oracle) =
                try_join3(client.eth_price(), client.node_count(), client.gas_oracle()).await?;

            ethusd = Some(eth_price.ethusd);
            node_count = Some(total_node_count.total_node_count);
            suggested_base_fee = Some(gas_oracle.suggested_base_fee);
            med_gas_price = Some(gas_oracle.propose_gas_price);
        }

        let (last_safe_block, last_finalized_block) = try_join(
            provider.get_block_with_txs(BlockNumber::Safe),
            provider.get_block_with_txs(BlockNumber::Finalized),
        )
        .await?;

        Ok(Statistics {
            ethusd,
            node_count,
            suggested_base_fee,
            med_gas_price,
            last_safe_block,
            last_finalized_block,
        })
    }
}
