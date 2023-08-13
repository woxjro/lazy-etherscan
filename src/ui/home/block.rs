use crate::app::App;
use crate::route::{HomeRoute, Route};
use ethers_core::types::{Block as EBlock, H256};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, block: EBlock<H256>, rect: Rect) {
    let detail_block = Block::default()
        .title(format!("Block #{}", block.number.unwrap()))
        .border_style(if let Route::Home(HomeRoute::Block(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(rect)
        else {
            return;
        };

    let contents = Paragraph::new(Text::from(format!(
        "Block Height: {}",
        block.number.unwrap()
    )))
    .block(detail_block.to_owned())
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Left);

    f.render_widget(contents, detail_rect);
    f.render_widget(detail_block, rect);
}
