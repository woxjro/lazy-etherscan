use crate::{
    app::{statistics::Statistics, App},
    ethers::types::{AddressInfo, BlockWithTransactionReceipts, TransactionWithReceipt},
    route::{ActiveBlock, Route, RouteId},
    widget::StatefulList,
};
use ethers::{
    core::types::{
        Address, BlockId, BlockNumber, Chain, NameOrAddress, Transaction, TransactionReceipt,
        TxHash, H256, U64,
    },
    etherscan::Client,
    providers::{Http, Middleware, Provider},
};
use futures::future::{join_all, try_join, try_join3};
use std::{
    fs::File,
    io::Write,
    process::Command,
    {error::Error, sync::Arc},
};
use tempfile::tempdir;
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
    GetDecodedInputData {
        transaction: Transaction,
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
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>, endpoint: &'a str) -> Self {
        Self { app, endpoint }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetStatistics => {
                let res = Self::get_statistics(self.endpoint).await;
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
                        Self::get_address_info(self.endpoint, address).await
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
            IoEvent::GetDecodedInputData { transaction } => {
                let res = Self::get_decoded_input_data(transaction).await;

                let mut app = self.app.lock().await;
                if let Ok(decoded_input_data) = res {
                    let current_route = app.get_current_route();
                    match current_route.get_id() {
                        RouteId::Transaction(transaction)
                        | RouteId::InputDataOfTransaction(transaction) => {
                            app.pop_current_route();
                            let new_transaction =
                                transaction.map(|transaction| TransactionWithReceipt {
                                    transaction: transaction.transaction,
                                    transaction_receipt: transaction.transaction_receipt,
                                    decoded_input_data,
                                });
                            let new_route_id = match current_route.get_id() {
                                RouteId::Transaction(_) => RouteId::Transaction(new_transaction),
                                RouteId::InputDataOfTransaction(_) => {
                                    RouteId::InputDataOfTransaction(new_transaction)
                                }
                                _ => unreachable!(),
                            };
                            app.set_route(Route::new(
                                new_route_id,
                                current_route.get_active_block(),
                            ));
                        }
                        _ => {}
                    }
                }
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
                    Self::get_statistics(self.endpoint),
                    Self::get_latest_blocks(self.endpoint, n),
                    Self::get_latest_transactions(self.endpoint, n),
                )
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

    async fn get_name_info(
        endpoint: &'a str,
        ens_id: &str,
    ) -> Result<Option<AddressInfo>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let address = provider.resolve_name(ens_id).await?;

        let avatar_url = provider.resolve_avatar(ens_id).await.ok();
        let balance = provider.get_balance(address, None).await?;

        Ok(Some(AddressInfo {
            address,
            balance,
            avatar_url,
            contract_abi: None,
            contract_source_code: None,
            ens_id: Some(ens_id.to_owned()),
        }))
    }

    async fn get_address_info(
        endpoint: &'a str,
        address: Address,
    ) -> Result<Option<AddressInfo>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let ens_id = provider.lookup_address(address).await.ok();

        let avatar_url = if let Some(ens_id) = ens_id.as_ref() {
            provider.resolve_avatar(ens_id).await.ok()
        } else {
            None
        };

        let (contract_source_code, contract_abi) =
            if let Ok(client) = Client::new_from_env(Chain::Mainnet) {
                try_join(
                    client.contract_source_code(address),
                    client.contract_abi(address),
                )
                .await
                .map_or((None, None), |res| (Some(res.0), Some(res.1)))
            } else {
                (None, None)
            };

        let balance = provider.get_balance(address, None).await?;

        Ok(Some(AddressInfo {
            address,
            balance,
            avatar_url,
            contract_abi,
            contract_source_code,
            ens_id,
        }))
    }

    async fn get_decoded_input_data(
        transaction: Transaction,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let decoded_input_data = if let Ok(client) = Client::new_from_env(Chain::Mainnet) {
            if let Some(to) = transaction.to {
                let abi = client.contract_abi(to).await?;

                let s = serde_json::to_string(&abi)?;

                let dir = tempdir()?;
                let file_path = dir.path().join("lazy-etherscan.tmp.abi.json");
                let mut file = File::create(&file_path)?;
                writeln!(file, "{}", s)?;

                let output = Command::new("ethereum-input-data-decoder")
                    .args([
                        "--abi",
                        file_path.to_str().unwrap(),
                        &transaction.input.to_string(),
                    ])
                    .output()
                    .map_or(None, |output| String::from_utf8(output.stdout).ok());

                drop(file);
                dir.close()?;

                output
            } else {
                None
            }
        } else {
            None
        };
        Ok(decoded_input_data)
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
                let decoded_input_data = if let Ok(client) = Client::new_from_env(Chain::Mainnet) {
                    if let Some(to) = transaction.to {
                        let abi = client.contract_abi(to).await?;

                        let s = serde_json::to_string(&abi)?;

                        let dir = tempdir()?;
                        let file_path = dir.path().join("lazy-etherscan.tmp.abi.json");
                        let mut file = File::create(&file_path)?;
                        writeln!(file, "{}", s)?;

                        let output = Command::new("ethereum-input-data-decoder")
                            .args([
                                "--abi",
                                file_path.to_str().unwrap(),
                                &transaction.input.to_string(),
                            ])
                            .output()
                            .map_or(None, |output| String::from_utf8(output.stdout).ok());

                        drop(file);
                        dir.close()?;

                        output
                    } else {
                        None
                    }
                } else {
                    None
                };

                Ok(Some(TransactionWithReceipt {
                    transaction,
                    transaction_receipt,
                    decoded_input_data,
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
                decoded_input_data: None,
            });
        }

        Ok(result)
    }

    async fn get_statistics(endpoint: &'a str) -> Result<Statistics, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;

        let mut ethusd = None;
        let mut node_count = None;
        let mut suggested_base_fee = None;
        let mut med_gas_price = None;
        if let Ok(client) = Client::new_from_env(Chain::Mainnet) {
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
