use crate::app::App;
use crate::route::ActiveBlock;
use crate::widget::Spinner;
use chrono::Utc;
use ethers_core::utils::format_ether;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: Rect) {
    let [latest_blocks_rect, latest_transactions_rect] = *Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(rect)
    else {
        return;
    };

    let latest_blocks_block = Block::default()
        .title("Latest Blocks")
        .border_style(Style::default().fg(
            if let ActiveBlock::LatestBlocks = app.get_current_route().get_active_block() {
                Color::Green
            } else {
                Color::White
            },
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let latest_transactions_block = Block::default()
        .title("Latest Transactions")
        .border_style(Style::default().fg(
            if let ActiveBlock::LatestTransactions = app.get_current_route().get_active_block() {
                Color::Green
            } else {
                Color::White
            },
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let header = vec![
        ListItem::new(format!(
            " {:^12} | {:^11} | {:^12} | {:^13} |", //TODO: remove these magic numbers
            "Block Height", "Hash", "Transactions", "Time"
        )),
        ListItem::new(format!(
            "{}+{}+{}+{}|",
            "-".repeat(14),
            "-".repeat(13),
            "-".repeat(14),
            "-".repeat(15),
        )), //TODO: remove these magic numbers
    ];
    let block_list = if let Some(latest_blocks) = app.latest_blocks.as_ref() {
        let mut res = header;

        for block in latest_blocks.items.to_owned() {
            res.push(ListItem::new(format!(
                "{:>13} | {:>12} | {:>7} txns | {:>4} secs ago |", //TODO: remove these magic numbers
                block.number.unwrap(),
                block.hash.unwrap(),
                block.transactions.len(),
                (Utc::now() - block.time().unwrap()).num_seconds()
            )));
        }
        List::new(res)
    } else {
        let mut res = header.to_owned();
        res.push(ListItem::new(format!(
            " Loading {}",
            Spinner::default().to_string()
        )));
        List::new(res)
    }
    .block(latest_blocks_block.to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(
        block_list,
        latest_blocks_rect,
        app.latest_blocks
            .as_mut()
            .map_or(&mut ListState::default(), |blocks| &mut blocks.state),
    );

    let header = vec![
        ListItem::new(format!(
            " {:^11} | {:^11} | {:^11} | {:^20} |", //TODO: remove these magic numbers
            "Hash", "From", "To", "Value (ETH)"
        )),
        ListItem::new(format!(
            "{}+{}+{}+{}|",
            "-".repeat(13),
            "-".repeat(13),
            "-".repeat(13),
            "-".repeat(22),
        )),
    ];
    let transaction_list = if let Some(latest_transactions) = app.latest_transactions.as_ref() {
        let mut res = header.to_owned();

        for tx in latest_transactions.items.to_owned() {
            res.push(ListItem::new(format!(
                " {:^11} | {:^11} | {:^11} | {:>19} |",
                tx.transaction.hash,
                tx.transaction.from,
                tx.transaction
                    .to
                    .map_or("".to_owned(), |to| format!("{to}")),
                format_ether(tx.transaction.value)
            )));
        }
        List::new(res)
    } else {
        let mut res = header.to_owned();
        res.push(ListItem::new(format!(
            " Loading {}",
            Spinner::default().to_string()
        )));
        List::new(res)
    }
    .block(latest_transactions_block.to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(
        transaction_list,
        latest_transactions_rect,
        app.latest_transactions
            .as_mut()
            .map_or(&mut ListState::default(), |txns| &mut txns.state),
    );

    f.render_widget(latest_blocks_block, latest_blocks_rect);
    f.render_widget(latest_transactions_block, latest_transactions_rect);
}
