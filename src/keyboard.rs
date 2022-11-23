use crossterm::event::KeyCode;
use crossterm::{event, terminal};

use crate::commands::Command;

pub struct Keyboard {}

impl Keyboard {
    pub fn listen(command: &mut Box<dyn Command>) {
        terminal::enable_raw_mode().expect("Could not enable raw mode");

        loop {
            match event::read().expect("Failed to read event") {
                event::Event::Key(event) => match event.code {
                    KeyCode::Char('c') => {
                        if event.modifiers == event::KeyModifiers::CONTROL {
                            break;
                        }
                    }
                    KeyCode::Up => command.scroll_up(),
                    KeyCode::Down | KeyCode::Enter => command.scroll_down(),
                    _ => {}
                },
                _ => {}
            }
        }

        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}
