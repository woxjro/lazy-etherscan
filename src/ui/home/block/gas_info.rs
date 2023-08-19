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
        //.title(format!("Block #{}", block.number.unwrap()))
        .border_style(if let Route::Home(HomeRoute::Block(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .padding(Padding::horizontal(2))
        .borders(Borders::NONE)
        .border_type(BorderType::Plain);

    let lines = [
        format!(
            "{:<20}: {}({}%)",
            "Gas Used",
            block.gas_used,
            block.gas_used * 100 / block.gas_limit
        ),
        format!("{:<20}: {}", "Gas Limit", block.gas_limit),
        format!(
            "{:<20}: {} ETH ({} Gwei)",
            "Base Fee Per Gas",
            format_ether(block.base_fee_per_gas.unwrap()),
            format_units(block.base_fee_per_gas.unwrap(), "gwei").unwrap()
        ),
        //format!("{:<20}: {}", "Burnt Fees", TODO),
        //format!("{:<20}: {}", "Extra Data", TODO),
        format!("More Details"),
        format!("{:<20}: {}", "Hash", block.hash.unwrap()),
        format!("{:<20}: {}", "Parent Hash", block.parent_hash),
        format!("{:<20}: {}", "StateRoot", block.state_root),
        format!(
            "{:<20}: {}",
            "WithdrawalsRoot",
            block.withdrawals_root.unwrap()
        ),
        format!("{:<20}: {}", "Nonce", block.nonce.unwrap()),
    ];

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
