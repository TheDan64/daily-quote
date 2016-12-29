// use std::io::B
#![feature(proc_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod database;
pub mod schema;
pub mod models;

use self::database::establish_connection;

fn main() {
    let conn = establish_connection();

    println!("Hello, world!");
}
