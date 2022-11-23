mod cli;
mod cli_parser;
mod commands;
mod keyboard;

use clap::Arg;

use cli::Cli;
use cli_parser::CliParser;
use commands::MoreCommand;

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

    cli.execute();
}
