mod app;
mod network;
mod route;
mod ui;
use app::App;
use crossterm::{event, execute, terminal};
use network::{IoEvent, Network};
use ratatui::prelude::*;
use route::Route;
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    // create app and run it
    let app = Arc::new(Mutex::new(App::new(sync_io_tx)));
    let cloned_app = Arc::clone(&app);

    std::thread::spawn(move || {
        let mut network = Network::new(&app);
        start_tokio(sync_io_rx, &mut network);
    });

    let res = start_ui(&mut terminal, &cloned_app).await;

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

async fn start_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &Arc<Mutex<App>>,
) -> Result<(), Box<dyn Error>> {
    let mut is_first_render = true;

    loop {
        let mut app = app.lock().await;
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

        if is_first_render {
            app.dispatch(IoEvent::GetLatestBlocks);
            app.dispatch(IoEvent::GetLatestTransactions);
            is_first_render = false;
        }
    }
}

#[tokio::main]
async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
