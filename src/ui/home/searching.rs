use crate::widget::Spinner;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, word: &str, rect: Rect) {
    let searching_block = Block::default()
        .title(format!(
            "{} Searching for {word}",
            Spinner::default().to_string()
        ))
        .border_style(Style::default().fg(Color::Green))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    f.render_widget(searching_block, rect);
}
