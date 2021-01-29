pub mod bot;

use docopt::Docopt;
use dotenv::dotenv;
use self::bot::Bot;
use std::env;
use std::io::Read;

const USAGE: &'static str = "
GroupMe Utilities

Usage:
  groupme bot send
  groupme bot send <message>
  groupme (-h | --help)
  groupme --version

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_bot: bool,
    cmd_send: bool,
    flag_help: bool,
    flag_version: bool,
    arg_message: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        let version = env!("CARGO_PKG_VERSION");

        return println!("groupme v{}", version);
    };

    println!("{:?}", args);

    if !args.cmd_bot && !args.cmd_send {
        panic!("Halp");
    }

    if args.arg_message.len() > 0 {
        panic!("Halp2");
    }

    dotenv().ok();

    let bot_id = match env::var("BOT_ID") {
        Ok(id) => id,
        Err(_) => panic!("Could not find config setting for `BOT_ID`")
    };

    let bot = Bot::new(bot_id);

    let success = bot.send_message(readable_to_string(std::io::stdin()));

    assert!(success);
}

fn readable_to_string<R: Read>(mut readable: R) -> String {
    let mut input_string = String::new();

    if let Err(e) = readable.read_to_string(&mut input_string) {
        panic!("Failed to read: {}", e);
    }

    input_string
}
