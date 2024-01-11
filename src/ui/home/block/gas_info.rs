use crate::{
    app::{block::SelectableBlockDetailItem, App},
    route::{ActiveBlock, RouteId},
};
use ethers::core::{
    types::{Block as EBlock, Transaction},
    utils::{format_ether, format_units},
};
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
        .padding(Padding::horizontal(2))
        .borders(Borders::NONE)
        .border_type(BorderType::Plain);

    let mut details = vec![
        Line::from(
            Span::raw(format!(
                "{:<20}: {}({}%)",
                "Gas Used",
                block.gas_used,
                block.gas_used * 100 / block.gas_limit
            ))
            .fg(Color::White),
        ),
        Line::from(Span::raw(format!("{:<20}: {}", "Gas Limit", block.gas_limit)).fg(Color::White)),
    ];

    // if past London
    if let Some(base_fee_per_gas) = block.base_fee_per_gas {
        details.push(Line::from(
            Span::raw(format!(
                "{:<20}: {} ETH ({} Gwei)",
                "Base Fee Per Gas",
                format_ether(base_fee_per_gas),
                format_units(base_fee_per_gas, "gwei").unwrap()
            ))
            .fg(Color::White),
        ));
    }

    let parent_hash_spans = vec![
        Span::raw(format!("{:<20}: ", "Parent Hash")).fg(Color::White),
        Span::styled(
            format!("{:#x}", block.parent_hash),
            Style::default().fg(Color::Cyan),
        ),
    ];
    details.append(&mut vec![
        //format!("{:<20}: {}", "Burnt Fees", TODO),
        //format!("{:<20}: {}", "Extra Data", TODO),
        Line::from(Span::raw("More Details".to_string()).fg(Color::White)),
        Line::from(
            Span::raw(format!("{:<20}: {:#x}", "Hash", block.hash.unwrap())).fg(Color::White),
        ),
        Line::from(
            if app.block_detail_list_state.selected()
                == Some(SelectableBlockDetailItem::ParentHash.into())
            {
                parent_hash_spans
                    .iter()
                    .map(|span| span.to_owned().add_modifier(Modifier::BOLD))
                    .collect::<Vec<_>>()
            } else {
                parent_hash_spans
            },
        ),
        Line::from(
            Span::raw(format!("{:<20}: {:#x}", "StateRoot", block.state_root)).fg(Color::White),
        ),
    ]);

    // if past Shanghai
    if let Some(withdrawals_root) = block.withdrawals_root {
        details.push(Line::from(
            Span::raw(format!(
                "{:<20}: {:#x}",
                "WithdrawalsRoot", withdrawals_root
            ))
            .fg(Color::White),
        ));
    }

    details.push(Line::from(
        Span::raw(format!("{:<20}: {:#x}", "Nonce", block.nonce.unwrap())).fg(Color::White),
    ));

    let paragraph = Paragraph::new(details)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
