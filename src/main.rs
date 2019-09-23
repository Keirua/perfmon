extern crate isahc;
extern crate chrono;
extern crate http;
use chrono::prelude::*;
use http::header::HeaderValue;

fn main() {
    let start = Utc::now();
    let mut response = isahc::head("https://www.evaneos.com").unwrap();
    let end = Utc::now();
    let millis = end.timestamp_millis()-start.timestamp_millis();
    let millis_str = format!("{}", millis);
    response.headers_mut().insert("X-MONITHOR-TIME-ms", HeaderValue::from_str(&millis_str).unwrap());
    println!("{:#?}", response.headers());
}