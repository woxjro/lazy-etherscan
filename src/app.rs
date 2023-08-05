use crate::network::IoEvent;
use crate::route::Route;
use ethers_core::types::{Block, Transaction};
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
    pub sidebar_items: Vec<String>,
    pub focus: usize,
    pub details_about: Option<SidebarCategory>,
    pub latest_blocks: Option<Vec<Block<Transaction>>>,
    pub latest_transactions: Option<Vec<Transaction>>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> App {
        App {
            route: Route::Home,
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
