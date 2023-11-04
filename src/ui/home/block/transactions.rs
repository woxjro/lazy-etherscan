use crate::app::App;
use crate::ethers::types::BlockWithTransactionReceipts;
use crate::route::{ActiveBlock, RouteId};
use crate::widget::Spinner;
use ethers::core::{
    types::{Transaction, U64},
    utils::{format_ether, format_units},
};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block_with_transaction_receipts: &BlockWithTransactionReceipts<Transaction>,
    rect: Rect,
) {
    let BlockWithTransactionReceipts {
        block,
        transaction_receipts,
    } = block_with_transaction_receipts;

    let selected_style = Style::default().add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = [
        "Hash",
        "Method",
        "Type",
        "From",
        "To",
        "Value (ETH)",
        "Fee",
        "Gas Price (Gwei)",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let items = block
        .transactions
        .iter()
        .map(|tx| {
            vec![
                Cell::from(format!("{}", tx.hash)).fg(Color::White),
                if tx.to.is_some() {
                    if tx.input.len() >= 4 {
                        Cell::from("ContractExecution").fg(Color::LightYellow) //TODO
                    } else {
                        Cell::from("Transfer").fg(Color::LightMagenta)
                    }
                } else {
                    Cell::from("ContractDeployment").fg(Color::LightCyan)
                },
                Cell::from(
                    (match tx.transaction_type {
                        Some(i) => {
                            if i == U64::from(1) {
                                "AccessList"
                            } else if i == U64::from(2) {
                                "EIP-1559"
                            } else {
                                "Unknown"
                            }
                        }
                        None => "Legacy",
                    })
                    .to_string(),
                )
                .fg(Color::White),
                Cell::from(format!("{}", tx.from)).fg(Color::White),
                Cell::from(tx.to.map_or("".to_owned(), |to| format!("{to}"))).fg(Color::White),
                Cell::from(format_ether(tx.value).to_string()).fg(Color::White),
                Cell::from(format!(
                    "{}",
                    if let Some(transaction_receipts) = transaction_receipts {
                        if let Some(transaction_receipt) = transaction_receipts
                            .iter()
                            .find(|receipt| receipt.transaction_hash == tx.hash)
                        {
                            transaction_receipt
                                .gas_used
                                .map_or(Spinner::default().to_string(), |gas_used| {
                                    format_ether(tx.gas_price.unwrap() * gas_used)
                                })
                        } else {
                            Spinner::default().to_string()
                        }
                    } else {
                        Spinner::default().to_string()
                    }
                ))
                .fg(Color::White),
                Cell::from(
                    format_units(tx.gas_price.unwrap(), "gwei")
                        .unwrap()
                        .to_string(),
                )
                .fg(Color::White),
            ]
        })
        .collect::<Vec<_>>();

    let rows = items
        .iter()
        .map(|cells| Row::new(cells.to_owned()).height(1).bottom_margin(1));

    let t = Table::new(rows.to_owned())
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Transactions")
                .fg(
                    if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                        if let RouteId::TransactionsOfBlock(_) = app.get_current_route().get_id() {
                            Color::Green
                        } else {
                            Color::White
                        }
                    } else {
                        Color::White
                    },
                ),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Max(12), //Hash
            Constraint::Max(18), //Method
            Constraint::Max(10), //Type
            Constraint::Max(12), //From
            Constraint::Max(12), //To
            Constraint::Max(20), //Value (ETH)
            Constraint::Max(10), //Fee
            Constraint::Max(20), //Gas Price (Gwei)
        ]);

    f.render_stateful_widget(t, rect, &mut app.transactions_table_state);
}
