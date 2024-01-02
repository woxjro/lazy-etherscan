use crate::{
    app::{address::SelectableContractDetailItem, App},
    ethers::types::AddressInfo,
    route::ActiveBlock,
};
use ethers::core::utils::format_ether;
use ratatui::{prelude::*, widgets::*};
use serde_json;

pub fn render<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    address_info: Option<AddressInfo>,
    rect: Rect,
) {
    if let Some(address_info) = address_info {
        let detail_block = Block::default()
            .title(format!("Address {:#x}", address_info.address))
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

        let [detail_rect, contract_detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(7), Constraint::Min(3)].as_ref())
            .split(rect)
        else {
            return;
        };

        let mut details = vec![];

        if let Some(token) = app
            .erc20_tokens
            .iter()
            .find(|erc20_token| erc20_token.contract_address == address_info.address)
        {
            details.push(Line::from(
                Span::raw(format!(
                    "{:<17}: {} ({})",
                    "ERC20", token.name, token.ticker
                ))
                .fg(Color::White),
            ));
        }

        if let Some(ens_id) = address_info.ens_id {
            details.push(Line::from(
                Span::raw(format!("{:<17}: {ens_id}", "FULL NAME")).fg(Color::White),
            ));
        }

        if let Some(avatar_url) = address_info.avatar_url {
            details.push(Line::from(
                Span::raw(format!("{:<17}: {avatar_url}", "AVATAR URL")).fg(Color::White),
            ));
        }

        details.push(Line::from(
            Span::raw(format!(
                "{:<17}: {} ETH",
                "ETH BALANCE",
                format_ether(address_info.balance)
            ))
            .fg(Color::White),
        ));

        let source_code_lines =
            if let Some(contract_source_code) = address_info.contract_source_code {
                let mut details = vec![];
                let source_code = contract_source_code.items[0]
                    .source_code()
                    .replace("\\n", "\n");
                let source_code = source_code.split('\n').collect::<Vec<_>>();

                for (idx, line) in source_code.iter().enumerate() {
                    details.push(Line::from(vec![
                        Span::raw(format!("{:>3}  ", idx + 1)).fg(Color::Gray),
                        Span::raw(format!("{}", line)).fg(Color::White),
                    ]));
                }
                details
            } else {
                vec![]
            };

        let abi_lines = if let Some(contract_abi) = address_info.contract_abi {
            let mut details = vec![];
            let contract_abi =
                serde_json::to_string_pretty(&serde_json::json!(contract_abi)).unwrap();

            let contract_abi_lines = contract_abi.split('\n').collect::<Vec<_>>();

            for (idx, line) in contract_abi_lines.iter().enumerate() {
                details.push(Line::from(vec![
                    Span::raw(format!("{:>3}  ", idx + 1)).fg(Color::Gray),
                    Span::raw(format!("{}", line)).fg(Color::White),
                ]));
            }
            details
        } else {
            vec![]
        };

        let create_block = |title| {
            Block::default()
                .borders(Borders::ALL)
                .gray()
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
        };

        if app.is_toggled {
            //TODO
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)])
                .split(contract_detail_rect);

            // render SOURCE CODE
            let block = Block::default().padding(Padding::new(1, 0, 0, 1));
            f.render_widget(
                Paragraph::new(source_code_lines.to_owned())
                    .alignment(Alignment::Left)
                    .block(create_block("SOURCE CODE"))
                    .scroll((0 as u16, 0)) //TODO
                    .wrap(Wrap { trim: false }),
                block.inner(chunks[0]),
            );

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[0]),
                &mut ScrollbarState::default().content_length(source_code_lines.len() as u16),
            );

            // render ABI
            let block = Block::default().padding(Padding::new(0, 1, 0, 1));
            f.render_widget(
                Paragraph::new(abi_lines.to_owned())
                    .alignment(Alignment::Left)
                    .block(create_block("ABI"))
                    .scroll((0 as u16, 0)) //TODO
                    .wrap(Wrap { trim: false }),
                block.inner(chunks[1]),
            );

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[1]),
                &mut ScrollbarState::default().content_length(abi_lines.len() as u16),
            );
        } else {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(0)])
                .split(contract_detail_rect);

            let block = Block::default().padding(Padding::horizontal(2));

            let titles = ["SOURCE CODE", "ABI"]
                .iter()
                .map(|t| Line::from(t.to_owned()))
                .collect();

            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::RIGHT | Borders::LEFT | Borders::TOP))
                .select(app.selectable_contract_detail_item.into())
                .style(Style::default())
                .highlight_style(Style::default().bold().green());
            f.render_widget(tabs, block.inner(chunks[0]));

            let inner = match app.selectable_contract_detail_item {
                SelectableContractDetailItem::ContractSourceCode => {
                    Paragraph::new(source_code_lines.to_owned())
                        .block(
                            Block::default()
                                .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM),
                        )
                        .alignment(Alignment::Left)
                        .scroll((0 as u16, 0)) //TODO
                        .wrap(Wrap { trim: false })
                }
                SelectableContractDetailItem::ContractAbi => Paragraph::new(abi_lines.to_owned())
                    .block(
                        Block::default().borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM),
                    )
                    .alignment(Alignment::Left)
                    .scroll((0 as u16, 0)) //TODO
                    .wrap(Wrap { trim: false }),
            };
            let block = Block::default().padding(Padding::new(2, 2, 0, 1));
            f.render_widget(inner, block.inner(chunks[1]));

            f.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼")),
                block.inner(chunks[1]),
                &mut ScrollbarState::default().content_length(match app
                    .selectable_contract_detail_item
                {
                    SelectableContractDetailItem::ContractSourceCode => source_code_lines.len(),
                    SelectableContractDetailItem::ContractAbi => abi_lines.len(),
                } as u16),
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
            .title("Address Not Found")
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
