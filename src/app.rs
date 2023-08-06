use crate::network::IoEvent;
use crate::route::Route;
use crate::widget::StatefulList;
use ethers_core::types::{Block, Transaction, H256};
use std::sync::mpsc::Sender;

pub struct App {
    pub route: Route,
    io_tx: Option<Sender<IoEvent>>,
    pub is_loading: bool,
    pub sidebar_items: Vec<String>,
    pub latest_blocks: Option<StatefulList<Block<H256>>>,
    pub latest_transactions: Option<StatefulList<Transaction>>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> App {
        App {
            route: Route::Home,
            is_loading: false,
            io_tx: Some(io_tx),
            sidebar_items: vec![
                "Latest Blocks".to_string(),
                "Latest Transactions".to_string(),
            ],
            latest_blocks: None,
            latest_transactions: None,
        }
    }

    pub fn set_route(&mut self, route: Route) {
        self.route = route;
    }

    // Send a network event to the network thread
    pub fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in network.rs
        self.is_loading = true;
        if let Some(io_tx) = &self.io_tx {
            if let Err(e) = io_tx.send(action) {
                self.is_loading = false;
                println!("Error from dispatch {}", e);
                // TODO: handle error
            };
        }
    }
}
