use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    let outer = f.size();
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from(" lazy-etherscan ").alignment(Alignment::Center))
        .border_type(BorderType::Rounded);
    let inner = outer_block.inner(outer);

    let [sidebar, detail] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1,3), Constraint::Ratio(2,3)].as_ref())
        .split(inner)
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

    let detail_block = Block::default()
        .title("Detail")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let top_block = Block::default()
        .title("Top")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let middle_block = Block::default()
        .title("Middle")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let bottom_block = Block::default()
        .title("Bottom")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    f.render_widget(top_block, top);
    f.render_widget(middle_block, middle);
    f.render_widget(bottom_block, bottom);
    f.render_widget(detail_block, detail);
    f.render_widget(outer_block, outer);
}
