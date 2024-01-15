mod block_info;
mod fee_info;
mod gas_info;
mod transactions;
mod withdrawals;
use crate::{
    app::App,
    ethers::types::BlockWithTransactionReceipts,
    route::{ActiveBlock, RouteId},
};

use ethers::core::types::Transaction;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block_with_transaction_receipts: Option<BlockWithTransactionReceipts<Transaction>>,
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

    if let Some(block_with_transaction_receipts) = block_with_transaction_receipts {
        if let RouteId::WithdrawalsOfBlock(_) = app.get_current_route().get_id() {
            withdrawals::render(f, app, &block_with_transaction_receipts, transactions_rect);
        } else {
            let _ =
                transactions::render(f, app, &block_with_transaction_receipts, transactions_rect);
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

        let BlockWithTransactionReceipts {
            block,
            transaction_receipts: _,
        } = block_with_transaction_receipts;

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
