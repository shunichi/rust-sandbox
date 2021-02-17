use std;
extern crate clap;
use clap::{App, SubCommand};

pub enum SubCommandType {
  List,
  Port,
  Link,
}

pub struct Options {
    pub sub_command : SubCommandType,
}

fn show_help(matches: clap::ArgMatches, code: i32) -> ! {
  println!("{}", matches.usage());
  std::process::exit(code);
}

pub fn parse_opts() -> Options {
    let matches = App::new("puma dev link helper")
        .version(clap::crate_version!())
        .subcommand(SubCommand::with_name("list")
                    .about("list apps"))
        .subcommand(SubCommand::with_name("port")
                    .about("find linked port"))
        .subcommand(SubCommand::with_name("link")
                    .about("link app"))
        .get_matches();

    match matches.subcommand() {
      ("list", ..) => { Options { sub_command: SubCommandType::List } },
      ("port", ..) => { Options { sub_command: SubCommandType::Port } },
      ("link", ..) => { Options { sub_command: SubCommandType::Link } },
      ("", ..)  => { Options { sub_command: SubCommandType::List } },
      _ => { show_help(matches, 1) }
    }
}
