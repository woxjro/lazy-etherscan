use crate::app::App;
use crate::route::{HomeRoute, Route};
use ethers_core::types::{Block as EBlock, Transaction};
use ethers_core::utils::{format_ether, format_units};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let detail_block = Block::default()
        .border_style(if let Route::Home(HomeRoute::Block(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .padding(Padding::horizontal(2))
        .borders(Borders::NONE)
        .border_type(BorderType::Plain);

    let mut lines = vec![
        format!(
            "{:<20}: {}({}%)",
            "Gas Used",
            block.gas_used,
            block.gas_used * 100 / block.gas_limit
        ),
        format!("{:<20}: {}", "Gas Limit", block.gas_limit),
    ];

    // if past London
    if let Some(base_fee_per_gas) = block.base_fee_per_gas {
        lines.push(format!(
            "{:<20}: {} ETH ({} Gwei)",
            "Base Fee Per Gas",
            format_ether(base_fee_per_gas),
            format_units(base_fee_per_gas, "gwei").unwrap()
        ));
    }

    lines.append(&mut vec![
        //format!("{:<20}: {}", "Burnt Fees", TODO),
        //format!("{:<20}: {}", "Extra Data", TODO),
        format!("More Details"),
        format!("{:<20}: {:#x}", "Hash", block.hash.unwrap()),
        format!("{:<20}: {:#x}", "Parent Hash", block.parent_hash),
        format!("{:<20}: {:#x}", "StateRoot", block.state_root),
    ]);

    // if past Shanghai
    if let Some(withdrawals_root) = block.withdrawals_root {
        lines.push(format!(
            "{:<20}: {:#x}",
            "WithdrawalsRoot", withdrawals_root
        ));
    }

    lines.push(format!("{:<20}: {:#x}", "Nonce", block.nonce.unwrap()));

    let lines = lines
        .iter()
        .map(|row| Line::from(Span::raw(row)))
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(lines)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
