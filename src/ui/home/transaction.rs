use crate::{
    app::transaction::{SelectableInputDataDetailItem, SelectableTransactionDetailItem},
    ethers::{
        transaction::calculate_transaction_fee,
        types::{ERC20Token, TransactionWithReceipt},
    },
    route::{ActiveBlock, RouteId},
    App,
};
use ethers::core::{
    types::U64,
    utils::{format_ether, format_units},
};
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    transaction_with_receipt: Option<TransactionWithReceipt>,
    rect: Rect,
) {
    if let Some(transaction_with_receipt) = transaction_with_receipt {
        let TransactionWithReceipt {
            transaction,
            transaction_receipt,
        } = transaction_with_receipt;

        let detail_block = Block::default()
            .title("Transaction Details")
            .border_style(
                if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            )
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let [detail_rect, input_data_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(15), Constraint::Min(1)].as_ref())
            .split(rect)
        else {
            return;
        };

        let mut details = vec![
            Line::from(
                Span::raw(format!(
                    "{:<17}: {:#x}",
                    "Transaction Hash", transaction.hash
                ))
                .fg(Color::White),
            ),
            Line::from(vec![
                Span::raw(format!("{:<17}: ", "Status")).fg(Color::White),
                transaction_receipt.status.map_or(Span::raw(""), |status| {
                    if status == U64::from(0) {
                        Span::styled("Failure", Style::default().fg(Color::Red))
                    } else {
                        Span::styled("Success", Style::default().fg(Color::Green))
                    }
                }),
            ]),
            Line::from(
                Span::raw(format!(
                    "{:<17}: #{}",
                    "Block",
                    transaction
                        .block_number
                        .map_or("pending...".to_owned(), |number| number.to_string())
                ))
                .fg(Color::White),
            ),
            Line::from(
                if app.transaction_detail_list_state.selected()
                    == Some(SelectableTransactionDetailItem::From.into())
                {
                    vec![
                        Span::raw(format!("{:<17}: ", "From"))
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                        Span::styled(
                            format!(
                                "{:#x} {}",
                                transaction.from,
                                if let Some(token) =
                                    ERC20Token::find_by_address(&app.erc20_tokens, transaction.from)
                                {
                                    format!("({}: {})", token.ticker, token.name)
                                } else if let Some(ens_id) =
                                    app.address2ens_id.get(&transaction.from)
                                {
                                    ens_id.as_ref().map_or("".to_string(), |ens_id| {
                                        format!("({})", ens_id.to_owned())
                                    })
                                } else {
                                    "".to_owned()
                                }
                            ),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]
                } else {
                    vec![
                        Span::raw(format!("{:<17}: ", "From")).fg(Color::White),
                        Span::styled(
                            format!(
                                "{:#x} {}",
                                transaction.from,
                                if let Some(token) =
                                    ERC20Token::find_by_address(&app.erc20_tokens, transaction.from)
                                {
                                    format!("({}: {})", token.ticker, token.name)
                                } else if let Some(ens_id) =
                                    app.address2ens_id.get(&transaction.from)
                                {
                                    ens_id.as_ref().map_or("".to_string(), |ens_id| {
                                        format!("({})", ens_id.to_owned())
                                    })
                                } else {
                                    "".to_owned()
                                }
                            ),
                            Style::default().fg(Color::Cyan),
                        ),
                    ]
                },
            ),
            Line::from(
                if app.transaction_detail_list_state.selected()
                    == Some(SelectableTransactionDetailItem::To.into())
                {
                    vec![
                        Span::raw(format!("{:<17}: ", "To"))
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                        Span::styled(
                            transaction
                                .to
                                .map_or("".to_owned(), |to| {
                                    format!(
                                        "{:#x} {}",
                                        to,
                                        if let Some(token) =
                                            ERC20Token::find_by_address(&app.erc20_tokens, to)
                                        {
                                            format!("({}: {})", token.ticker, token.name)
                                        } else if let Some(ens_id) = app.address2ens_id.get(&to) {
                                            ens_id.as_ref().map_or("".to_string(), |ens_id| {
                                                format!("({})", ens_id.to_owned())
                                            })
                                        } else {
                                            "".to_owned()
                                        }
                                    )
                                })
                                .to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]
                } else {
                    vec![
                        Span::raw(format!("{:<17}: ", "To")).fg(Color::White),
                        Span::styled(
                            transaction
                                .to
                                .map_or("".to_owned(), |to| {
                                    format!(
                                        "{:#x} {}",
                                        to,
                                        if let Some(token) =
                                            ERC20Token::find_by_address(&app.erc20_tokens, to)
                                        {
                                            format!("({}: {})", token.ticker, token.name)
                                        } else if let Some(ens_id) = app.address2ens_id.get(&to) {
                                            ens_id.as_ref().map_or("".to_string(), |ens_id| {
                                                format!("({})", ens_id.to_owned())
                                            })
                                        } else {
                                            "".to_owned()
                                        }
                                    )
                                })
                                .to_string(),
                            Style::default().fg(Color::Cyan),
                        ),
                    ]
                },
            ),
            Line::from(
                Span::raw(format!(
                    "{:<17}: {}",
                    "Transaction Type",
                    transaction.transaction_type.map_or("Legacy", |ty| {
                        if ty == U64::from(1) {
                            "1"
                        } else {
                            "2(EIP-1559)"
                        }
                    })
                ))
                .fg(Color::White),
            ),
            Line::from(Span::raw(format!("{:<17}: {}", "Gas", transaction.gas)).fg(Color::White)),
            Line::from(
                Span::raw(format!(
                    "{:<17}: {} ETH",
                    "Value",
                    format_ether(transaction.value)
                ))
                .fg(Color::White),
            ),
            Line::from(
                Span::raw(format!(
                    "{:<17}: {} ETH",
                    "Transaction Fee",
                    calculate_transaction_fee(&transaction, &transaction_receipt, None)
                        .unwrap_or("".to_string())
                ))
                .fg(Color::White),
            ),
        ];

        if let Some(gas_price) = transaction.gas_price {
            details.push(Line::from(
                Span::raw(format!(
                    "{:<17}: {} Gwei",
                    "Gas Price",
                    format_units(gas_price, "gwei").unwrap()
                ))
                .fg(Color::White),
            ));
        }

        details.push(Line::from(
            if app.transaction_detail_list_state.selected()
                == Some(SelectableTransactionDetailItem::InputData.into())
            {
                Span::raw(format!(
                    "{:<17}: {}",
                    "Input Data",
                    if let RouteId::InputDataOfTransaction(_) = app.get_current_route().get_id() {
                        "▼"
                    } else {
                        "▶"
                    }
                ))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
            } else {
                Span::raw(format!(
                    "{:<17}: {}",
                    "Input Data",
                    if let RouteId::InputDataOfTransaction(_) = app.get_current_route().get_id() {
                        "▼"
                    } else {
                        "▶"
                    }
                ))
                .fg(Color::White)
            },
        ));

        let input_data = transaction
            .input
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .chunks(64)
            .map(|window| window.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut raw_input_data = vec![];
        for (idx, line) in input_data.iter().enumerate() {
            raw_input_data.push(Line::from(vec![
                Span::raw(format!("{:>3}  ", idx + 1)).fg(Color::Gray),
                Span::raw(line.to_string()).fg(Color::White),
            ]));
        }

        let raw_decoded_input_data: Vec<Line> = vec![Line::from("")];

        app.input_data_scroll_state = app
            .input_data_scroll_state
            .content_length(raw_input_data.len() as u16);

        app.decoded_input_data_scroll_state = app
            .decoded_input_data_scroll_state
            .content_length(raw_decoded_input_data.len() as u16);

        if app.is_toggled {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(input_data_rect);

            // render INPUT DATA
            let block = Block::default().padding(Padding::new(1, 0, 0, 1));
            f.render_widget(
                Paragraph::new(raw_input_data.to_owned())
                    .alignment(Alignment::Left)
                    .block(
                        if let SelectableInputDataDetailItem::InputData =
                            SelectableInputDataDetailItem::from(
                                app.input_data_detail_list_state
                                    .selected()
                                    .unwrap_or(SelectableInputDataDetailItem::InputData.into()),
                            )
                        {
                            Block::default()
                                .borders(Borders::ALL)
                                .green()
                                .title(Span::styled(
                                    "INPUT DATA",
                                    Style::default().add_modifier(Modifier::BOLD).green(),
                                ))
                        } else {
                            Block::default()
                                .borders(Borders::ALL)
                                .gray()
                                .title(Span::styled(
                                    "INPUT DATA",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ))
                        },
                    )
                    .scroll((app.input_data_scroll, 0))
                    .wrap(Wrap { trim: false }),
                block.inner(chunks[0]),
            );

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[0]),
                &mut app.input_data_scroll_state,
            );

            // render DECODED INPUT DATA
            let block = Block::default().padding(Padding::new(0, 1, 0, 1));
            f.render_widget(
                Paragraph::new(raw_decoded_input_data)
                    .alignment(Alignment::Left)
                    .block(
                        //FIXME
                        if let SelectableInputDataDetailItem::DecodedInputData =
                            SelectableInputDataDetailItem::from(
                                app.input_data_detail_list_state
                                    .selected()
                                    .unwrap_or(SelectableInputDataDetailItem::InputData.into()),
                            )
                        {
                            Block::default()
                                .borders(Borders::ALL)
                                .green()
                                .title(Span::styled(
                                    "DECODED INPUT DATA",
                                    Style::default().add_modifier(Modifier::BOLD).green(),
                                ))
                        } else {
                            Block::default()
                                .borders(Borders::ALL)
                                .gray()
                                .title(Span::styled(
                                    "DECODED INPUT DATA",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ))
                        },
                    )
                    .scroll((app.decoded_input_data_scroll, 0))
                    .wrap(Wrap { trim: false }),
                block.inner(chunks[1]),
            );

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[1]),
                &mut app.decoded_input_data_scroll_state,
            );
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(0)])
                .split(input_data_rect);

            let titles = ["INPUT DATA", "DECODED INPUT DATA"]
                .iter()
                .map(|t| Line::from(t.to_owned()))
                .collect();

            let tabs = Tabs::new(titles)
                .block(
                    Block::default()
                        .borders(Borders::RIGHT | Borders::LEFT | Borders::TOP)
                        .border_style(
                            if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                                if let RouteId::InputDataOfTransaction(_) =
                                    app.get_current_route().get_id()
                                {
                                    Style::default().fg(Color::Green)
                                } else {
                                    Style::default().fg(Color::White)
                                }
                            } else {
                                Style::default().fg(Color::White)
                            },
                        ),
                )
                .select(
                    app.input_data_detail_list_state
                        .selected()
                        .unwrap_or(SelectableInputDataDetailItem::InputData.into()),
                )
                .style(Style::default())
                .highlight_style(Style::default().bold().green());
            f.render_widget(
                tabs,
                Block::default()
                    .padding(Padding::horizontal(1))
                    .inner(chunks[0]),
            );

            let block = Block::default().padding(Padding::new(1, 1, 0, 1));
            f.render_widget(
                Paragraph::new(raw_input_data.to_owned())
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM)
                            .border_style(
                                if let ActiveBlock::Main =
                                    app.get_current_route().get_active_block()
                                {
                                    if let RouteId::InputDataOfTransaction(_) =
                                        app.get_current_route().get_id()
                                    {
                                        Style::default().fg(Color::Green)
                                    } else {
                                        Style::default().fg(Color::White)
                                    }
                                } else {
                                    Style::default().fg(Color::White)
                                },
                            ),
                    )
                    .scroll((app.input_data_scroll, 0))
                    .wrap(Wrap { trim: false }),
                block.inner(chunks[1]),
            );

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[1]),
                &mut app.input_data_scroll_state,
            );
        }

        let details = Paragraph::new(details)
            .block(detail_block.to_owned())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        f.render_widget(details, detail_rect);
        f.render_widget(detail_block, rect);
    } else {
        let detail_block = Block::default()
            .title("Transaction Not Found")
            .border_style(
                if let ActiveBlock::Main = app.get_current_route().get_active_block() {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            )
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        f.render_widget(detail_block, rect);
    }
}
