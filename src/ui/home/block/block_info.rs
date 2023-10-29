use crate::app::{block::SelectableBlockDetailItem, App};
use crate::route::{ActiveBlock, RouteId};
use ethers::core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let detail_block = Block::default()
        .border_style(
            if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                if let RouteId::Block(_) = app.get_current_route().get_id() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                }
            } else {
                Style::default().fg(Color::White)
            },
        )
        .padding(Padding::new(2, 2, 2, 0))
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain);

    let mut lines = vec![
        Line::from(
            Span::raw(format!("{:<20}: {}", "Block Height", block.number.unwrap()))
                .fg(Color::White),
        ),
        //format!("{:<20}: {}", "Status", TODO),
        Line::from(
            Span::raw(format!(
                "{:<20}: {}",
                "Timestamp",
                block.time().map_or("".to_string(), |time| time.to_string())
            ))
            .fg(Color::White),
        ),
        //format!("{:<20}: Block proposed on slot {}, epoch {}", "Proposed On", TODO),
    ];

    let transactions_span = Span::raw(format!(
        "{:<20}: {} {} transactions",
        "Transactions ",
        if let RouteId::TransactionsOfBlock(_) = app.get_current_route().get_id() {
            "▼"
        } else {
            "▶"
        },
        block.transactions.len()
    ))
    .fg(Color::White);

    lines.push(
        if let RouteId::TransactionsOfBlock(_) = app.get_current_route().get_id() {
            Line::from(transactions_span.add_modifier(Modifier::BOLD))
        } else if app.block_detail_list_state.selected()
            == Some(SelectableBlockDetailItem::Transactions.into())
        {
            Line::from(transactions_span.add_modifier(Modifier::BOLD))
        } else {
            Line::from(transactions_span)
        },
    );

    //if past Shanghai
    if let Some(withdrawals) = block.withdrawals.as_ref() {
        let withdrawals_span = Span::raw(format!(
            "{:<20}: {} {} withdrawals in this block",
            "Withdrawals",
            if let RouteId::WithdrawalsOfBlock(_) = app.get_current_route().get_id() {
                "▼"
            } else {
                "▶"
            },
            withdrawals.len()
        ))
        .fg(Color::White);
        lines.push(Line::from(
            if app.block_detail_list_state.selected()
                == Some(SelectableBlockDetailItem::Withdrawls.into())
            {
                withdrawals_span.add_modifier(Modifier::BOLD)
            } else {
                withdrawals_span
            },
        ));
    }

    let paragraph = Paragraph::new(lines)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
