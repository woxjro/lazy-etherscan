use crate::{
    app::transaction::SelectableTransactionDetailItem,
    ethers::types::{ERC20Token, TransactionWithReceipt},
    route::ActiveBlock,
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

        let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 1)].as_ref())
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
                    format_ether(
                        transaction.gas_price.unwrap() * transaction_receipt.gas_used.unwrap() //TODO gas_price.unwrap()
                    )
                ))
                .fg(Color::White),
            ),
            Line::from(
                Span::raw(format!(
                    "{:<17}: {} Gwei",
                    "Gas Price",
                    format_units(transaction.gas_price.unwrap(), "gwei").unwrap()
                ))
                .fg(Color::White),
            ),
        ];

        let input_data = transaction
            .input
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .chunks(64)
            .map(|window| window.iter().collect::<String>())
            .collect::<Vec<String>>();

        for (i, row) in input_data.iter().enumerate() {
            if i == 0 {
                details.push(Line::from(
                    Span::raw(format!("{:<17}: {}", "Input Data", row)).fg(Color::White),
                ));
            } else {
                details.push(Line::from(
                    Span::raw(format!("{:<19}{}", "", row)).fg(Color::White),
                ));
            }
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
