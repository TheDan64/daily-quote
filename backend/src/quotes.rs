use diesel::expression::dsl::any;
use diesel::{insert, update};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Author, NewAuthor, NewQuote, Quote};
use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;
use regex::Regex;
use schema::author::table as author_table;
use schema::quote::table as quote_table;
use std::collections::{HashSet, HashMap};
use std::io::{stdin, stdout, Write};
use std::iter::FromIterator;

struct UnlinkedNewQuote<'a> {
    author_name: &'a str,
    note: &'a str,
    text: &'a str,
}

pub fn store_quotes<'a>(dbsession: PgConnection, quotes: &'a Vec<String>, mark_retrieved: bool) {
    use schema::author::dsl::{author, name, id};

    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^"(.*)" - ([\w .]*)(, (.*))?$"#).unwrap();
    }

    let mut new_quote_count = 0;
    let mut new_author_count = 0;
    let mut author_names: HashSet<&str> = HashSet::new();
    let mut unlinked_new_quotes = Vec::with_capacity(quotes.len());

    for string_quote in quotes {
        let group = match RE.captures(&string_quote) {
            Some(g) => g,
            None => panic!("Invalidly formatted quote found: {}", string_quote)
        };

        let quote_text = group.get(1).unwrap().as_str();
        let author_name = group.get(2).unwrap().as_str();
        let quote_note = group.get(4).as_ref().map(|m| m.as_str()).unwrap_or("");

        author_names.insert(author_name);

        let unlinked_new_quote = UnlinkedNewQuote {
            text: quote_text,
            note: quote_note,
            author_name: author_name,
        };

        unlinked_new_quotes.push(unlinked_new_quote);

        new_quote_count += 1;
    }

    let existing_authors: Vec<(String, _)> = author.filter(name.eq(any(Vec::from_iter(author_names))))
                                                   .select((name, id))
                                                   .load(&dbsession)
                                                   .expect("Error loading authors");

    let mut author_id_by_name = HashMap::new();

    for (author_name, quote_author_id) in existing_authors {
        author_id_by_name.insert(author_name, quote_author_id);
    }

    let mut new_quotes: Vec<NewQuote> = vec![];

    for ul_quote in unlinked_new_quotes {
        let mut update_hash_map = false;
        let author_id = match author_id_by_name.get(ul_quote.author_name) {
            Some(author_id) => *author_id,
            None => {
                println!("Found new author: {}", ul_quote.author_name);
                print!("Add an author note [optional]: ");

                let mut author_note = String::with_capacity(17);

                stdout().flush().expect("Error flushing stdout");
                stdin().read_line(&mut author_note).expect("Error reading from stdin");

                if let Some('\n') = author_note.chars().next_back() {
                    author_note.pop();
                }

                if let Some('\r') = author_note.chars().next_back() {
                    author_note.pop();
                }

                let new_author = NewAuthor {
                    name: ul_quote.author_name,
                    note: &author_note,
                };

                let author_insert: Author = insert(&new_author).into(author_table)
                                                               .get_result(&dbsession)
                                                               .expect("Error inserting new author");
                new_author_count += 1;

                update_hash_map = true;

                author_insert.id
            }
        };

        if update_hash_map {
            author_id_by_name.insert(ul_quote.author_name.into(), author_id);
        }

        let new_quote = NewQuote {
            author_id: author_id,
            note: ul_quote.note,
            retrieved: mark_retrieved,
            text: ul_quote.text,
        };

        new_quotes.push(new_quote);
    }

    insert(&new_quotes).into(quote_table)
                       .execute(&dbsession)
                       .expect("Error saving new quotes");

    println!("Created {} new authors and {} new quotes", new_author_count, new_quote_count);
}

pub enum RetrievalRequest {
    Random,
    RandomRetrieved,
    RandomUnretrievedAndMark(bool),
    FirstUnretrievedAndMark(bool),
    IdAndMark(i64, bool),
}

pub fn retrieve_quote(dbsession: PgConnection, request: RetrievalRequest) -> String {
    use schema::quote::dsl::{quote, id as quote_id, retrieved};

    let mut rng = thread_rng();

    let (quote_dbo, author_dbo) = match request {
        RetrievalRequest::Random => {
            let count: i64 = quote.count()
                                  .first(&dbsession)
                                  .expect("Unable to count quotes in the database");

            let random_quote_id = Range::new(1, count + 1).ind_sample(&mut rng);

            quote_table.inner_join(author_table)
                       .filter(quote_id.eq(random_quote_id))
                       .first(&dbsession)
                       .expect("Could not retrieve quote")
        },
        RetrievalRequest::RandomRetrieved => {
            let count: i64 = quote.count()
                                  .filter(retrieved.eq(true))
                                  .first(&dbsession)
                                  .expect("Unable to count quotes in the database");

            let offset = Range::new(0, count).ind_sample(&mut rng);

            quote_table.inner_join(author_table)
                       .filter(retrieved.eq(true))
                       .offset(offset)
                       .first(&dbsession)
                       .expect("Could not retrieve quote from the database")
        },
        RetrievalRequest::RandomUnretrievedAndMark(mark_retrieved) => {
            let count: i64 = quote.count()
                                  .filter(retrieved.eq(false))
                                  .first(&dbsession)
                                  .expect("Unable to count quotes in the database");

            if count == 0 {
                panic!("No unretrieved quotes found in the database"); // FIXME
            }

            let offset: i64 = Range::new(0, count).ind_sample(&mut rng);

            let (quote_dbo, author_dbo) = quote_table.inner_join(author_table)
                                                     .filter(retrieved.eq(false))
                                                     .offset(offset)
                                                     .first(&dbsession)
                                                     .expect("Unable to retrieve quote from the database");
            if mark_retrieved {
                update(&quote_dbo).set(retrieved.eq(true))
                                  .execute(&dbsession)
                                  .expect("Unable to save changes to quote dbo");
            }

            (quote_dbo, author_dbo)
        },
        RetrievalRequest::FirstUnretrievedAndMark(mark_retrieved) => {
            let (quote_dbo, author_dbo) = quote_table.inner_join(author_table)
                                                     .filter(retrieved.eq(false))
                                                     .order(quote_id)
                                                     .first(&dbsession)
                                                     .expect("Unable to retrieve quote from the database");
            if mark_retrieved {
                update(&quote_dbo).set(retrieved.eq(true))
                                  .execute(&dbsession)
                                  .expect("Unable to save changes to quote dbo");
            }

            (quote_dbo, author_dbo)
        },
        RetrievalRequest::IdAndMark(id, mark_retrieved) => {
            // FIXME: Possible issue when value out of range
            let (quote_dbo, author_dbo) = quote_table.inner_join(author_table)
                                                     .filter(quote_id.eq(id))
                                                     .first(&dbsession)
                                                     .expect("Could not retrieve quote");

            if mark_retrieved {
                update(&quote_dbo).set(retrieved.eq(true))
                                  .execute(&dbsession)
                                  .expect("Unable to save changes to quote dbo");
            }

            (quote_dbo, author_dbo)
        }
    };

    format_quote(quote_dbo, author_dbo)
}

fn format_quote(quote: Quote, author: Author) -> String {
    let mut quote_len = quote.text.len() + 5; // 2 quotation marks + 2 spaces + dash

    if quote.note.len() > 0 {
        quote_len += quote.note.len() + 2; // + comma + space
    }

    if author.note.len() > 0 {
        quote_len += author.note.len() + 3; // + space + 2 brackets
    }

    let mut quote_string = String::with_capacity(quote_len);

    quote_string.push('"');
    quote_string.push_str(&quote.text);
    quote_string.push('"');
    quote_string.push_str(" - ");
    quote_string.push_str(&author.name);

    if author.note.len() > 0 {
        quote_string.push_str(" [");
        quote_string.push_str(&author.note);
        quote_string.push(']');
    }

    if quote.note.len() > 0 {
        quote_string.push_str(", ");
        quote_string.push_str(&quote.note);
    }

    quote_string
}
