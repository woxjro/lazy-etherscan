use crate::{
    app::App,
    ethers::types::{BlockWithTransactionReceipts, ERC20Token},
    route::{ActiveBlock, RouteId},
    widget::Spinner,
};
use anyhow::Result;
use ethers::core::{
    types::{Transaction, TransactionReceipt, U64},
    utils::{format_ether, format_units},
};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block_with_transaction_receipts: &BlockWithTransactionReceipts<Transaction>,
    rect: Rect,
) -> Result<()> {
    let BlockWithTransactionReceipts {
        block,
        transaction_receipts,
    } = block_with_transaction_receipts;

    let selected_style = Style::default().add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);
    let header = if app.is_toggled {
        vec![
            "",
            "Hash",
            "Method",
            "Type",
            "From",
            "To",
            "Value (ETH)",
            "Fee",
            "Gas Price (Gwei)",
            "Gas Used",
            "Status",
            "#(Log)",
        ]
    } else {
        vec![
            "",
            "Hash",
            "Method",
            "Type",
            "From",
            "To",
            "Value (ETH)",
            //"Fee",
            "Gas Price (Gwei)",
            //"Gas Used",
            //"Status",
            //"#(Log)",
        ]
    };

    let header_cells = header
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let items = block
        .transactions
        .iter()
        .enumerate()
        .map(|(i, tx)| {
            if let Some(transaction_receipts) = transaction_receipts {
                create_row(
                    i,
                    tx,
                    app,
                    transaction_receipts
                        .iter()
                        .find(|receipt| receipt.transaction_hash == tx.hash),
                )
            } else {
                create_row(i, tx, app, None)
            }
        })
        .collect::<Vec<_>>();

    let rows = items
        .iter()
        .map(|cells| Row::new(cells.to_owned()).height(1).bottom_margin(1));

    let widths = if app.is_toggled {
        vec![
            Constraint::Max(4),
            Constraint::Max(12), //Hash
            Constraint::Max(18), //Method
            Constraint::Max(10), //Type
            Constraint::Max(12), //From
            Constraint::Max(12), //To
            Constraint::Max(20), //Value (ETH)
            Constraint::Max(10), //Fee
            Constraint::Max(20), //Gas Price (Gwei)
            Constraint::Max(10), //Gas Used
            Constraint::Max(10), //Status
            Constraint::Max(10), //#(Log)
        ]
    } else {
        vec![
            Constraint::Max(4),
            Constraint::Max(12), //Hash
            Constraint::Max(18), //Method
            Constraint::Max(10), //Type
            Constraint::Max(12), //From
            Constraint::Max(12), //To
            Constraint::Max(20), //Value (ETH)
            Constraint::Max(20), //Gas Price (Gwei)
        ]
    };

    let t = Table::new(rows.to_owned())
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Transactions")
                .fg(
                    if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                        if let RouteId::TransactionsOfBlock(_) = app.get_current_route().get_id() {
                            Color::Green
                        } else {
                            Color::White
                        }
                    } else {
                        Color::White
                    },
                ),
        )
        .highlight_style(selected_style)
        .widths(&widths);

    f.render_stateful_widget(t, rect, &mut app.transactions_table_state);
    Ok(())
}

fn create_row<'a>(
    i: usize,
    tx: &Transaction,
    app: &App,
    transaction_receipt: Option<&TransactionReceipt>,
) -> Vec<Cell<'a>> {
    let mut row = vec![
        Cell::from(format!(" {} ", i + 1)).fg(Color::White),
        Cell::from(format!("{}", tx.hash)).fg(Color::White),
        if tx.to.is_some() {
            if tx.input.len() >= 4 {
                Cell::from("ContractExecution").fg(Color::LightYellow)
            } else {
                Cell::from("Transfer").fg(Color::LightMagenta)
            }
        } else {
            Cell::from("ContractDeployment").fg(Color::LightCyan)
        },
        Cell::from(
            (match tx.transaction_type {
                Some(i) => {
                    if i == U64::from(1) {
                        "AccessList"
                    } else if i == U64::from(2) {
                        "EIP-1559"
                    } else {
                        "Unknown"
                    }
                }
                None => "Legacy",
            })
            .to_string(),
        )
        .fg(Color::White),
        Cell::from(
            if let Some(token) = ERC20Token::find_by_address(&app.erc20_tokens, tx.from) {
                token.ticker.to_string()
            } else if let Some(ens_id) = app.address2ens_id.get(&tx.from) {
                ens_id
                    .as_ref()
                    .map_or(format!("{}", tx.from), |ens_id| ens_id.to_owned())
            } else {
                format!("{}", tx.from)
            },
        )
        .fg(
            if ERC20Token::find_by_address(&app.erc20_tokens, tx.from).is_some() {
                Color::Cyan
            } else if let Some(ens_id) = app.address2ens_id.get(&tx.from) {
                if ens_id.is_some() {
                    Color::Cyan
                } else {
                    Color::White
                }
            } else {
                Color::White
            },
        ),
        Cell::from(tx.to.map_or("".to_owned(), |to| {
            if let Some(token) = ERC20Token::find_by_address(&app.erc20_tokens, to) {
                token.ticker.to_string()
            } else if let Some(ens_id) = app.address2ens_id.get(&to) {
                ens_id
                    .as_ref()
                    .map_or(format!("{to}"), |ens_id| ens_id.to_owned())
            } else {
                format!("{to}")
            }
        }))
        .fg(tx.to.map_or(Color::White, |to| {
            if ERC20Token::find_by_address(&app.erc20_tokens, to).is_some() {
                Color::Cyan
            } else if let Some(ens_id) = app.address2ens_id.get(&to) {
                if ens_id.is_some() {
                    Color::Cyan
                } else {
                    Color::White
                }
            } else {
                Color::White
            }
        })),
        Cell::from(format_ether(tx.value).to_string()).fg(Color::White),
    ];

    if app.is_toggled {
        row.push(
            Cell::from(if let Some(transaction_receipt) = transaction_receipt {
                transaction_receipt
                    .gas_used
                    .map_or(Spinner::default().to_string(), |gas_used| {
                        format_ether(tx.gas_price.unwrap() * gas_used)
                    })
            } else {
                Spinner::default().to_string()
            })
            .fg(Color::White),
        );
    }

    row.push(
        Cell::from(
            format_units(tx.gas_price.unwrap(), "gwei")
                .unwrap()
                .to_string(),
        )
        .fg(Color::White),
    );

    if app.is_toggled {
        row.append(&mut vec![
            Cell::from(if let Some(transaction_receipt) = transaction_receipt {
                transaction_receipt
                    .gas_used
                    .map_or("".to_string(), |gas_used| {
                        format_units(gas_used, "gwei").unwrap().to_string()
                    })
            } else {
                Spinner::default().to_string()
            })
            .fg(Color::White),
            if let Some(transaction_receipt) = transaction_receipt {
                transaction_receipt.status.map_or(
                    Cell::from(Spinner::default().to_string()).fg(Color::White),
                    |status| {
                        if status == U64::from(0) {
                            Cell::from("Failure".to_string()).fg(Color::Red)
                        } else {
                            Cell::from("Success".to_string()).fg(Color::Green)
                        }
                    },
                )
            } else {
                Cell::from(Spinner::default().to_string()).fg(Color::White)
            },
            Cell::from(if let Some(transaction_receipt) = transaction_receipt {
                transaction_receipt.logs.len().to_string()
            } else {
                Spinner::default().to_string()
            })
            .fg(Color::White),
        ]);
    }

    row
}
