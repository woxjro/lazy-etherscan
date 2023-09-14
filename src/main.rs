mod app;
mod ethers;
mod network;
mod route;
mod ui;
mod widget;
use app::{
    block::SelectableBlockDetailItem, statistics::Statistics,
    transaction::SelectableTransactionDetailItem, App, InputMode,
};
use clap::Parser;
use crossterm::{event, execute, terminal};
use network::{IoEvent, Network};
use ratatui::prelude::*;
use route::{ActiveBlock, Route, RouteId};
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Json-RPC URL
    #[arg(short, long, default_value = "https://eth.llamarpc.com")]
    endpoint: String,
}

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

    let args = Args::parse();
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

async fn start_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &Arc<Mutex<App>>,
) -> Result<(), Box<dyn Error>> {
    let mut is_first_render = true;

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui::ui_home(f, &mut app))?;

        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                event::Event::Key(key) => {
                    if let ActiveBlock::SearchBar = app.route.get_active_block() {
                        match app.input_mode {
                            InputMode::Normal => match key.code {
                                event::KeyCode::Char('i') => {
                                    app.input_mode = InputMode::Editing;
                                }
                                event::KeyCode::Char('q') => {
                                    return Ok(());
                                }
                                event::KeyCode::Char('1') => {
                                    app.change_active_block(ActiveBlock::LatestBlocks);
                                }
                                event::KeyCode::Char('2') => {
                                    app.change_active_block(ActiveBlock::LatestTransactions);
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
                            event::KeyCode::Enter => match app.route.get_active_block() {
                                ActiveBlock::LatestBlocks => {
                                    let latest_blocks = app.latest_blocks.clone();
                                    if let Some(blocks) = latest_blocks {
                                        if let Some(i) = blocks.get_selected_item_index() {
                                            app.set_route(Route::new(
                                                RouteId::Block(Some(blocks.items[i].to_owned())),
                                                ActiveBlock::Main,
                                            ));
                                        }
                                    }
                                }
                                ActiveBlock::LatestTransactions => {
                                    let latest_transactions = app.latest_transactions.clone();
                                    if let Some(transactions) = latest_transactions {
                                        if let Some(i) = transactions.get_selected_item_index() {
                                            app.set_route(Route::new(
                                                RouteId::Transaction(Some(
                                                    transactions.items[i].to_owned(),
                                                )),
                                                ActiveBlock::Main,
                                            ));
                                        }
                                    }
                                }
                                ActiveBlock::Main => match app.route.get_id() {
                                    RouteId::Block(block) => {
                                        if let Some(i) = app.block_detail_list_state.selected() {
                                            match SelectableBlockDetailItem::from(i) {
                                                SelectableBlockDetailItem::Transactions => {
                                                    app.set_route(Route::new(
                                                        RouteId::TransactionsOfBlock(
                                                            block.to_owned(),
                                                        ),
                                                        ActiveBlock::Main,
                                                    ));
                                                }
                                                SelectableBlockDetailItem::Withdrawls => {
                                                    app.set_route(Route::new(
                                                        RouteId::WithdrawalsOfBlock(
                                                            block.to_owned(),
                                                        ),
                                                        ActiveBlock::Main,
                                                    ));
                                                }
                                                SelectableBlockDetailItem::FeeRecipient => {
                                                    if let Some(block) = block.as_ref() {
                                                        if let Some(address) = block.author {
                                                            app.dispatch(IoEvent::GetAddressInfo {
                                                                address,
                                                            });
                                                        }
                                                    }
                                                }
                                                SelectableBlockDetailItem::ParentHash => {
                                                    if let Some(block) = block.as_ref() {
                                                        app.dispatch(IoEvent::GetBlockByHash {
                                                            hash: block.parent_hash,
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    RouteId::TransactionsOfBlock(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if let Some(i) = app.transactions_table_state.selected()
                                            {
                                                if let Some(transaction) =
                                                    block.transactions.iter().nth(i)
                                                {
                                                    app.dispatch(
                                                        IoEvent::GetTransactionWithReceipt {
                                                            transaction_hash: transaction.hash,
                                                        },
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    RouteId::Transaction(transaction) => {
                                        if let Some(i) =
                                            app.transaction_detail_list_state.selected()
                                        {
                                            match SelectableTransactionDetailItem::from(i) {
                                                SelectableTransactionDetailItem::From => {
                                                    if let Some(transaction) = transaction.as_ref()
                                                    {
                                                        app.dispatch(IoEvent::GetAddressInfo {
                                                            address: transaction.transaction.from,
                                                        });
                                                    }
                                                }
                                                SelectableTransactionDetailItem::To => {
                                                    if let Some(transaction) = transaction.as_ref()
                                                    {
                                                        if let Some(address) =
                                                            transaction.transaction.to
                                                        {
                                                            app.dispatch(IoEvent::GetAddressInfo {
                                                                address,
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            event::KeyCode::Char('q') => {
                                return Ok(());
                            }
                            event::KeyCode::Char('s') => {
                                app.change_active_block(ActiveBlock::SearchBar);
                            }
                            event::KeyCode::Char('1') => {
                                app.change_active_block(ActiveBlock::LatestBlocks);
                            }
                            event::KeyCode::Char('2') => {
                                app.change_active_block(ActiveBlock::LatestTransactions);
                            }
                            event::KeyCode::Char('j') => match app.route.get_active_block() {
                                ActiveBlock::LatestBlocks => {
                                    if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                        latest_blocks.next();
                                        let latest_blocks = app.latest_blocks.clone();
                                        if let Some(blocks) = latest_blocks {
                                            if let Some(i) = blocks.get_selected_item_index() {
                                                app.set_route(Route::new(
                                                    RouteId::Block(Some(
                                                        blocks.items[i].to_owned(),
                                                    )),
                                                    ActiveBlock::LatestBlocks,
                                                ));
                                            }
                                        }
                                    }
                                }
                                ActiveBlock::LatestTransactions => {
                                    if let Some(latest_transactions) =
                                        app.latest_transactions.as_mut()
                                    {
                                        latest_transactions.next();
                                        let latest_transactions = app.latest_transactions.clone();
                                        if let Some(transactions) = latest_transactions {
                                            if let Some(i) = transactions.get_selected_item_index()
                                            {
                                                app.set_route(Route::new(
                                                    RouteId::Transaction(Some(
                                                        transactions.items[i].to_owned(),
                                                    )),
                                                    ActiveBlock::LatestTransactions,
                                                ));
                                            }
                                        }
                                    }
                                }
                                ActiveBlock::Main => match app.route.get_id() {
                                    RouteId::Block(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if let Some(i) = app.block_detail_list_state.selected()
                                            {
                                                app.block_detail_list_state.select(Some(
                                                    SelectableBlockDetailItem::from(i)
                                                        .next(block)
                                                        .into(),
                                                ));
                                            } else {
                                                app.block_detail_list_state.select(Some(
                                                    SelectableBlockDetailItem::Transactions.into(),
                                                ));
                                            }
                                        }
                                    }
                                    RouteId::TransactionsOfBlock(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if block.transactions.len() > 0 {
                                                if let Some(i) =
                                                    app.transactions_table_state.selected()
                                                {
                                                    app.transactions_table_state.select(Some(
                                                        (i + 1) % block.transactions.len(),
                                                    ));
                                                } else {
                                                    app.transactions_table_state.select(Some(0));
                                                }
                                            }
                                        }
                                    }
                                    RouteId::WithdrawalsOfBlock(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if let Some(withdrawals) = block.withdrawals.as_ref() {
                                                if let Some(i) =
                                                    app.withdrawals_table_state.selected()
                                                {
                                                    app.withdrawals_table_state
                                                        .select(Some((i + 1) % withdrawals.len()));
                                                } else {
                                                    app.withdrawals_table_state.select(Some(0));
                                                }
                                            }
                                        }
                                    }
                                    RouteId::Transaction(transaction) => {
                                        if let Some(transaction) = transaction.as_ref() {
                                            if let Some(i) =
                                                app.transaction_detail_list_state.selected()
                                            {
                                                app.transaction_detail_list_state.select(Some(
                                                    SelectableTransactionDetailItem::from(i)
                                                        .next(transaction)
                                                        .into(),
                                                ));
                                            } else {
                                                app.transaction_detail_list_state.select(Some(
                                                    SelectableTransactionDetailItem::From.into(),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            event::KeyCode::Char('k') => match app.route.get_active_block() {
                                ActiveBlock::LatestBlocks => {
                                    if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                        latest_blocks.previous();
                                        let latest_blocks = app.latest_blocks.clone();
                                        if let Some(blocks) = latest_blocks {
                                            if let Some(i) = blocks.get_selected_item_index() {
                                                app.set_route(Route::new(
                                                    RouteId::Block(Some(
                                                        blocks.items[i].to_owned(),
                                                    )),
                                                    ActiveBlock::LatestBlocks,
                                                ));
                                            }
                                        }
                                    }
                                }
                                ActiveBlock::LatestTransactions => {
                                    if let Some(latest_transactions) =
                                        app.latest_transactions.as_mut()
                                    {
                                        latest_transactions.previous();
                                        let latest_transactions = app.latest_transactions.clone();
                                        if let Some(transactions) = latest_transactions {
                                            if let Some(i) = transactions.get_selected_item_index()
                                            {
                                                app.set_route(Route::new(
                                                    RouteId::Transaction(Some(
                                                        transactions.items[i].to_owned(),
                                                    )),
                                                    ActiveBlock::LatestTransactions,
                                                ));
                                            }
                                        }
                                    }
                                }
                                ActiveBlock::Main => match app.route.get_id() {
                                    RouteId::Block(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if let Some(i) = app.block_detail_list_state.selected()
                                            {
                                                app.block_detail_list_state.select(Some(
                                                    SelectableBlockDetailItem::from(i)
                                                        .previous(block)
                                                        .into(),
                                                ));
                                            } else {
                                                app.block_detail_list_state.select(Some(
                                                    SelectableBlockDetailItem::Transactions.into(),
                                                ));
                                            }
                                        }
                                    }
                                    RouteId::TransactionsOfBlock(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if block.transactions.len() > 0 {
                                                if let Some(i) =
                                                    app.transactions_table_state.selected()
                                                {
                                                    app.transactions_table_state.select(Some(
                                                        (i + block.transactions.len() - 1)
                                                            % block.transactions.len(),
                                                    ));
                                                } else {
                                                    app.transactions_table_state.select(Some(0));
                                                }
                                            }
                                        }
                                    }
                                    RouteId::WithdrawalsOfBlock(block) => {
                                        if let Some(block) = block.as_ref() {
                                            if let Some(withdrawals) = block.withdrawals.as_ref() {
                                                if let Some(i) =
                                                    app.withdrawals_table_state.selected()
                                                {
                                                    app.withdrawals_table_state.select(Some(
                                                        (i + withdrawals.len() - 1)
                                                            % withdrawals.len(),
                                                    ));
                                                } else {
                                                    app.withdrawals_table_state.select(Some(0));
                                                }
                                            }
                                        }
                                    }
                                    RouteId::Transaction(transaction) => {
                                        if let Some(transaction) = transaction.as_ref() {
                                            if let Some(i) =
                                                app.transaction_detail_list_state.selected()
                                            {
                                                app.transaction_detail_list_state.select(Some(
                                                    SelectableTransactionDetailItem::from(i)
                                                        .previous(transaction)
                                                        .into(),
                                                ));
                                            } else {
                                                app.transaction_detail_list_state.select(Some(
                                                    SelectableTransactionDetailItem::From.into(),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            event::KeyCode::Char('r') => match app.route.get_active_block() {
                                ActiveBlock::LatestBlocks => {
                                    let height = terminal.size().unwrap().height as usize;
                                    app.statistics = Statistics::new();
                                    app.latest_blocks = None;
                                    app.dispatch(IoEvent::GetStatistics);
                                    app.dispatch(IoEvent::GetLatestBlocks {
                                        n: (height - 3 * 4) / 2 - 4,
                                    });
                                }
                                ActiveBlock::LatestTransactions => {
                                    let height = terminal.size().unwrap().height as usize;
                                    app.latest_transactions = None;
                                    app.dispatch(IoEvent::GetLatestTransactions {
                                        n: (height - 3 * 4) / 2 - 4,
                                    });
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                event::Event::Paste(data) => {
                    if let ActiveBlock::SearchBar = app.route.get_active_block() {
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
