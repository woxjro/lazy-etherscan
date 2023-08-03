mod app;
mod route;
mod ui;
use app::App;
use route::Route;

use crossterm::{event, execute, terminal};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
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
        match app.route {
            Route::Home => {
                terminal.draw(|f| ui::ui_home(f, &app))?;
            }
            Route::Search => {
                terminal.draw(|f| ui::ui_search(f, &app))?;
            }
            Route::Blocks => {
                terminal.draw(|f| ui::ui_blocks(f, &app))?;
            }
            Route::Transactions => {
                terminal.draw(|f| ui::ui_transations(f, &app))?;
            }
        };

        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => {
                        return Ok(());
                    }
                    event::KeyCode::Char('s') => app.set_route(Route::Search),
                    event::KeyCode::Char('1') => app.set_route(Route::Blocks),
                    event::KeyCode::Char('2') => app.set_route(Route::Transactions),
                    //event::KeyCode::Char('3') => app.set(2),
                    //event::KeyCode::Enter => app.set(3),
                    _ => {}
                }
            }
        }
    }
}
