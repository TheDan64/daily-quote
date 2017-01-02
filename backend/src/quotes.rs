use diesel::pg::PgConnection;
use models::{Author, NewAuthor, NewQuote, Quote};
use regex::Regex;

pub fn store_quotes(conn: PgConnection, quotes: Vec<String>) {
    lazy_static! {
        static ref re: Regex = Regex::new(r#"^"(.*)" - ([\w .]*)(, (.*))?$"#).unwrap();
    }

    for quote in quotes {
        let group = re.captures(&quote).unwrap();

        let quote = group[1];
        let author = group[2];
        let quote_note = group[4];

        println!("\"{}\" - {}, {}", quote, author, quote_note);
    }
}

pub fn retrieve_quote(conn: PgConnection) {

}
