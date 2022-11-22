use clap::ArgMatches;

pub struct CliParser {
    pub command: Option<String>,
    pub option: Option<String>,
    pub file_path: Option<String>,
}

impl CliParser {}

impl From<ArgMatches> for CliParser {
    fn from(matches: ArgMatches) -> Self {
        CliParser {
            command: matches.get_one::<String>("command").cloned(),
            option: matches.get_one::<String>("option").cloned(),
            file_path: matches.get_one::<String>("file_path").cloned(),
        }
    }
}
