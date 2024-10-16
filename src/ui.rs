use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Render the current frame of the ASCII art, centered in the terminal
    frame.render_widget(
        Paragraph::new(app.art.clone())
            .block(
                Block::bordered()
                .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center),
        frame.area()
    );
}
