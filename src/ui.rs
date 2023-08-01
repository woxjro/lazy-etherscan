use crate::app::{App, Field};
use cfonts::Options;
use ratatui::{prelude::*, widgets::*};

/// /home
pub fn ui_home<B: Backend>(f: &mut Frame<B>, app: &App) {
    //println!("{}", title.text);

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

    let searchbar_block = Block::default()
        .title("Serach by Address/ Txn Hash / Block / Token / Domain Name")
        .border_style(Style::default())
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
    let [top, middle, bottom] = *Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(1,3), Constraint::Ratio(1,3)].as_ref())
            .split(sidebar)
        else {
            return;
        };

    let sidebar_items = [top, middle, bottom];

    let blocks = (0..(app.sidebar_items.len()))
        .map(|i| {
            if app.focus == i {
                Block::default()
                    .title(app.sidebar_items[i])
                    .border_style(Style::default().fg(Color::Green))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
            } else {
                Block::default()
                    .title(app.sidebar_items[i])
                    .border_style(Style::default())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
            }
        })
        .collect::<Vec<_>>();

    let block_list = List::new([
        ListItem::new("Block 1"),
        ListItem::new("Block 2"),
        ListItem::new("Block 3"),
        ListItem::new("Block 4"),
        ListItem::new("Block 5"),
        ListItem::new("Block 6"),
    ])
    .block(blocks[0].to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    f.render_widget(block_list, top);

    let transaction_list = List::new([
        ListItem::new("Transaction 1"),
        ListItem::new("Transaction 2"),
        ListItem::new("Transaction 3"),
        ListItem::new("Transaction 4"),
        ListItem::new("Transaction 5"),
        ListItem::new("Transaction 6"),
    ])
    .block(blocks[1].to_owned())
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    f.render_widget(transaction_list, middle);

    for i in 0..(app.sidebar_items.len()) {
        f.render_widget(blocks[i].to_owned(), sidebar_items[i]);
    }

    let detail_block = if app.focus == Field::Details.get_index() {
        Block::default()
            .title(format!(
                "Details about {:?}",
                app.details_about.as_ref().unwrap()
            ))
            .border_style(Style::default().fg(Color::Green))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
    } else {
        Block::default()
            .title("Welcome")
            .border_style(Style::default())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
    };

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(detail)
        else {
            return;
        };

    let paragraph = Paragraph::new(Text::from(
        cfonts::render(Options {
            text: String::from("lazy|etherscan"),
            font: cfonts::Fonts::FontBlock,
            ..Options::default()
        })
        .text,
    ))
    .block(detail_block.to_owned())
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

    f.render_widget(paragraph, detail_rect);
    f.render_widget(detail_block, detail);
}
