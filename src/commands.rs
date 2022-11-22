use crossterm::{cursor::MoveUp, style::Print, terminal, ExecutableCommand};
use std::{
    fs::read_to_string,
    io::{stdout, Write},
};

pub trait Command {
    fn execute(&mut self);

    fn scroll_down(&mut self) -> bool;

    fn scroll_up(&mut self) -> bool;
}

pub struct MoreCommand {
    pub option: String,
    pub content: String,
    pub buffer: Vec<String>,
    pub start_line_index: usize,
}

impl MoreCommand {
    pub fn new(option: String, file_path: String) -> MoreCommand {
        let content = read_to_string(file_path).expect("could not read file");

        MoreCommand {
            option,
            content,
            buffer: vec![],
            start_line_index: 0,
        }
    }

    fn indicate(&self) {
        let mut stdout = stdout();
        let (_width, height) = terminal::size().expect("Could not get terminal size");

        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(MoveUp(height))
            .unwrap()
            .execute(Print(
                self.buffer[self.start_line_index..(self.start_line_index + (height as usize))]
                    .iter()
                    .map(|line| format!("\r{}\n", line))
                    .collect::<Vec<String>>()
                    .join(""),
            ))
            .unwrap()
            .flush()
            .expect("Failed to scroll");
    }
}

impl Command for MoreCommand {
    fn execute(&mut self) {
        let mut lines = self.content.lines();

        loop {
            let line = lines.next();

            if line == None {
                break;
            }

            self.buffer.push(line.unwrap().to_string());
        }

        let mut stdout = stdout();

        let (_width, height) = terminal::size().expect("Could not get terminal size");

        stdout
            .execute(Print(
                self.buffer[self.start_line_index..(self.start_line_index + (height as usize))]
                    .iter()
                    .map(|line| format!("\r{}\n", line))
                    .collect::<Vec<String>>()
                    .join(""),
            ))
            .expect("sth wrong");
    }

    fn scroll_down(&mut self) -> bool {
        let (_width, height) = terminal::size().expect("Could not get terminal size");

        if self.start_line_index >= self.buffer.len() - height as usize {
            return false;
        }

        self.start_line_index += 1;

        self.indicate();

        return true;
    }

    fn scroll_up(&mut self) -> bool {
        if self.start_line_index > 0 {
            self.start_line_index -= 1;
        }

        self.indicate();

        return true;
    }
}
