mod app;
mod ethers;
mod network;
mod route;
mod ui;
mod widget;
use app::{App, InputMode};
use crossterm::{event, execute, terminal};
use network::{IoEvent, Network};
use ratatui::prelude::*;
use route::{HomeRoute, Route};
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;

const ENDPOINT: &'static str = "https://eth.llamarpc.com";

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
        let mut network = Network::new(&app, ENDPOINT);
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
        match app.route.to_owned() {
            Route::Home(_) => {
                terminal.draw(|f| ui::ui_home(f, &mut app))?;
            }
        };

        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                event::Event::Key(key) => {
                    if let Route::Home(HomeRoute::Search) = app.route {
                        match app.input_mode {
                            InputMode::Normal => match key.code {
                                event::KeyCode::Char('i') => {
                                    app.input_mode = InputMode::Editing;
                                }
                                event::KeyCode::Char('q') => {
                                    return Ok(());
                                }
                                event::KeyCode::Char('1') => {
                                    app.set_route(Route::Home(HomeRoute::LatestBlocks))
                                }
                                event::KeyCode::Char('2') => {
                                    app.set_route(Route::Home(HomeRoute::LatestTransactions))
                                }
                                _ => {}
                            },
                            InputMode::Editing if key.kind == event::KeyEventKind::Press => {
                                match key.code {
                                    event::KeyCode::Enter => {
                                        app.submit_message();
                                    }
                                    event::KeyCode::Char(to_insert) => {
                                        app.enter_char(to_insert);
                                    }
                                    event::KeyCode::Backspace => {
                                        app.delete_char();
                                    }
                                    event::KeyCode::Left => {
                                        app.move_cursor_left();
                                    }
                                    event::KeyCode::Right => {
                                        app.move_cursor_right();
                                    }
                                    event::KeyCode::Esc => {
                                        app.input_mode = InputMode::Normal;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            event::KeyCode::Enter => match app.route {
                                Route::Home(HomeRoute::LatestBlocks) => {
                                    let latest_blocks = app.latest_blocks.clone();
                                    if let Some(blocks) = latest_blocks {
                                        if let Some(i) = blocks.get_selected_item_index() {
                                            app.set_route(Route::Home(HomeRoute::Block(
                                                blocks.items[i].to_owned(),
                                            )));
                                        }
                                    }
                                }
                                Route::Home(HomeRoute::LatestTransactions) => {
                                    let latest_transactions = app.latest_transactions.clone();
                                    if let Some(transactions) = latest_transactions {
                                        if let Some(i) = transactions.get_selected_item_index() {
                                            app.set_route(Route::Home(HomeRoute::Transaction(
                                                transactions.items[i].to_owned(),
                                            )));
                                        }
                                    }
                                }
                                _ => {}
                            },
                            event::KeyCode::Char('q') => {
                                return Ok(());
                            }
                            event::KeyCode::Char('s') => {
                                app.set_route(Route::Home(HomeRoute::Search))
                            }
                            event::KeyCode::Char('1') => {
                                app.set_route(Route::Home(HomeRoute::LatestBlocks))
                            }
                            event::KeyCode::Char('2') => {
                                app.set_route(Route::Home(HomeRoute::LatestTransactions))
                            }
                            event::KeyCode::Char('j') => match app.route {
                                Route::Home(HomeRoute::Root)
                                | Route::Home(HomeRoute::LatestBlocks) => {
                                    if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                        latest_blocks.next();
                                    }
                                }
                                Route::Home(HomeRoute::LatestTransactions) => {
                                    if let Some(latest_transactions) =
                                        app.latest_transactions.as_mut()
                                    {
                                        latest_transactions.next();
                                    }
                                }
                                _ => {}
                            },
                            event::KeyCode::Char('k') => match app.route {
                                Route::Home(HomeRoute::Root)
                                | Route::Home(HomeRoute::LatestBlocks) => {
                                    if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                        latest_blocks.previous();
                                    }
                                }
                                Route::Home(HomeRoute::LatestTransactions) => {
                                    if let Some(latest_transactions) =
                                        app.latest_transactions.as_mut()
                                    {
                                        latest_transactions.previous();
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                event::Event::Paste(data) => {
                    if let Route::Home(HomeRoute::Search) = app.route {
                        match app.input_mode {
                            InputMode::Normal => {}
                            InputMode::Editing => {
                                app.paste(data);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if is_first_render {
            app.dispatch(IoEvent::GetStatistics);

            let height = terminal.size().unwrap().height as usize;
            app.dispatch(IoEvent::GetLatestBlocks {
                n: (height - 3 * 4) / 2 - 4,
            });

            app.dispatch(IoEvent::GetLatestTransactions {
                n: (height - 3 * 4) / 2 - 4,
            });

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
