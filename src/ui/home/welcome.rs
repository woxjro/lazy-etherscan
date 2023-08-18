use ratatui::{prelude::*, widgets::*};

pub fn render<B: Backend>(f: &mut Frame<B>, rect: Rect) {
    let welcome_block = Block::default()
        .title("Welcome")
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let [detail_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1,1)].as_ref())
            .split(rect) else { return; };

    let banner = Paragraph::new(Text::from(
        cfonts::render(cfonts::Options {
            text: String::from("lazy|etherscan"),
            font: cfonts::Fonts::FontBlock,
            ..cfonts::Options::default()
        })
        .text,
    ))
    .block(welcome_block.to_owned())
    .wrap(Wrap { trim: false })
    .alignment(Alignment::Center);

    f.render_widget(banner, detail_rect);
    f.render_widget(welcome_block, rect);
}
