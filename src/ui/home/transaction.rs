use crate::app::App;
use crate::ethers::types::TransactionWithReceipt;
use crate::route::{HomeRoute, Route};
use ethers_core::utils::{format_ether, format_units};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    transaction_with_receipt: TransactionWithReceipt,
    rect: Rect,
) {
    let TransactionWithReceipt {
        transaction,
        transaction_receipt,
    } = transaction_with_receipt;

    let detail_block = Block::default()
        .title("Transaction Details")
        .border_style(if let Route::Home(HomeRoute::Transaction(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .padding(Padding::new(2, 2, 1, 1))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(rect) else { return; };

    let lines = [
        format!("{:<15}: {}", "Txn Hash", transaction.hash),
        format!("{:<15}: #{}", "Block", transaction.block_number.unwrap()),
        format!("{:<15}: {}", "From", transaction.from),
        format!("{:<15}: {}", "To", transaction.to.unwrap()),
        format!("{:<15}: {}", "Type", transaction.transaction_type.unwrap()),
        format!("{:<15}: {}", "Gas", transaction.gas),
        format!("{:<15}: {} ETH", "Value", format_ether(transaction.value)),
        format!(
            "{:<15}: {} ETH",
            "Transaction Fee",
            format_ether(transaction.gas_price.unwrap() * transaction_receipt.gas_used.unwrap())
        ),
        format!(
            "{:<15}: {} Gwei",
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
