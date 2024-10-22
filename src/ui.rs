use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let inner_area = outer_block.inner(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(inner_area);

    frame.render_widget(outer_block, frame.area());

    frame.render_widget(
        Paragraph::new(app.art.clone()).alignment(Alignment::Center),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(app.git_log.join("\n"))
            .scroll((app.scroll, 0))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        chunks[1],
    );
}
