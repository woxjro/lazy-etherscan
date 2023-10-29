mod app;
mod ethers;
mod network;
mod route;
mod ui;
mod widget;
use app::event_handling::event_handling;
use app::App;
use chrono::Utc;
use clap::Parser;
use crossterm::{event, execute, terminal};
use log::LevelFilter;
use network::{IoEvent, Network};
use ratatui::prelude::*;
use serde::Deserialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs;
use std::io::{BufReader, Read};
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct Settings {
    etherscan: Option<Etherscan>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Etherscan {
    api_key: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Json-RPC URL
    #[arg(short, long, default_value = "https://eth.public-rpc.com")]
    endpoint: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
            std::fs::File::create(format!("logs/{}.log", Utc::now().format("%Y%m%d%H%M"))).unwrap(),
        ),
    ])
    .unwrap();

    let settings: Option<Settings> = match read_file("./settings.toml".to_owned()) {
        Ok(s) => toml::from_str(&s).unwrap_or(None),
        Err(_) => None,
    };

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

    let args = Args::parse();
    std::thread::spawn(move || {
        let mut network = Network::new(
            &app,
            &args.endpoint,
            settings
                .as_ref()
                .map_or(&None, |settings| &settings.etherscan),
        );
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

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

async fn start_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &Arc<Mutex<App>>,
) -> Result<(), Box<dyn Error>> {
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
