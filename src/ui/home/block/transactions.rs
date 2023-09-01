use crate::app::App;
use crate::route::ActiveBlock;
use ethers_core::types::{Block as EBlock, Transaction, U64};
use ethers_core::utils::{format_ether, format_units};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let selected_style = Style::default().add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = [
        "Hash",
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
                format!("{}", tx.hash),
                format!(
                    "{}",
                    match tx.transaction_type {
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
                    }
                ),
                format!("{}", tx.from),
                tx.to.map_or("".to_owned(), |to| format!("{to}")),
                format!("{}", format_ether(tx.value)),
                //TODO:format!( "{}", format_ether(tx.gas_price.unwrap() * tx_receipt.gas_used)),
                //transaction_receipt.gas_usedが必要
                format!(""),
                format!("{}", format_units(tx.gas_price.unwrap(), "gwei").unwrap()),
            ]
        })
        .collect::<Vec<_>>();
    let rows = items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item
            .iter()
            .map(|c| Cell::from(c.to_owned()).fg(Color::White));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows.to_owned())
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Transactions")
                .fg(if let ActiveBlock::Main = app.route.get_active_block() {
                    Color::Green
                } else {
                    Color::White
                }),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Max(15),
            Constraint::Max(10),
            Constraint::Max(15),
            Constraint::Max(15),
            Constraint::Max(20),
            Constraint::Max(10),
            Constraint::Max(20),
        ]);

    f.render_stateful_widget(t, rect, &mut app.transactions_table_state);
}
