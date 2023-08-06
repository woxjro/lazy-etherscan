use crate::app::App;
use crate::route::Route;
use cfonts::Options;
use chrono::Utc;
use ratatui::{prelude::*, widgets::*};

/// /home
pub fn render_home_layout<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    let outer = f.size();

    let [searchbar, rest] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(0)].as_ref())
            .split(outer)
        else {
            return;
        };

    let searchbar_block = if let Route::Search = app.route {
        Block::default().border_style(Style::default().fg(Color::Green))
    } else {
        Block::default().border_style(Style::default())
    }
    .title("Serach by Address / Txn Hash / Block / Token / Domain Name")
    .borders(Borders::ALL)
    .border_type(BorderType::Plain);

    f.render_widget(searchbar_block, searchbar);

    let [sidebar, detail] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(2,3)].as_ref())
            .split(rest)
        else {
            return;
        };

    let [statistics, latest_status] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(9), Constraint::Min(0)].as_ref())
            .split(sidebar)
        else {
            return;
        };

    let [right_statistics, left_statistics] = *Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Ratio(1,2), Constraint::Ratio(1,2)].as_ref())
            .split(statistics)
        else {
            return;
        };

    let [statistics1, statistics2,statistics3] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(1,3),Constraint::Ratio(1,3) ].as_ref())
            .split(right_statistics)
        else {
            return;
        };

    let [statistics4, statistics5,statistics6] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(1,3),Constraint::Ratio(1,3) ].as_ref())
            .split(left_statistics)
        else {
            return;
        };

    let statistic_items = [
        statistics1,
        statistics2,
        statistics3,
        statistics4,
        statistics5,
        statistics6,
    ];

    let statistic_titles = [
        "ETHER PRICE",
        "TRANSACTIONS",
        "LAST SAFE BLOCK",
        "MARKET CAP",
        "MED GAS PRICE",
        "LAST FINALIZED BLOCK",
    ];

    for (i, statistic_item) in statistic_items.iter().enumerate() {
        let block = Block::default()
            .title(statistic_titles[i])
            .border_style(Style::default())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        f.render_widget(block, statistic_item.to_owned());
    }

    let [top, middle] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Ratio(1,2), Constraint::Ratio(1,2)].as_ref())
            .split(latest_status)
        else {
            return;
        };

    let sidebar_items = [top, middle];

    let mut blocks = (0..(app.sidebar_items.len()))
        .map(|i| {
            Block::default()
                .title(app.sidebar_items[i].to_owned())
                .border_style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
        })
        .collect::<Vec<_>>();

    match app.route {
        Route::Home => {
            blocks[0] = blocks[0]
                .to_owned()
                .border_style(Style::default().fg(Color::Green));
        }
        Route::Blocks => {
            blocks[0] = blocks[0]
                .to_owned()
                .border_style(Style::default().fg(Color::Green));
        }
        Route::Transactions => {
            blocks[1] = blocks[1]
                .to_owned()
                .border_style(Style::default().fg(Color::Green));
        }
        _ => {}
    }

    let blocks = blocks;

    let block_list = if let Some(latest_blocks) = app.latest_blocks.to_owned() {
        let mut res = vec![
            ListItem::new(format!(
                " {:^11} | {:^11} | {:^12} | {:^12} |", //TODO: remove this magic number
                "Block Height", "Hash", "Transactions", "Time"
            )),
            ListItem::new("-".repeat(59)), //TODO: remove this magic number
        ];

        for block in latest_blocks {
            res.push(ListItem::new(format!(
                "{:>13} | {:>12} | {:>7} txns | {:>3} secs ago |", //TODO: remove this magic number
                block.number.unwrap(),
                block.hash.unwrap(),
                block.transactions.len(),
                (Utc::now() - block.time().unwrap()).num_seconds()
            )));
        }
        List::new(res)
    } else {
        List::new([
            ListItem::new(format!(
                " {:^11} | {:^11} | {:^12} | {:^12} |", //TODO: remove this magic number
                "Block Height", "Hash", "Transactions", "Time"
            )),
            ListItem::new("-".repeat(59)), //TODO: remove this magic number
            ListItem::new("is loading..."),
        ])
    }
    .block(blocks[0].to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    f.render_widget(block_list, top);

    let transaction_list = if let Some(latest_transactions) = app.latest_transactions.to_owned() {
        let mut res = vec![];

        for tx in latest_transactions {
            res.push(ListItem::new(format!("{}", tx.hash)));
        }
        List::new(res)
    } else {
        List::new([ListItem::new("is loading...")])
    }
    .block(blocks[1].to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    f.render_widget(transaction_list, middle);

    for i in 0..app.sidebar_items.len() {
        f.render_widget(blocks[i].to_owned(), sidebar_items[i]);
    }

    let welcome_block = Block::default()
        .title("Welcome")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(detail)
        else {
            return;
        };

    let banner = Paragraph::new(Text::from(
        cfonts::render(Options {
            text: String::from("lazy|etherscan"),
            font: cfonts::Fonts::FontBlock,
            ..Options::default()
        })
        .text,
    ))
    .block(welcome_block.to_owned())
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

    f.render_widget(banner, detail_rect);
    f.render_widget(welcome_block, detail);
}
