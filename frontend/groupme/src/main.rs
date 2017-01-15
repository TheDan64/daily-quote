extern crate docopt;
extern crate dotenv;
extern crate rustc_serialize;

use docopt::Docopt;

const USAGE: &'static str = "
GroupMe Utilities

Usage:
  groupme bot
  groupme (-h | --help)
  groupme --version

Options:
  -h --help    Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_bot: bool,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        let version = env!("CARGO_PKG_VERSION");

        return println!("groupme v{}", version);
    };

    println!("Hello, world!");
    println!("{:?}", args);
}
