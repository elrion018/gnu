use std::fs::read_to_string;

pub trait Command {
    fn execute(&self);

    fn scroll_down(&mut self);

    fn scroll_up(&mut self);
}

pub struct MoreCommand {
    pub option: String,
    pub content: String,
    pub end_line_index: i32,
}

impl MoreCommand {
    pub fn new(option: String, file_path: String) -> MoreCommand {
        let content = read_to_string(file_path).expect("could not read file");

        MoreCommand {
            option,
            content,
            end_line_index: 5,
        }
    }
}

impl Command for MoreCommand {
    fn execute(&self) {
        let mut lines = self.content.lines();

        for _ in 0..self.end_line_index {
            let line = lines.next();

            println!("{}\r", line.unwrap());
        }
    }

    fn scroll_down(&mut self) {
        self.end_line_index += 1;

        self.execute();
    }

    fn scroll_up(&mut self) {
        self.end_line_index -= 1;

        self.execute();
    }
}
