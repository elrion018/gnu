use crate::commands::Command;

pub struct Cli {
    command: Option<Box<dyn Command>>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli { command: None }
    }

    pub fn set_command(&mut self, new_command: Box<dyn Command>) {
        self.command = Some(new_command);
    }

    pub fn get_command(&mut self) -> &mut Option<Box<dyn Command>> {
        &mut self.command
    }
}
