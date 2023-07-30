use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    pub sidebar_items: Vec<&'a str>,
    pub focus: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            sidebar_items: vec!["Top", "Middle", "Bottom"],
            focus: 0,
        }
    }

    pub fn next(&mut self) {
        self.focus = (self.focus + 1) % self.sidebar_items.len();
    }

    pub fn previous(&mut self) {
        if self.focus > 0 {
            self.focus -= 1;
        } else {
            self.focus = self.sidebar_items.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                if let KeyCode::Tab = key.code {
                    app.next()
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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

    let detail_block = Block::default()
        .title("Detail")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    f.render_widget(detail_block, detail);
}
