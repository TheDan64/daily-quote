use schema::{author, quote};

#[derive(Queryable)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub note: String,
}

#[derive(Queryable)]
pub struct Quote {
    pub id: i64,
    pub author_id: i64,
    pub text: String,
    pub note: String,
    pub retrieved: bool,
}

#[derive(Insertable)]
#[table_name="author"]
pub struct NewAuthor<'a> {
    pub name: &'a str,
    pub note: &'a str,
}

#[derive(Insertable)]
#[table_name="quote"]
pub struct NewQuote<'a> {
    pub author_id: i64,
    pub text: &'a str,
    pub note: &'a str,
    pub retrieved: bool,
}
