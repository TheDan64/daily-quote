// use schema::{account};

#[derive(Queryable)]
pub struct Quote {
    pub id: i64,
    pub author: String,
    pub author_note: Optional<String>,
    pub note: Optional<String>, // Better name?
    pub sent: bool, // last_sent?
}
