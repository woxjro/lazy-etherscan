mod block_info;
mod fee_info;
mod gas_info;
mod transactions;
mod withdrawals;
use crate::app::App;
use crate::route::ActiveBlock;
use crate::route::RouteId;
use ethers::core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: Option<EBlock<Transaction>>,
    rect: Rect,
) {
    let height = rect.height;
    let [detail_rect, transactions_rect] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((height - 12) / 2 + 10),
                Constraint::Length((height - 12) / 2 + 2),
            ]
            .as_ref(),
        )
        .split(rect)
    else {
        return;
    };

    if let Some(block) = block {
        if let RouteId::WithdrawalsOfBlock(_) = app.get_current_route().get_id() {
            withdrawals::render(f, app, &block, transactions_rect);
        } else {
            transactions::render(f, app, &block, transactions_rect);
        }

        let [block_info_rect, fee_info_rect, gas_info_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(5, 16),
                    Constraint::Ratio(4, 16),
                    Constraint::Ratio(6, 16),
                ]
                .as_ref(),
            )
            .split(detail_rect)
        else {
            return;
        };

        let detail_block = Block::default()
            .title(format!("Block #{}", block.number.unwrap()))
            .border_style(
                if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            )
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        block_info::render(f, app, &block, block_info_rect);
        fee_info::render(f, app, &block, fee_info_rect);
        gas_info::render(f, app, &block, gas_info_rect);

        f.render_widget(detail_block, rect);
    } else {
        let detail_block = Block::default()
            .title("Block Not Found")
            .border_style(
                if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            )
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        f.render_widget(detail_block, rect);
    }
}
