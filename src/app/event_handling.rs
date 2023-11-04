use crate::app::{
    block::SelectableBlockDetailItem, statistics::Statistics,
    transaction::SelectableTransactionDetailItem, App, InputMode,
};
use crate::ethers::types::BlockWithTransactionReceipts;
use crate::network::IoEvent;
use crate::route::{ActiveBlock, Route, RouteId};
use crossterm::event;
use ethers::core::types::NameOrAddress;
use log::debug;
use ratatui::{prelude::*, Terminal};

type IsQ = bool;

pub fn event_handling<B>(event: event::Event, app: &mut App, terminal: &Terminal<B>) -> IsQ
where
    B: Backend,
{
    match event {
        event::Event::Key(key) => {
            debug!("{:?}", key.code);
            if let ActiveBlock::SearchBar = app.get_current_route().get_active_block() {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        event::KeyCode::Char('e') => {
                            if key.modifiers == event::KeyModifiers::CONTROL {
                                app.is_toggled = !app.is_toggled;
                            }
                        }
                        event::KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        }
                        event::KeyCode::Char('q') => {
                            return true;
                        }
                        event::KeyCode::Char('1') => {
                            app.change_active_block(ActiveBlock::LatestBlocks);
                        }
                        event::KeyCode::Char('2') => {
                            app.change_active_block(ActiveBlock::LatestTransactions);
                        }
                        event::KeyCode::Char('p') => {
                            if key.modifiers == event::KeyModifiers::CONTROL {
                                app.pop_current_route();
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == event::KeyEventKind::Press => {
                        match key.code {
                            event::KeyCode::Enter => {
                                app.submit_message();
                                app.input_mode = InputMode::Normal;
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
                    event::KeyCode::Enter => match app.get_current_route().get_active_block() {
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
                        ActiveBlock::Main => match app.get_current_route().get_id() {
                            RouteId::Block(block) => {
                                if let Some(i) = app.block_detail_list_state.selected() {
                                    match SelectableBlockDetailItem::from(i) {
                                        SelectableBlockDetailItem::Transactions => {
                                            app.set_route(Route::new(
                                                RouteId::TransactionsOfBlock(block.to_owned()),
                                                ActiveBlock::Main,
                                            ));
                                        }
                                        SelectableBlockDetailItem::Withdrawls => {
                                            app.set_route(Route::new(
                                                RouteId::WithdrawalsOfBlock(block.to_owned()),
                                                ActiveBlock::Main,
                                            ));
                                        }
                                        SelectableBlockDetailItem::FeeRecipient => {
                                            if let Some(BlockWithTransactionReceipts {
                                                block,
                                                transaction_receipts: _,
                                            }) = block.as_ref()
                                            {
                                                if let Some(address) = block.author {
                                                    app.dispatch(IoEvent::GetNameOrAddressInfo {
                                                        name_or_address: NameOrAddress::Address(
                                                            address,
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        SelectableBlockDetailItem::ParentHash => {
                                            if let Some(BlockWithTransactionReceipts {
                                                block,
                                                transaction_receipts: _,
                                            }) = block.as_ref()
                                            {
                                                app.dispatch(IoEvent::GetBlockByHash {
                                                    hash: block.parent_hash,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            RouteId::TransactionsOfBlock(block) => {
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if let Some(i) = app.transactions_table_state.selected() {
                                        if let Some(transaction) = block.transactions.get(i) {
                                            app.dispatch(IoEvent::GetTransactionWithReceipt {
                                                transaction_hash: transaction.hash,
                                            });
                                        }
                                    }
                                }
                            }
                            RouteId::Transaction(transaction) => {
                                if let Some(i) = app.transaction_detail_list_state.selected() {
                                    match SelectableTransactionDetailItem::from(i) {
                                        SelectableTransactionDetailItem::From => {
                                            if let Some(transaction) = transaction.as_ref() {
                                                app.dispatch(IoEvent::GetNameOrAddressInfo {
                                                    name_or_address: NameOrAddress::Address(
                                                        transaction.transaction.from,
                                                    ),
                                                });
                                            }
                                        }
                                        SelectableTransactionDetailItem::To => {
                                            if let Some(transaction) = transaction.as_ref() {
                                                if let Some(address) = transaction.transaction.to {
                                                    app.dispatch(IoEvent::GetNameOrAddressInfo {
                                                        name_or_address: NameOrAddress::Address(
                                                            address,
                                                        ),
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
                    event::KeyCode::Char('e') => {
                        if key.modifiers == event::KeyModifiers::CONTROL {
                            match app.get_current_route().get_active_block() {
                                ActiveBlock::LatestBlocks | ActiveBlock::LatestTransactions => {
                                    app.change_active_block(ActiveBlock::Main);
                                }
                                _ => {}
                            }

                            app.is_toggled = !app.is_toggled;
                        }
                    }
                    event::KeyCode::Char('p') => {
                        if key.modifiers == event::KeyModifiers::CONTROL {
                            app.pop_current_route();
                        }
                    }
                    event::KeyCode::Char('q') => {
                        return true;
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
                    event::KeyCode::Char('j') => match app.get_current_route().get_active_block() {
                        ActiveBlock::LatestBlocks => {
                            if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                latest_blocks.next();
                                let latest_blocks = app.latest_blocks.clone();
                                if let Some(blocks) = latest_blocks {
                                    if let Some(i) = blocks.get_selected_item_index() {
                                        app.set_route(Route::new(
                                            RouteId::Block(Some(blocks.items[i].to_owned())),
                                            ActiveBlock::LatestBlocks,
                                        ));
                                    }
                                }
                            }
                        }
                        ActiveBlock::LatestTransactions => {
                            if let Some(latest_transactions) = app.latest_transactions.as_mut() {
                                latest_transactions.next();
                                let latest_transactions = app.latest_transactions.clone();
                                if let Some(transactions) = latest_transactions {
                                    if let Some(i) = transactions.get_selected_item_index() {
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
                        ActiveBlock::Main => match app.get_current_route().get_id() {
                            RouteId::Block(block) => {
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if let Some(i) = app.block_detail_list_state.selected() {
                                        app.block_detail_list_state.select(Some(
                                            SelectableBlockDetailItem::from(i).next(block).into(),
                                        ));
                                    } else {
                                        app.block_detail_list_state.select(Some(
                                            SelectableBlockDetailItem::Transactions.into(),
                                        ));
                                    }
                                }
                            }
                            RouteId::TransactionsOfBlock(block) => {
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if !block.transactions.is_empty() {
                                        if let Some(i) = app.transactions_table_state.selected() {
                                            app.transactions_table_state
                                                .select(Some((i + 1) % block.transactions.len()));
                                        } else {
                                            app.transactions_table_state.select(Some(0));
                                        }
                                    }
                                }
                            }
                            RouteId::WithdrawalsOfBlock(block) => {
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if let Some(withdrawals) = block.withdrawals.as_ref() {
                                        if let Some(i) = app.withdrawals_table_state.selected() {
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
                                    if let Some(i) = app.transaction_detail_list_state.selected() {
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
                    event::KeyCode::Char('k') => match app.get_current_route().get_active_block() {
                        ActiveBlock::LatestBlocks => {
                            if let Some(latest_blocks) = app.latest_blocks.as_mut() {
                                latest_blocks.previous();
                                let latest_blocks = app.latest_blocks.clone();
                                if let Some(blocks) = latest_blocks {
                                    if let Some(i) = blocks.get_selected_item_index() {
                                        app.set_route(Route::new(
                                            RouteId::Block(Some(blocks.items[i].to_owned())),
                                            ActiveBlock::LatestBlocks,
                                        ));
                                    }
                                }
                            }
                        }
                        ActiveBlock::LatestTransactions => {
                            if let Some(latest_transactions) = app.latest_transactions.as_mut() {
                                latest_transactions.previous();
                                let latest_transactions = app.latest_transactions.clone();
                                if let Some(transactions) = latest_transactions {
                                    if let Some(i) = transactions.get_selected_item_index() {
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
                        ActiveBlock::Main => match app.get_current_route().get_id() {
                            RouteId::Block(block) => {
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if let Some(i) = app.block_detail_list_state.selected() {
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
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if !block.transactions.is_empty() {
                                        if let Some(i) = app.transactions_table_state.selected() {
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
                                if let Some(BlockWithTransactionReceipts {
                                    block,
                                    transaction_receipts: _,
                                }) = block.as_ref()
                                {
                                    if let Some(withdrawals) = block.withdrawals.as_ref() {
                                        if let Some(i) = app.withdrawals_table_state.selected() {
                                            app.withdrawals_table_state.select(Some(
                                                (i + withdrawals.len() - 1) % withdrawals.len(),
                                            ));
                                        } else {
                                            app.withdrawals_table_state.select(Some(0));
                                        }
                                    }
                                }
                            }
                            RouteId::Transaction(transaction) => {
                                if let Some(transaction) = transaction.as_ref() {
                                    if let Some(i) = app.transaction_detail_list_state.selected() {
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
                    event::KeyCode::Char('r') => match app.get_current_route().get_active_block() {
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
            if let ActiveBlock::SearchBar = app.get_current_route().get_active_block() {
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
    false
}
