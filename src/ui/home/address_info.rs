use crate::{app::App, ethers::types::AddressInfo, route::ActiveBlock};
use ethers::core::utils::format_ether;
use ratatui::{prelude::*, widgets::*};

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

        let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 1)].as_ref())
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

        if let Some(contract_metadata) = address_info.contract_metadata {
            details.push(Line::from(
                Span::raw(format!("{:<17}:", "CONTRACT METADATA")).fg(Color::White),
            ));

            let source_code = contract_metadata.items[0]
                .source_code()
                .replace("\\n", "\n");
            let source_code = source_code.split('\n').collect::<Vec<_>>();

            for line in source_code {
                details.push(Line::from(
                    Span::raw(format!("{:<19}{}", "", line)).fg(Color::White),
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
