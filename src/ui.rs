mod common;
use crate::app::App;
use ratatui::prelude::*;

/// /home
pub fn ui_home<B: Backend>(f: &mut Frame<B>, app: &App) {
    common::render_home_layout(f, app);
}

pub fn ui_search<B: Backend>(f: &mut Frame<B>, app: &App) {
    common::render_home_layout(f, app);
}

pub fn ui_blocks<B: Backend>(f: &mut Frame<B>, app: &App) {
    common::render_home_layout(f, app);
}

pub fn ui_transations<B: Backend>(f: &mut Frame<B>, app: &App) {
    common::render_home_layout(f, app);
}
