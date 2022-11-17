extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::commands::Command;

pub fn listen(command: &mut Box<dyn Command>) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        write!(stdout, ":{}{}", Goto(1, 1), clear::All).unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl(_) => break,
            Key::Up => command.scroll_up(),
            Key::Down => command.scroll_down(),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
