mod address_info;
mod block;
mod latest_status;
mod searching;
mod statistics;
mod transaction;
mod welcome;
use crate::{
    app::{App, InputMode},
    route::{ActiveBlock, RouteId},
};
use ratatui::{prelude::*, widgets::*};

/// /home
pub fn render_home_layout<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    let outer = f.size();

    let [searchbar, rest, navigation_bar] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(0), Constraint::Max(1)].as_ref())
        .split(outer)
    else {
        return;
    };

    let searchbar_block = Block::default()
        .border_style(Style::default().fg(
            if let ActiveBlock::SearchBar = app.get_current_route().get_active_block() {
                Color::Green
            } else {
                Color::White
            },
        ))
        .title(format!(
            "Search by Address / Txn Hash / Block / Token / Domain Name ({})",
            match app.input_mode {
                InputMode::Normal => "Press 'q' to exit, 'i' to start editing.",
                InputMode::Editing => "Press 'Esc' to stop editing, 'Enter' to search.",
            }
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::White))
        .block(searchbar_block);
    f.render_widget(input, searchbar);

    let message = Paragraph::new(" <up>/<down>: k/j, <esc>: Cancel, q: Quit, ?: Keybindings, 1-2: Jump to panel, s: Focus on the Search bar")
        .style(Style::default().fg(Color::White));
    f.render_widget(message, navigation_bar);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                searchbar.x + app.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                searchbar.y + 1,
            )
        }
    }

    if app.is_toggled {
        match app.get_current_route().get_id() {
            RouteId::AddressInfo(address_info) => {
                address_info::render(f, app, address_info, rest);
            }
            RouteId::Block(block_with_transaction_receipts) => {
                block::render(f, app, block_with_transaction_receipts, rest);
            }
            RouteId::TransactionsOfBlock(block_with_transaction_receipts) => {
                block::render(f, app, block_with_transaction_receipts, rest);
            }
            RouteId::WithdrawalsOfBlock(block_with_transaction_receipts) => {
                block::render(f, app, block_with_transaction_receipts, rest);
            }
            RouteId::Transaction(transaction) | RouteId::InputDataOfTransaction(transaction) => {
                transaction::render(f, app, transaction, rest);
            }
            RouteId::Welcome => {
                welcome::render(f, app, rest);
            }
            RouteId::Searching(message) => {
                searching::render(f, &message, rest);
            }
        }
    } else {
        let [sidebar, detail] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
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

        statistics::render(f, app, statistics);
        latest_status::render(f, app, latest_status);

        match app.get_current_route().get_id() {
            RouteId::AddressInfo(address_info) => {
                address_info::render(f, app, address_info, detail);
            }
            RouteId::Block(block) => {
                block::render(f, app, block, detail);
            }
            RouteId::TransactionsOfBlock(block) => {
                block::render(f, app, block, detail);
            }
            RouteId::WithdrawalsOfBlock(block) => {
                block::render(f, app, block, detail);
            }
            RouteId::Transaction(transaction) | RouteId::InputDataOfTransaction(transaction) => {
                transaction::render(f, app, transaction, detail);
            }
            RouteId::Welcome => {
                welcome::render(f, app, detail);
            }
            RouteId::Searching(message) => {
                searching::render(f, &message, detail);
            }
        }
    }

    let size = f.size();
    if app.show_popup {
        let block = Block::default()
            .title("Keybindings - Press Esc to close the popup")
            .borders(Borders::ALL);

        //TODO: Enter
        let input = Paragraph::new(vec![
            Line::from(Span::raw(format!(" {:<4}: {}", "j", "Down")).fg(Color::White)),
            Line::from(Span::raw(format!(" {:<4}: {}", "k", "Up")).fg(Color::White)),
            Line::from(
                Span::raw(format!(" {:<4}: {}", "s", "Move to the Search Bar")).fg(Color::White),
            ),
            Line::from(
                Span::raw(format!(" {:<4}: {}", "1", "Move to the Latest Blocks")).fg(Color::White),
            ),
            Line::from(
                Span::raw(format!(
                    " {:<4}: {}",
                    "2", "Move to the Latest Transactions"
                ))
                .fg(Color::White),
            ),
        ])
        .style(Style::default().fg(Color::Green))
        .block(block.to_owned());

        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
        f.render_widget(input, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
