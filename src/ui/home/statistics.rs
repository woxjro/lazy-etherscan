use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, _app: &mut App, rect: Rect) {
    let [right_statistics, left_statistics] = *Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Ratio(1,2), Constraint::Ratio(1,2)].as_ref())
            .split(rect)
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
}
