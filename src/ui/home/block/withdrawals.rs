use crate::app::App;
use crate::route::{ActiveBlock, RouteId};
use ethers_core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let selected_style = Style::default().add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let header_cells = ["Index", "Validator Index", "Address", "Amount"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let items = block.withdrawals.as_ref().map_or(vec![], |withdrawals| {
        withdrawals
            .iter()
            .map(|withdrawal| {
                vec![
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
                .title("Transactions")
                .fg(if let ActiveBlock::Main = app.route.get_active_block() {
                    if let RouteId::TransactionsOfBlock(_) = app.route.get_id() {
                        Color::Green
                    } else {
                        Color::White
                    }
                } else {
                    Color::White
                }),
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Max(12), //Index
            Constraint::Max(12), //Validator Index
            Constraint::Max(12), //Address
            Constraint::Max(12), //Amount
        ]);

    f.render_stateful_widget(t, rect, &mut app.transactions_table_state);
}
