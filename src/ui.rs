use crate::app::{App, Field};
use ratatui::{prelude::*, widgets::*};

/// /home
pub fn ui_home<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    let outer = f.size();

    let [sidebar, detail] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(2,3)].as_ref())
            .split(outer)
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
            .title("Details")
            .border_style(Style::default())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
    };

    f.render_widget(detail_block, detail);
}
