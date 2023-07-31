use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

enum Field {
    LatestBlocks,
    LatestTransactions,
    Bottom,
    Details,
}

#[derive(Debug)]
enum SidebarCategory {
    LatestBlocks,
    LatestTransactions,
    Bottom,
}

impl From<usize> for SidebarCategory {
    fn from(id: usize) -> Self {
        if id == 0 {
            Self::LatestBlocks
        } else if id == 1 {
            Self::LatestTransactions
        } else if id == 2 {
            Self::Bottom
        } else {
            panic!()
        }
    }
}

impl Field {
    fn get_index(&self) -> usize {
        match self {
            Self::LatestBlocks => 0,
            Self::LatestTransactions => 1,
            Self::Bottom => 2,
            Self::Details => 3,
        }
    }
}

struct App<'a> {
    pub sidebar_items: Vec<&'a str>,
    pub focus: usize,
    pub details_about: Option<SidebarCategory>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            sidebar_items: vec!["Latest Blocks", "Latest Transactions", "Bottom"],
            focus: 0,
            details_about: None,
        }
    }

    pub fn next(&mut self) {
        self.focus = (self.focus + 1) % self.sidebar_items.len();
    }

    pub fn set(&mut self, focus: usize) {
        if focus < 3 {
            self.details_about = None;
        } else if focus == 3 {
            self.details_about = Some(SidebarCategory::from(self.focus));
        }
        self.focus = focus;
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
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('1') => app.set(0),
                    KeyCode::Char('2') => app.set(1),
                    KeyCode::Char('3') => app.set(2),
                    KeyCode::Enter => app.set(3),
                    _ => {}
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
