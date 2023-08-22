use crate::app::App;
use crate::route::{HomeRoute, Route};
use ethers_core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let detail_block = Block::default()
        //.title(format!("Block #{}", block.number.unwrap()))
        .border_style(if let Route::Home(HomeRoute::Block(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .padding(Padding::horizontal(2))
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain);

    let lines = [
        format!(
            "{:<20}: {}",
            "Fee Recipient",
            if let Some(addr) = block.author {
                format!("{:#x}", addr)
            } else {
                format!("pending...")
            }
        ),
        //ref: https://docs.alchemy.com/docs/how-to-calculate-ethereum-miner-rewards#calculate-a-miner-reward
        //format!("Block Reward: {} ETH", /* TODO */):
        format!(
            "{:<20}: {}",
            "Total Difficulty",
            block.total_difficulty.unwrap()
        ),
        format!("{:<20}: {} bytes", "Size", block.size.unwrap()),
    ];

    let lines = lines
        .iter()
        .map(|row| Line::from(Span::raw(row)))
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(lines)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
