use crate::app::{statistics::Statistics, App};
use crate::ethers::types::{AddressInfo, BlockWithTransactionReceipts, TransactionWithReceipt};
use crate::route::{ActiveBlock, Route, RouteId};
use crate::widget::StatefulList;
use crate::Etherscan;
use ethers::{
    core::types::{
        Address, BlockId, BlockNumber, Chain, NameOrAddress, Transaction, TransactionReceipt,
        TxHash, H256, U64,
    },
    etherscan::Client,
    providers::{Http, Middleware, Provider},
};
use futures::future::{join_all, try_join, try_join3};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

const RATE_LIMIT: usize = 60;

pub enum IoEvent {
    GetStatistics,
    GetNameOrAddressInfo {
        name_or_address: NameOrAddress,
        is_searching: bool,
    },
    GetBlock {
        number: U64,
    },
    GetBlockByHash {
        hash: H256,
    },
    GetTransactionWithReceipt {
        transaction_hash: TxHash,
    },
    GetTransactionReceipts {
        transactions: Vec<Transaction>,
    },
    GetLatestBlocks {
        n: usize,
    },
    GetLatestTransactions {
        n: usize,
    },
    LookupAddresses {
        addresses: Vec<Address>,
    },
    InitialSetup {
        n: usize,
    },
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
            IoEvent::GetNameOrAddressInfo {
                name_or_address,
                is_searching,
            } => {
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
                if is_searching {
                    app.pop_current_route();
                }
                app.set_route(Route::new(
                    RouteId::AddressInfo(if let Ok(some) = res { some } else { None }),
                    ActiveBlock::Main,
                ));

                app.is_loading = false;
            }
            IoEvent::GetBlock { number } => {
                let res = Self::get_block(self.endpoint, number).await;
                if let Ok(block) = res {
                    {
                        let mut app = self.app.lock().await;
                        app.pop_current_route();
                        app.set_route(Route::new(
                            RouteId::Block(block.to_owned()),
                            ActiveBlock::Main,
                        ));
                    }

                    if let Some(block) = block {
                        let mut addresses = vec![];
                        for transaction in block.block.transactions {
                            addresses.push(transaction.from);
                            if let Some(to) = transaction.to {
                                addresses.push(to);
                            }
                        }

                        let _ = self.update_app_with_ens_ids(&addresses).await;
                    }
                }
                let mut app = self.app.lock().await;
                app.is_loading = false;
            }
            IoEvent::GetBlockByHash { hash } => {
                let res = Self::get_block(self.endpoint, hash).await;
                if let Ok(block) = res {
                    {
                        let mut app = self.app.lock().await;
                        app.pop_current_route();
                        app.set_route(Route::new(
                            RouteId::Block(block.to_owned()),
                            ActiveBlock::Main,
                        ));
                    }

                    if let Some(block) = block {
                        let mut addresses = vec![];
                        for transaction in block.block.transactions {
                            addresses.push(transaction.from);
                            if let Some(to) = transaction.to {
                                addresses.push(to);
                            }
                        }

                        let _ = self.update_app_with_ens_ids(&addresses).await;
                    }
                }
                let mut app = self.app.lock().await;
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
            IoEvent::GetTransactionReceipts { transactions } => {
                let splitted_transactions = transactions.chunks(RATE_LIMIT).collect::<Vec<_>>();

                for transactions in splitted_transactions {
                    let res = Self::get_transaction_receipts(self.endpoint, transactions).await;
                    let mut app = self.app.lock().await;
                    if let Ok(receipts) = res {
                        app.update_block_with_transaction_receipts(receipts);
                    }
                }
                let mut app = self.app.lock().await;
                app.is_loading = false;
            }
            IoEvent::InitialSetup { n } => {
                let (statistics, blocks, transactions) = try_join3(
                    Self::get_statistics(
                        self.endpoint,
                        self.etherscan
                            .as_ref()
                            .and_then(|etherscan| etherscan.api_key.to_owned()),
                    ),
                    Self::get_latest_blocks(self.endpoint, n),
                    Self::get_latest_transactions(self.endpoint, n),
                )
                .await
                .unwrap(); //TODO: remove unwrap
                let mut addresses = vec![];
                for transaction in &transactions {
                    addresses.push(transaction.transaction.from);
                    if let Some(to) = transaction.transaction.to {
                        addresses.push(to);
                    }
                }

                {
                    let mut app = self.app.lock().await;
                    app.statistics = statistics;

                    app.latest_blocks = Some(StatefulList::with_items(blocks));
                    app.latest_transactions = Some(StatefulList::with_items(transactions));
                }

                let _ = self.update_app_with_ens_ids(&addresses).await;

                let mut app = self.app.lock().await;
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

                let mut addresses = vec![];
                for transaction in &transactions {
                    addresses.push(transaction.transaction.from);
                    if let Some(to) = transaction.transaction.to {
                        addresses.push(to);
                    }
                }

                {
                    let mut app = self.app.lock().await;
                    app.latest_transactions = Some(StatefulList::with_items(transactions));
                }

                let _ = self.update_app_with_ens_ids(&addresses).await;

                let mut app = self.app.lock().await;
                app.is_loading = false;
            }
            IoEvent::LookupAddresses { addresses } => {
                let _ = self.update_app_with_ens_ids(&addresses).await;
                let mut app = self.app.lock().await;
                app.is_loading = false;
            }
        }
    }

    async fn get_block<T: Into<BlockId> + Send + Sync>(
        endpoint: &'a str,
        block_hash_or_number: T,
    ) -> Result<Option<BlockWithTransactionReceipts<Transaction>>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let block = provider.get_block_with_txs(block_hash_or_number).await?;
        let query = if let Some(block) = block.as_ref() {
            block
                .transactions
                .iter()
                .map(|tx| provider.get_transaction_receipt(tx.hash))
                .collect()
        } else {
            vec![]
        };
        let transaction_receipts = join_all(query).await;
        let transaction_receipts = transaction_receipts
            .iter()
            .filter_map(|receipt| receipt.as_ref().ok().and_then(|receipt| receipt.clone()))
            .collect::<Vec<_>>();

        if let Some(block) = block {
            Ok(Some(BlockWithTransactionReceipts {
                block,
                transaction_receipts: Some(transaction_receipts),
            }))
        } else {
            Ok(None)
        }
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
            contract_abi: None,
            contract_source_code: None,
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

        let contract_source_code = if let Some(api_key) = etherscan_api_key.to_owned() {
            let client = Client::builder()
                .with_api_key(api_key)
                .chain(Chain::Mainnet)?
                .build()?;

            client.contract_source_code(address).await.ok()
        } else {
            None
        };

        let contract_abi = if let Some(api_key) = etherscan_api_key {
            let client = Client::builder()
                .with_api_key(api_key)
                .chain(Chain::Mainnet)?
                .build()?;

            client.contract_abi(address).await.ok()
        } else {
            None
        };

        let balance = provider.get_balance(address, None /* TODO */).await?;

        //TODO: Not Found
        Ok(Some(AddressInfo {
            address,
            balance,
            avatar_url,
            contract_abi,
            contract_source_code,
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
    ) -> Result<Vec<BlockWithTransactionReceipts<Transaction>>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let block_number = provider.get_block_number().await?;

        let mut blocks = vec![];
        for i in 0..n {
            let block = provider.get_block_with_txs(block_number - i);
            blocks.push(block);
        }

        let blocks = join_all(blocks).await;

        let mut latest_blocks = vec![];
        for block in blocks.into_iter().flatten().flatten() {
            latest_blocks.push(BlockWithTransactionReceipts {
                block,
                transaction_receipts: None,
            });
        }
        Ok(latest_blocks)
    }

    async fn get_transaction_receipts(
        endpoint: &'a str,
        transactions: &[Transaction],
    ) -> Result<Vec<TransactionReceipt>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let query = transactions
            .iter()
            .map(|tx| provider.get_transaction_receipt(tx.hash))
            .collect::<Vec<_>>();
        let res = join_all(query).await;
        let mut transaction_receips = vec![];

        for receipt in res.into_iter().flatten().flatten() {
            transaction_receips.push(receipt);
        }

        Ok(transaction_receips)
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

    async fn update_app_with_ens_ids(
        &mut self,
        addresses: &[Address],
    ) -> Result<(), Box<dyn Error>> {
        let chunked_addresses = addresses.chunks(RATE_LIMIT).collect::<Vec<_>>();
        for addresses in chunked_addresses {
            let results = Self::lookup_addresses(self.endpoint, addresses).await?;
            let mut app = self.app.lock().await;

            for (address, ens_id) in results {
                if ens_id.is_some() {
                    app.address2ens_id.insert(address, ens_id);
                } else {
                    app.address2ens_id.entry(address).or_insert(ens_id);
                }
            }
        }
        Ok(())
    }

    async fn lookup_addresses(
        endpoint: &'a str,
        addresses: &[Address],
    ) -> Result<Vec<(Address, Option<String>)>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let query = addresses
            .iter()
            .map(|&address| provider.lookup_address(address))
            .collect::<Vec<_>>();

        let results = join_all(query).await;

        let res = addresses
            .iter()
            .zip(results.iter())
            .map(|(address, ens_id)| {
                (
                    address.to_owned(),
                    ens_id
                        .as_ref()
                        .map_or(None, |ens_id| Some(ens_id.to_owned())),
                )
            })
            .collect::<Vec<_>>();

        Ok(res)
    }
}
