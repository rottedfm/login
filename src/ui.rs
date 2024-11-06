use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let inner_area = outer_block.inner(frame.area());

    // Outer vertical layout for top (art) and bottom (horizontal chunks)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(5),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(inner_area);

    // Top section: Render `app.art`
    frame.render_widget(
        Paragraph::new(app.art.clone()).alignment(Alignment::Center),
        vertical_chunks[0],
    );

    frame.render_widget(
        Paragraph::new("-=> SYSTEM LOG <=-").alignment(Alignment::Center),
        vertical_chunks[1],
    );

    // Bottom section: Inner horizontal layout for left, middle (git_log), and right chunks
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33), // Left chunk
                Constraint::Percentage(34), // Middle chunk (git_log)
                Constraint::Percentage(33), // Right chunk
            ]
            .as_ref(),
        )
        .split(vertical_chunks[2]);

    // Render the outer block
    frame.render_widget(outer_block, frame.area());

    // Prepare styled git log text
    let git_log_text = Text::from(
        app.git_log
            .iter()
            .map(|line| {
                if line.starts_with('+') {
                    // Bold style for lines starting with '+'
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default().fg(ratatui::style::Color::Green),
                    ))
                } else if line.starts_with('-') {
                    // Italic style for lines starting with '-'
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default().fg(ratatui::style::Color::Red),
                    ))
                } else {
                    // Default style for other lines
                    Line::from(Span::raw(line.clone()))
                }
            })
            .collect::<Vec<Line>>(),
    );

    // Render the middle chunk with styled `git_log`
    frame.render_widget(
        Paragraph::new(git_log_text)
            .scroll((app.scroll, 0))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true }),
        horizontal_chunks[1],
    );
}
