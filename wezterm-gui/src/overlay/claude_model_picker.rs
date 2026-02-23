use termwiz::cell::AttributeChange;
use termwiz::color::ColorAttribute;
use termwiz::input::{InputEvent, KeyCode, KeyEvent};
use termwiz::surface::{Change, Position};
use termwiz::terminal::Terminal;

use mux::termwiztermtab::TermWizTerminal;

struct ModelEntry {
    label: &'static str,
    value: &'static str,
}

const MODELS: &[ModelEntry] = &[
    ModelEntry {
        label: "Claude Sonnet 4",
        value: "sonnet",
    },
    ModelEntry {
        label: "Claude Opus 4",
        value: "opus",
    },
    ModelEntry {
        label: "Claude Haiku 4",
        value: "haiku",
    },
];

pub fn claude_model_picker(mut term: TermWizTerminal) -> anyhow::Result<Option<String>> {
    term.set_raw_mode()?;
    let mut active_idx: usize = 0;

    loop {
        // Render: clear screen, show title, show models with highlight
        let mut changes = vec![
            Change::ClearScreen(ColorAttribute::Default),
            Change::CursorPosition {
                x: Position::Absolute(0),
                y: Position::Absolute(0),
            },
            Change::Title("Select Claude Model".to_string()),
            Change::Text("Select Claude Model:\r\n\r\n".to_string()),
        ];

        for (i, model) in MODELS.iter().enumerate() {
            if i == active_idx {
                changes.push(AttributeChange::Reverse(true).into());
            }
            changes.push(Change::Text(format!(
                "  {}. {}\r\n",
                i + 1,
                model.label
            )));
            if i == active_idx {
                changes.push(AttributeChange::Reverse(false).into());
            }
        }

        changes.push(Change::Text(
            "\r\nPress Enter to select, Esc to cancel\r\n".to_string(),
        ));
        term.render(&changes)?;

        // Handle input
        match term.poll_input(None)? {
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::UpArrow,
                ..
            })) => {
                active_idx = active_idx.saturating_sub(1);
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::DownArrow,
                ..
            })) => {
                if active_idx + 1 < MODELS.len() {
                    active_idx += 1;
                }
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Char(c),
                ..
            })) => {
                if let Some(idx) = c.to_digit(10) {
                    let idx = idx as usize;
                    if idx >= 1 && idx <= MODELS.len() {
                        return Ok(Some(format!("/model {}\r", MODELS[idx - 1].value)));
                    }
                }
                if c == 'k' {
                    active_idx = active_idx.saturating_sub(1);
                }
                if c == 'j' && active_idx + 1 < MODELS.len() {
                    active_idx += 1;
                }
            }
            Some(InputEvent::Key(KeyEvent {
                key: KeyCode::Enter,
                ..
            })) => {
                return Ok(Some(format!("/model {}\r", MODELS[active_idx].value)));
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
