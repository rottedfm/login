use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Scroll up in git log
        KeyCode::Char('k') | KeyCode::Char('H') => {
            app.scroll_up();
        }
        // Scroll down in git log
        KeyCode::Char('j') | KeyCode::Char('J') => {
            app.scroll_down();
        }
        // Launch Hyprland
        KeyCode::Enter => {
            let _ = app.launcher();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
