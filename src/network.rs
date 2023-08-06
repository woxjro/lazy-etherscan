use crate::app::App;
use ethers_core::types::{Block, H256};
use ethers_providers::{Http, Middleware, Provider};
use futures::future::join_all;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum IoEvent {
    GetLatestBlocks,
    GetLatestTransactions,
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Self { app: app }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetLatestBlocks => {
                let mut app = self.app.lock().await;
                let blocks = get_latest_blocks().await.unwrap();
                app.latest_blocks = Some(blocks);
                app.is_loading = false;
            }
            IoEvent::GetLatestTransactions => todo!(),
        }
    }
}

async fn get_latest_blocks() -> Result<Vec<Block<H256>>, Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com")?;

    let block_number = provider.get_block_number().await?;

    let mut blocks = vec![];
    for i in 0..10 {
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
