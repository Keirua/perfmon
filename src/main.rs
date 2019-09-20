extern crate isahc;
extern crate chrono;
use chrono::prelude::*;

fn main() {
    let start = Utc::now();
    let response = isahc::head("https://httpbin.org/get").unwrap();
    let end = Utc::now();
    println!("{:#?}", response.headers());
    println!("{}ms", end.timestamp_millis()-start.timestamp_millis());
}