use crate::app::App;
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
}
