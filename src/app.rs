use crate::network::IoEvent;
use crate::route::Route;
use ethers_core::types::{Block, Transaction, H256};
use std::sync::mpsc::Sender;

pub enum Field {
    LatestBlocks,
    LatestTransactions,
    Bottom,
    Details,
}

impl Field {
    pub fn get_index(&self) -> usize {
        match self {
            Self::LatestBlocks => 0,
            Self::LatestTransactions => 1,
            Self::Bottom => 2,
            Self::Details => 3,
        }
    }
}

#[derive(Debug)]
pub enum SidebarCategory {
    LatestBlocks,
    LatestTransactions,
    Bottom,
}

impl From<usize> for SidebarCategory {
    fn from(id: usize) -> Self {
        if id == 0 {
            Self::LatestBlocks
        } else if id == 1 {
            Self::LatestTransactions
        } else if id == 2 {
            Self::Bottom
        } else {
            panic!()
        }
    }
}

pub struct App {
    pub route: Route,
    io_tx: Option<Sender<IoEvent>>,
    pub is_loading: bool,
    pub sidebar_items: Vec<String>,
    pub focus: usize,
    pub details_about: Option<SidebarCategory>,
    pub latest_blocks: Option<Vec<Block<H256>>>,
    pub latest_transactions: Option<Vec<Transaction>>,
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
                "Bottom".to_string(),
            ],
            focus: 0,
            details_about: None,
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

    /*
    pub fn next(&mut self) {
        self.focus = (self.focus + 1) % self.sidebar_items.len();
    }


    pub fn previous(&mut self) {
        if self.focus > 0 {
            self.focus -= 1;
        } else {
            self.focus = self.sidebar_items.len() - 1;
        }
    }
    */
}
