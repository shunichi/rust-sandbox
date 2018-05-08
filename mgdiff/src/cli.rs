use clap::{App, Arg};

pub struct Options {
    pub down: bool,
    pub down_multi: bool,
    pub branch: String,
}

pub fn parse_opts() -> Options {
    let matches = App::new("rails migration diff")
        .version(crate_version!())
        .arg(Arg::with_name("down")
                 .short("d")
                 .long("down")
                 .help("migrate down"))
        .arg(Arg::with_name("down-multi")
                 .short("m")
                 .long("down-multi")
                 .help("migrate down multiple at once"))
        .arg(Arg::with_name("BRANCH")
            .required(true)
            .help("specify branch name switching to"))
        .get_matches();
    Options {
      down: matches.is_present("down"),
      down_multi: matches.is_present("down-multi"),
      branch: matches.value_of("BRANCH").unwrap().to_string(),
    }
}
