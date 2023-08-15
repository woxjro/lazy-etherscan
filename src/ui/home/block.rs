use crate::app::App;
use crate::route::{HomeRoute, Route};
use ethers_core::types::{Block as EBlock, H256};
use ethers_core::utils::{format_ether, format_units};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, block: EBlock<H256>, rect: Rect) {
    let detail_block = Block::default()
        .title(format!("Block #{}", block.number.unwrap()))
        .border_style(if let Route::Home(HomeRoute::Block(_)) = app.route {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        })
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(rect)
        else {
            return;
        };

    let lines = [
        format!("{:<20}: {}", "Block Height", block.number.unwrap()),
        format!("{:<20}: {}", "Block Hash", block.hash.unwrap()),
        format!("{:<20}: {}", "Timestamp", block.time().unwrap().to_string()),
        format!("{:<20}: {}", "Transactions ", block.transactions.len()),
        format!(
            "{:<20}: {} withdrawals in this block",
            "Withdrawals",
            block.withdrawals.unwrap().len()
        ),
        format!(
            "{:<20}: {}",
            "Fee Recipient",
            if let Some(addr) = block.author {
                format!("{addr}")
            } else {
                format!("pending...")
            }
        ),
        format!(
            "{:<20}: {}",
            "Total Difficulty",
            block.total_difficulty.unwrap()
        ),
        format!("{:<20}: {}", "Size", block.size.unwrap()),
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
