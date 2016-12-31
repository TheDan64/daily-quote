use diesel::pg::PgConnection;

pub fn store_quotes(conn: PgConnection, quotes: String) -> () {
	println!("{:?}", quotes);
}

pub fn retrieve_quote(conn: PgConnection) -> () {

}
