#![feature(proc_macro)]
#![feature(plugin)]
#![plugin(docopt_macros)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate docopt;
extern crate dotenv;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rustc_serialize;

pub mod database;
pub mod schema;
pub mod models;
pub mod quotes;

use self::database::{establish_connection};
use self::quotes::{retrieve_quote, store_quotes};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use diesel::connection::Connection;

docopt!(Args derive Debug, "
Quote Storage & Retrieval Utilities

Usage:
  quote_storage store <file> [--mark-retrieved]
  quote_storage retrieve <quote-id> [--mark-retrieved]
  quote_storage retrieve [--random | --random-retrieved | --random-unretrieved ] [--mark-retrieved]
  quote_storage (-h | --help)
  quote_storage --version

Options:
  -h --help             Show this screen.
  --version             Show version.
  --random              Retrieves a random quote.
  --random-retrieved    Retrieves a random already retrieved quote.
  --random-unretrieved  Retrieves a random quote that has not already been retrieved.
  --mark-retrieved      Marks quotes as retrieved.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    if args.flag_version {
        let version = env!{"CARGO_PKG_VERSION"};

        return println!("quote_storage v{}", version);
    }

    let conn = establish_connection();

    conn.begin_test_transaction().unwrap(); // TMP

    if args.cmd_store {
        let path = Path::new(&args.arg_file);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("FIXME: Invalid file reference found :'( {}", e)
        };

        store_quotes(conn, quotes_from_buffered_reader(BufReader::new(file)));
    } else if args.cmd_retrieve {
        // let quote = retrieve_quote(conn, args);

    } else {
        unreachable!("Should not be able to get here!")
    }
}

fn quotes_from_buffered_reader<B: BufRead>(bufreader: B) -> Vec<String> {
    bufreader.lines().map(|l| l.expect("Could not parse line")).collect()
}
