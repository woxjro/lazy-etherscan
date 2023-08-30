use crate::app::App;
use crate::route::ActiveBlock;
use ethers_core::types::{Block as EBlock, Transaction};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: &EBlock<Transaction>,
    rect: Rect,
) {
    let detail_block = Block::default()
        .border_style(if let ActiveBlock::Main = app.route.get_active_block() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        })
        .padding(Padding::horizontal(2))
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain);

    let fee_recipient_spans = vec![
        Span::raw(format!("{:<20}: ", "Fee Recipient")).fg(Color::White),
        Span::styled(
            format!(
                "{}",
                if let Some(addr) = block.author {
                    format!("{:#x}", addr)
                } else {
                    format!("pending...")
                }
            ),
            Style::default().fg(Color::Cyan),
        ),
    ];

    let details = vec![
        Line::from(
            if app.block_detail_list_state.selected() == Some(App::BLOCK_DETAIL_FEE_RECIPIENT_INDEX)
            {
                fee_recipient_spans
                    .iter()
                    .map(|span| span.to_owned().add_modifier(Modifier::BOLD))
                    .collect::<Vec<_>>()
            } else {
                fee_recipient_spans
            },
        ),
        //ref: https://docs.alchemy.com/docs/how-to-calculate-ethereum-miner-rewards#calculate-a-miner-reward
        //format!("Block Reward: {} ETH", /* TODO */):
        Line::from(
            Span::raw(format!(
                "{:<20}: {}",
                "Total Difficulty",
                block.total_difficulty.unwrap()
            ))
            .fg(Color::White),
        ),
        Line::from(
            Span::raw(format!("{:<20}: {} bytes", "Size", block.size.unwrap())).fg(Color::White),
        ),
    ];

    let paragraph = Paragraph::new(details)
        .block(detail_block.to_owned())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, rect);
}
