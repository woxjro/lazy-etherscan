use crate::app::App;
use crate::ethers::types::TransactionWithReceipt;
use crate::route::{HomeRoute, Route};
use crate::widget::StatefulList;
use ethers_core::types::BlockNumber;
use ethers_core::types::{Block, Transaction, U64};
use ethers_providers::{Http, Middleware, Provider};
use futures::future::join_all;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum IoEvent {
    GetBlock { number: U64 },
    GetLatestBlocks { n: usize },
    GetLatestTransactions { n: usize },
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
            IoEvent::GetBlock { number } => {
                let res = Self::get_block(self.endpoint, number).await;
                let mut app = self.app.lock().await;
                if let Ok(some) = res {
                    if let Some(block) = some {
                        app.set_route(Route::Home(HomeRoute::Block(block)));
                    }
                }
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

    async fn get_block(
        endpoint: &'a str,
        number: U64,
    ) -> Result<Option<Block<Transaction>>, Box<dyn Error>> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let block = provider.get_block_with_txs(number).await?;
        Ok(block)
    }

    async fn get_latest_blocks(
        endpoint: &'a str,
        n: usize,
    ) -> Result<Vec<Block<Transaction>>, Box<dyn Error>> {
        if n == 0 {
            Ok(vec![])
        } else {
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
                res.push(block.unwrap().unwrap());
            }
            Ok(res)
        }
    }

    async fn get_latest_transactions(
        endpoint: &'a str,
        n: usize,
    ) -> Result<Vec<TransactionWithReceipt>, Box<dyn Error>> {
        if n == 0 {
            Ok(vec![])
        } else {
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
    }
}
