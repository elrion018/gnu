use std::{fs::read_to_string, path::PathBuf};

use clap::builder::Command;
use clap::{Arg, ArgMatches};

trait GnuCommand {
    fn execute(&self);
}

struct MoreCommand {
    pub option: String,
    pub file_path: String,
}

impl MoreCommand {
    fn new(option: String, file_path: String) -> MoreCommand {
        MoreCommand { option, file_path }
    }
}

impl GnuCommand for MoreCommand {
    fn execute(&self) {
        let content = read_to_string(&self.file_path).expect("could not read file");

        for line in content.lines() {
            println!("{}", line);
        }
        // .. 여기서 option과 같은 MoreCommand의 필드를 참조해서 구현하고 싶음
    }
}
struct CliParser {
    command: Option<String>,
    option: Option<String>,
    file_path: Option<String>,
}

impl From<ArgMatches> for CliParser {
    fn from(m: ArgMatches) -> Self {
        CliParser {
            command: m.get_one::<String>("command").cloned(),
            option: m.get_one::<String>("option").cloned(),
            file_path: m.get_one::<String>("file_path").cloned(),
        }
    }
}

struct Cli {
    command: Option<Box<dyn GnuCommand>>,
}

impl Cli {
    fn new() -> Cli {
        Cli { command: None }
    }

    fn set_command(&mut self, new_command: Box<dyn GnuCommand>) {
        self.command = Some(new_command);
    }

    fn get_command(&self) -> &Option<Box<dyn GnuCommand>> {
        &self.command
    }
}

fn main() {
    let arg_matches = Command::new("gnu")
        .arg(Arg::new("command").required(true))
        .arg(Arg::new("option").required(true))
        .arg(Arg::new("file_path").required(true))
        .get_matches();

    let mut cli = Cli::new();

    let args = CliParser::from(arg_matches);
    let command = args.command;
    let option = args.option;
    let file_path = args.file_path;

    match command.unwrap().as_str() {
        "more" => cli.set_command(Box::new(MoreCommand::new(
            option.unwrap(),
            file_path.unwrap(),
        ))),
        _ => (),
    };

    cli.get_command().as_ref().unwrap().execute();
}
