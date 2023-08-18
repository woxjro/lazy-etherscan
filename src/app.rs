use crate::ethers::types::TransactionWithReceipt;
use crate::network::IoEvent;
use crate::route::{HomeRoute, Route};
use crate::widget::StatefulList;
use ethers_core::types::{Block, Transaction, U64};
use std::sync::mpsc::Sender;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub route: Route,
    io_tx: Option<Sender<IoEvent>>,
    pub is_loading: bool,
    pub sidebar_items: Vec<String>,
    pub latest_blocks: Option<StatefulList<Block<Transaction>>>,
    pub latest_transactions: Option<StatefulList<TransactionWithReceipt>>,
    //Search
    pub input_mode: InputMode,
    pub input: String,
    /// Position of cursor in the editor area.
    pub cursor_position: usize,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> App {
        App {
            route: Route::Home(HomeRoute::Search),
            is_loading: false,
            io_tx: Some(io_tx),
            sidebar_items: vec![
                "Latest Blocks".to_string(),
                "Latest Transactions".to_string(),
            ],
            latest_blocks: None,
            latest_transactions: None,
            input_mode: InputMode::Normal,
            input: "".to_owned(),
            cursor_position: 0,
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

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn paste(&mut self, data: String) {
        self.input = format!("{}{}", self.input, data);
        for _ in 0..data.len() {
            self.move_cursor_right();
        }
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        if let Ok(i) = self.input.to_string().parse::<u64>() {
            let number = U64::from(i);
            self.dispatch(IoEvent::GetBlock { number });
        }
        self.input.clear();
        self.reset_cursor();
    }
}
