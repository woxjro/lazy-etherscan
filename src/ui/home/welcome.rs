use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App, rect: Rect) {
    let welcome_block = Block::default()
        .title("Welcome")
        .border_style(Style::default().fg(Color::White))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [logo_rect, details_rect] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .margin(1)
        .split(rect)
    else {
        return;
    };

    let details_block = Block::default();

    let banner = Paragraph::new(Text::from(
        cfonts::render(cfonts::Options {
            text: String::from("lazy|etherscan"),
            font: cfonts::Fonts::FontBlock,
            ..cfonts::Options::default()
        })
        .text,
    ))
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

    let details = Paragraph::new(vec![
        Line::from(
            Span::raw(format!("   {:<13}: {}", "RPC Endpoint", app.endpoint)).fg(Color::White),
        ),
        Line::from(Span::raw(format!("   {:<13}: {}", "Version", "v0.1.0")).fg(Color::White)),
        Line::from(
            Span::raw(format!(
                "   {:<13}: {}",
                "Document", "https://woxjro.github.io/lazy-etherscan"
            ))
            .fg(Color::White),
        ),
        Line::from(
            Span::raw(format!(
                "   {:<13}: {}",
                "Repository", "https://github.com/woxjro/lazy-etherscan"
            ))
            .fg(Color::White),
        ),
    ])
    .block(details_block.to_owned())
    .alignment(Alignment::Left);

    f.render_widget(welcome_block, rect);
    f.render_widget(banner, logo_rect);
    f.render_widget(details, details_rect);
    f.render_widget(details_block, details_rect);
}
