use termwiz::color::ColorAttribute;
use termwiz::input::{InputEvent, KeyCode, KeyEvent};
use termwiz::surface::{Change, Position};
use termwiz::terminal::Terminal;

use mux::termwiztermtab::TermWizTerminal;

pub fn claude_session_history(mut term: TermWizTerminal) -> anyhow::Result<Option<String>> {
    term.set_raw_mode()?;
    let mut input = String::new();

    loop {
        let changes = vec![
            Change::ClearScreen(ColorAttribute::Default),
            Change::CursorPosition {
                x: Position::Absolute(0),
                y: Position::Absolute(0),
            },
            Change::Title("Claude Session History".to_string()),
            Change::Text("Resume Claude Session\r\n\r\n".to_string()),
            Change::Text("Enter session ID (or Esc to cancel):\r\n".to_string()),
            Change::Text(format!("> {}", input)),
        ];
        term.render(&changes)?;

        match term.poll_input(None)? {
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Enter,
                ..
            })) => {
                if !input.is_empty() {
                    return Ok(Some(format!("/resume {}\r", input.trim())));
                }
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Char(c),
                ..
            })) => {
                input.push(c);
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Backspace,
                ..
            })) => {
                input.pop();
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Escape,
                ..
            })) => {
                return Ok(None);
            }
            _ => {}
        }
    }
}
