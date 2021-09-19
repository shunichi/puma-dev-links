extern crate clap;
use std;

pub enum SubCommand {
  List,
  Port { app_name: Option<String> },
  Link { app_name: Option<String> },
  Unlink { app_name: Option<String> },
  Procfile,
}

pub struct Options {
    pub sub_command : SubCommand,
}

fn show_help(matches: clap::ArgMatches, code: i32) -> ! {
  println!("{}", matches.usage());
  std::process::exit(code);
}

fn app_name(matches: Option<&clap::ArgMatches>) -> Option<String> {
  matches.and_then( |a| a.value_of("APP").map(|v| v.to_owned()))
}

pub fn parse_opts() -> Options {
    let matches = clap::App::new("puma-dev link helper")
        .version(clap::crate_version!())
        .subcommand(clap::SubCommand::with_name("list")
                    .about("List apps"))
        .subcommand(clap::SubCommand::with_name("port")
                    .about("Show linked port")
                    .arg(clap::Arg::with_name("APP")
                    .help("App name")
                    .index(1)))
        .subcommand(clap::SubCommand::with_name("link")
                    .about("Link app")
                    .arg(clap::Arg::with_name("APP")
                        .help("App name")
                        .index(1)))
        .subcommand(clap::SubCommand::with_name("unlink")
                    .about("Unlink app")
                    .arg(clap::Arg::with_name("APP")
                    .help("App name")
                    .index(1)))
        .subcommand(clap::SubCommand::with_name("procfile")
                    .about("Create Procfile template"))
        .get_matches();

    match matches.subcommand() {
      ("list", ..) => { Options { sub_command: SubCommand::List } },
      ("procfile", ..) => { Options { sub_command: SubCommand::Procfile } },
      ("port", args) => {
        Options {
          sub_command: SubCommand::Port { app_name: app_name(args) }
        }
      },
      ("link", args ) => {
        Options {
          sub_command: SubCommand::Link { app_name: app_name(args) }
        }
      },
      ("unlink", args) => {
        Options {
          sub_command: SubCommand::Unlink { app_name: app_name(args) }
        }
      },
      ("", ..)  => { Options { sub_command: SubCommand::List } },
      _ => { show_help(matches, 1) }
    }
}
