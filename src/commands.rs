use crossterm::{cursor::MoveUp, style::Print, terminal, ExecutableCommand};
use std::{
    fs::read_to_string,
    io::{stdout, Write},
};

pub trait Command {
    fn execute(&mut self);

    fn click_down(&mut self) {}

    fn click_up(&mut self) {}

    fn click_f(&mut self) {}

    fn click_b(&mut self) {}
}

pub struct MoreCommand {
    pub option: String,
    pub content: String,
    pub file_path: String,
    pub buffer: Vec<String>,
    pub start_line_index: usize,
}

impl MoreCommand {
    pub fn new(option: String, file_path: String) -> MoreCommand {
        let content = read_to_string(&file_path).expect("could not read file");

        MoreCommand {
            option,
            content,
            file_path,
            buffer: vec![],
            start_line_index: 0,
        }
    }

    fn indicate(&self) {
        let mut stdout = stdout();
        let height = self.get_terminal_height();

        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Could not clear All lines")
            .execute(MoveUp(height as u16))
            .expect("Could not move cursor to up")
            .execute(Print(
                self.buffer[self.start_line_index..(self.start_line_index + (height as usize))]
                    .iter()
                    .map(|line| format!("\r{}\n", line))
                    .collect::<Vec<String>>()
                    .join(""),
            ))
            .expect("Could not print lines")
            .flush()
            .expect("Could not flush");

        match self.check_is_before_end_scroll() {
            true => stdout.execute(Print("END")),
            false => stdout.execute(Print(":")),
        }
        .unwrap();
    }

    pub fn check_is_end_scroll(&self) -> bool {
        let height = self.get_terminal_height();

        if self.start_line_index > self.buffer.len() - height {
            return true;
        }

        return false;
    }

    pub fn check_is_before_end_scroll(&self) -> bool {
        let height = self.get_terminal_height();

        if self.start_line_index == self.buffer.len() - height {
            return true;
        }

        return false;
    }

    fn get_terminal_height(&self) -> usize {
        let (_, height) = terminal::size().expect("Could not get terminal size");

        height as usize
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

        let height = self.get_terminal_height();

        stdout
            .execute(Print(
                self.buffer[self.start_line_index..(self.start_line_index + (height as usize))]
                    .iter()
                    .map(|line| format!("\r{}\n", line))
                    .collect::<Vec<String>>()
                    .join(""),
            ))
            .expect("Could not get lines")
            .execute(Print(&self.file_path))
            .expect("Could not get file path");
    }

    fn click_down(&mut self) {
        self.start_line_index += 1;

        if self.check_is_end_scroll() {
            let mut stdout = stdout();

            stdout
                .execute(terminal::Clear(terminal::ClearType::CurrentLine))
                .expect("Colud not clear current line");

            terminal::disable_raw_mode().expect("Could not disable raw mode");

            std::process::exit(exitcode::OK);
        }

        self.indicate();
    }

    fn click_up(&mut self) {
        if self.start_line_index > 0 {
            self.start_line_index -= 1;
        }

        self.indicate();
    }

    fn click_b(&mut self) {
        let height = self.get_terminal_height();

        if (self.start_line_index as i32 - height as i32) >= 0 {
            self.start_line_index -= height;

            self.indicate();
        }
    }

    fn click_f(&mut self) {
        let height = self.get_terminal_height();

        if self.start_line_index == self.buffer.len() - height {
            let mut stdout = stdout();

            stdout
                .execute(terminal::Clear(terminal::ClearType::CurrentLine))
                .expect("Colud not clear current line");

            terminal::disable_raw_mode().expect("Could not disable raw mode");

            std::process::exit(exitcode::OK);
        }

        if self.start_line_index + height > self.buffer.len() - height {
            self.start_line_index = self.buffer.len() - height
        } else {
            self.start_line_index += height;
        }

        self.indicate();
    }
}
