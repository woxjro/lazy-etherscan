use crate::app::App;
use crate::widget::StatefulList;
use ethers_core::types::{Block, Transaction, H256};
use ethers_providers::{Http, Middleware, Provider};
use futures::future::join_all;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum IoEvent {
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
            IoEvent::GetLatestBlocks { n } => {
                let mut app = self.app.lock().await;
                let blocks = Self::get_latest_blocks(self.endpoint, n).await.unwrap();
                app.latest_blocks = Some(StatefulList::with_items(blocks));
                app.is_loading = false;
            }
            IoEvent::GetLatestTransactions { n } => {
                let mut app = self.app.lock().await;
                let transactions = Self::get_latest_transactions(self.endpoint, n)
                    .await
                    .unwrap();
                app.latest_transactions = Some(StatefulList::with_items(transactions));
                app.is_loading = false;
            }
        }
    }

    async fn get_latest_blocks(
        endpoint: &'a str,
        n: usize,
    ) -> Result<Vec<Block<H256>>, Box<dyn Error>> {
        if n == 0 {
            Ok(vec![])
        } else {
            let provider = Provider::<Http>::try_from(endpoint)?;
            let block_number = provider.get_block_number().await?;

            let mut blocks = vec![];
            for i in 0..n {
                let block = provider.get_block(block_number - i);
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
    ) -> Result<Vec<Transaction>, Box<dyn Error>> {
        if n == 0 {
            Ok(vec![])
        } else {
            let provider = Provider::<Http>::try_from(endpoint)?;

            let block_number = provider.get_block_number().await?;

            let block = provider.get_block(block_number).await?;

            let txs = if let Some(block) = block {
                block
                    .transactions
                    .iter()
                    .take(n)
                    .map(|&tx| provider.get_transaction(tx))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            let txs = join_all(txs).await;

            let mut res = vec![];

            for tx in txs {
                res.push(tx.unwrap().unwrap());
            }

            Ok(res)
        }
    }
}