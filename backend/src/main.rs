#![feature(proc_macro)]
#![feature(plugin)]
#![plugin(docopt_macros)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate docopt;
extern crate dotenv;
#[macro_use] extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;

pub mod database;
pub mod schema;
pub mod models;
pub mod quotes;

use self::database::{establish_connection};
use self::quotes::{retrieve_quote, store_quotes, RetrievalRequest};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

docopt!(Args derive Debug, "
Quote Storage & Retrieval Utilities

Usage:
  quote_storage store <file> [--mark-retrieved]
  quote_storage retrieve <quote-id> [--mark-retrieved]
  quote_storage retrieve [--random-retrieved | --random-unretrieved | --first-unretrieved] [--mark-retrieved]
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

    if args.flag_version {
        let version = env!{"CARGO_PKG_VERSION"};

        return println!("quote_storage v{}", version);
    }

    let dbsession = establish_connection();

    // dbsession.begin_test_transaction().unwrap(); // Testing

    if args.cmd_store {
        let path = Path::new(&args.arg_file);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("FIXME: Invalid file reference found :'( {}", e)
        };

        store_quotes(dbsession, &quotes_from_buffered_reader(BufReader::new(file)), args.flag_mark_retrieved);

    } else if args.cmd_retrieve {
        let request = if args.flag_random_retrieved {
            RetrievalRequest::RandomRetrieved
        } else if args.flag_random_unretrieved {
            RetrievalRequest::RandomUnretrievedAndMark(args.flag_mark_retrieved)
        } else if args.flag_first_unretrieved {
            RetrievalRequest::FirstUnretrievedAndMark(args.flag_mark_retrieved)
        } else if args.arg_quote_id.len() > 0 {
            let id = args.arg_quote_id.parse::<i64>().unwrap(); // FIXME: Handle error gracefully

            RetrievalRequest::IdAndMark(id, args.flag_mark_retrieved)
        } else {
            RetrievalRequest::Random
        };

        let quote = retrieve_quote(dbsession, request);

        println!("{}", quote);

    } else {
        unreachable!("Should not be able to get here!")
    }
}

fn quotes_from_buffered_reader<B: BufRead>(bufreader: B) -> Vec<String> {
    bufreader.lines().map(|l| l.expect("Could not parse line")).collect()
}
