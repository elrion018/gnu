use crate::commands::Command;
use clap::{Arg, ArgMatches};
use commands::MoreCommand;

mod commands;
mod keyboard;

struct CliParser {
    command: Option<String>,
    option: Option<String>,
    file_path: Option<String>,
}

impl From<ArgMatches> for CliParser {
    fn from(matches: ArgMatches) -> Self {
        CliParser {
            command: matches.get_one::<String>("command").cloned(),
            option: matches.get_one::<String>("option").cloned(),
            file_path: matches.get_one::<String>("file_path").cloned(),
        }
    }
}

struct Cli {
    command: Option<Box<dyn Command>>,
}

impl Cli {
    fn new() -> Cli {
        Cli { command: None }
    }

    fn set_command(&mut self, new_command: Box<dyn Command>) {
        self.command = Some(new_command);
    }

    fn get_command(&mut self) -> &mut Option<Box<dyn Command>> {
        &mut self.command
    }
}

fn main() {
    let mut cli = Cli::new();
    let arg_matches = clap::builder::Command::new("gnu")
        .arg(Arg::new("command").required(true))
        .arg(Arg::new("option").required(true))
        .arg(Arg::new("file_path").required(true))
        .get_matches();
    let args = CliParser::from(arg_matches);

    match args.command.unwrap().as_str() {
        "more" => cli.set_command(Box::new(MoreCommand::new(
            args.option.unwrap(),
            args.file_path.unwrap(),
        ))),
        _ => (),
    };

    let command = cli.get_command().as_mut().unwrap();

    command.execute();
    keyboard::listen(command);
}
