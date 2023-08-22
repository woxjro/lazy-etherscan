use crate::app::App;
use crate::ethers::types::TransactionWithReceipt;
use crate::route::{HomeRoute, Route};
use ethers_core::types::U64;
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

    let mut details = vec![
        Line::from(Span::raw(format!(
            "{:<17}: {:#x}",
            "Transaction Hash", transaction.hash
        ))),
        Line::from(vec![
            Span::raw(format!("{:<17}: ", "Status")),
            transaction_receipt.status.map_or(Span::raw(""), |status| {
                if status == U64::from(0) {
                    Span::styled("Failure", Style::default().fg(Color::Red))
                } else {
                    Span::styled("Success", Style::default().fg(Color::Green))
                }
            }),
        ]),
        Line::from(Span::raw(format!(
            "{:<17}: #{}",
            "Block",
            transaction.block_number.unwrap()
        ))),
        Line::from(Span::raw(format!(
            "{:<17}: {:#x}",
            "From", transaction.from
        ))),
        Line::from(Span::raw(format!(
            "{:<17}: {}",
            "To",
            transaction
                .to
                .map_or("".to_owned(), |to| format!("{:#x}", to)),
        ))),
        Line::from(Span::raw(format!(
            "{:<17}: {}",
            "Transaction Type",
            transaction.transaction_type.map_or("Legacy", |ty| {
                if ty == U64::from(1) {
                    "1"
                } else {
                    "2(EIP-1559)"
                }
            })
        ))),
        Line::from(Span::raw(format!("{:<17}: {}", "Gas", transaction.gas))),
        Line::from(Span::raw(format!(
            "{:<17}: {} ETH",
            "Value",
            format_ether(transaction.value)
        ))),
        Line::from(Span::raw(format!(
            "{:<17}: {} ETH",
            "Transaction Fee",
            format_ether(transaction.gas_price.unwrap() * transaction_receipt.gas_used.unwrap())
        ))),
        Line::from(Span::raw(format!(
            "{:<17}: {} Gwei",
            "Gas Price",
            format_units(transaction.gas_price.unwrap(), "gwei").unwrap()
        ))),
    ];

    let input_data = transaction
        .input
        .to_string()
        .chars()
        .collect::<Vec<_>>()
        .chunks(64)
        .map(|window| window.iter().collect::<String>())
        .collect::<Vec<String>>();

    for (i, row) in input_data.iter().enumerate() {
        if i == 0 {
            details.push(Line::from(Span::raw(format!(
                "{:<17}: {}",
                "Input Data", row
            ))));
        } else {
            details.push(Line::from(Span::raw(format!("{:<19}{}", "", row))));
        }
    }

    let details = Paragraph::new(details)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(details, detail_rect);
    f.render_widget(detail_block, rect);
}
