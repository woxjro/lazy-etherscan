use crate::app::App;
use crate::route::ActiveBlock;
use ethers_core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let detail_block = Block::default()
        .border_style(if let ActiveBlock::Main = app.route.get_active_block() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        })
        .padding(Padding::new(2, 2, 2, 0))
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain);

    let mut lines = vec![
        format!("{:<20}: {}", "Block Height", block.number.unwrap()),
        //format!("{:<20}: {}", "Status", TODO),
        format!("{:<20}: {}", "Timestamp", block.time().unwrap().to_string()),
        //format!("{:<20}: Block proposed on slot {}, epoch {}", "Proposed On", TODO),
        format!(
            "{:<20}: {} transactions",
            "Transactions ",
            block.transactions.len()
        ),
    ];

    //if past Shanghai
    if let Some(withdrawals) = block.withdrawals.as_ref() {
        lines.push(format!(
            "{:<20}: {} withdrawals in this block",
            "Withdrawals",
            withdrawals.len()
        ));
    }

    let lines = lines
        .iter()
        .map(|row| Line::from(Span::raw(row).fg(Color::White)))
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(lines)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
