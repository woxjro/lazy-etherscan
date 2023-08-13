mod home;
use crate::app::App;
use ratatui::prelude::*;

/// /home
pub fn ui_home<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    home::render_home_layout(f, app);
}
