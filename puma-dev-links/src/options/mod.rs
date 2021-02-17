use std;
extern crate clap;
use clap::{App, Arg, SubCommand};

pub enum SubCommandType {
  List,
  Port { app_name: Option<String> },
  Link { app_name: Option<String> },
  Unlink { app_name: Option<String> },
}

pub struct Options {
    pub sub_command : SubCommandType,
}

fn show_help(matches: clap::ArgMatches, code: i32) -> ! {
  println!("{}", matches.usage());
  std::process::exit(code);
}

fn app_name(matches: Option<&clap::ArgMatches>) -> Option<String> {
  matches.and_then( |a| a.value_of("APP").map(|v| v.to_owned()))
}

pub fn parse_opts() -> Options {
    let matches = App::new("puma-dev link helper")
        .version(clap::crate_version!())
        .subcommand(SubCommand::with_name("list")
                    .about("List apps"))
        .subcommand(SubCommand::with_name("port")
                    .about("Show linked port")
                    .arg(Arg::with_name("APP")
                    .help("App name")
                    .index(1)))
        .subcommand(SubCommand::with_name("link")
                    .about("Link app")
                    .arg(Arg::with_name("APP")
                        .help("App name")
                        .index(1)))
        .subcommand(SubCommand::with_name("unlink")
                    .about("Unlink app")
                    .arg(Arg::with_name("APP")
                    .help("App name")
                    .index(1)))
        .get_matches();

    match matches.subcommand() {
      ("list", ..) => { Options { sub_command: SubCommandType::List } },
      ("port", args) => {
        Options {
          sub_command: SubCommandType::Port { app_name: app_name(args) }
        }
      },
      ("link", args ) => {
        Options {
          sub_command: SubCommandType::Link { app_name: app_name(args) }
        }
      },
      ("unlink", args) => {
        Options {
          sub_command: SubCommandType::Unlink { app_name: app_name(args) }
        }
      },
      ("", ..)  => { Options { sub_command: SubCommandType::List } },
      _ => { show_help(matches, 1) }
    }
}
