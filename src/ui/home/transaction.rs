use crate::app::App;
use crate::route::{HomeRoute, Route};
use ethers_core::types::Transaction;
use ethers_core::utils::{format_ether, format_units};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, transaction: Transaction, rect: Rect) {
    let detail_block = Block::default()
        .title("Transaction Details")
        .border_style(if let Route::Home(HomeRoute::Transaction(_)) = app.route {
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

    let lines = [
        format!("{:<10}: #{}", "Block", transaction.block_number.unwrap()),
        format!("{:<10}: {}", "Txn Hash", transaction.hash),
        format!("{:<10}: {}", "From", transaction.from),
        format!("{:<10}: {}", "To", transaction.to.unwrap()),
        format!("{:<10}: {} ETH", "Value", format_ether(transaction.value)),
        format!("{:<10}: {}", "Type", transaction.transaction_type.unwrap()),
        format!("{:<10}: {}", "Gas", transaction.gas),
        format!(
            "{:<10}: {} Gwei",
            "Gas Price",
            format_units(transaction.gas_price.unwrap(), "gwei").unwrap()
        ),
    ];

    let lines = lines
        .iter()
        .map(|row| Line::from(Span::raw(row)))
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(lines)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, detail_rect);
    f.render_widget(detail_block, rect);
}
