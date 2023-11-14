use crate::app::App;
use crate::ethers::types::BlockWithTransactionReceipts;
use crate::route::{ActiveBlock, RouteId};
use ethers::core::types::Transaction;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block_with_transaction_receipts: &BlockWithTransactionReceipts<Transaction>,
    rect: Rect,
) {
    let BlockWithTransactionReceipts {
        block,
        transaction_receipts: _,
    } = block_with_transaction_receipts;

    let selected_style = Style::default().add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = ["", "Index", "Validator Index", "Address", "Amount"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let items = block.withdrawals.as_ref().map_or(vec![], |withdrawals| {
        withdrawals
            .iter()
            .enumerate()
            .map(|(i, withdrawal)| {
                vec![
                    Cell::from(format!("{}", i + 1)).fg(Color::White),
                    Cell::from(format!("{}", withdrawal.index)).fg(Color::White),
                    Cell::from(format!("{}", withdrawal.validator_index)).fg(Color::White),
                    Cell::from(format!("{}", withdrawal.address)).fg(Color::White),
                    Cell::from(format!("{}", withdrawal.amount)).fg(Color::White),
                ]
            })
            .collect::<Vec<_>>()
    });

    let rows = items
        .iter()
        .map(|cells| Row::new(cells.to_owned()).height(1).bottom_margin(1));

    let t = Table::new(rows.to_owned())
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Withdrawals")
                .fg(
                    if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                        if let RouteId::WithdrawalsOfBlock(_) = app.get_current_route().get_id() {
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
            Constraint::Max(3),
            Constraint::Max(12), //Index
            Constraint::Max(16), //Validator Index
            Constraint::Max(12), //Address
            Constraint::Max(12), //Amount
        ]);

    f.render_stateful_widget(t, rect, &mut app.withdrawals_table_state);
}
