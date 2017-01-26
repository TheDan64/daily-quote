use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::author::dsl::author;
use schema::quote::dsl::{quote, retrieved};

pub fn count_quotes(dbsession: PgConnection, unretrieved: bool) -> i64 {
    quote.count()
         .filter(retrieved.eq(!unretrieved))
         .first(&dbsession)
         .expect("Could not count quotes in the database")
}

pub fn count_authors(dbsession: PgConnection) -> i64 {
    author.count()
          .first(&dbsession)
          .expect("Count not count authors in the database")
}
