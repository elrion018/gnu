use crate::commands::Command;
use crate::keyboard::Keyboard;

pub struct Cli {
    command: Option<Box<dyn Command>>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli { command: None }
    }

    pub fn execute(&mut self) {
        let command = self.command.as_mut().unwrap();

        command.execute();
        Keyboard::listen(command);
    }

    pub fn set_command(&mut self, new_command: Box<dyn Command>) {
        self.command = Some(new_command);
    }
}
