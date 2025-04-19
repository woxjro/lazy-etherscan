mod app;
mod ethers;
mod network;
mod route;
mod ui;
mod widget;
use anyhow::Result;
use app::{event_handling::event_handling, App};
use chrono::Utc;
use clap::Parser;
use crossterm::{event, execute, terminal};
use log::LevelFilter;
use network::{IoEvent, Network};
use ratatui::prelude::*;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{io, sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Json-RPC URL
    #[arg(short, long, default_value = "https://eth.llamarpc.com")]
    endpoint: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = std::fs::create_dir("logs");
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create(format!("logs/{}.log", Utc::now().format("%Y%m%d%H%M")))?,
        ),
    ])?;

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

    let args = Args::parse();

    // create app and run it
    let app = Arc::new(Mutex::new(App::new(sync_io_tx, &args.endpoint)));
    let cloned_app = Arc::clone(&app);

    std::thread::spawn(move || {
        let mut network = Network::new(&app, &args.endpoint);
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

async fn start_ui<B: Backend>(terminal: &mut Terminal<B>, app: &Arc<Mutex<App>>) -> Result<()> {
    let mut is_first_render = true;

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui::ui_home(f, &mut app))?;

        if event::poll(Duration::from_millis(250))? {
            let is_q = event_handling(event::read()?, &mut app, terminal);
            if is_q {
                return Ok(());
            }
        }

        if is_first_render {
            let height = terminal.size()?.height as usize;
            app.dispatch(IoEvent::InitialSetup {
                n: (height - 3 * 4) / 2 - 4,
            });

            is_first_render = false;
        }
    }
}

#[tokio::main]
async fn start_tokio(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        let _ = network.handle_network_event(io_event).await;
    }
}
