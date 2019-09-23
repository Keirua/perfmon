extern crate isahc;
extern crate chrono;
extern crate http;
extern crate structopt;
use chrono::prelude::*;
use http::header::{HeaderMap, HeaderValue};
use http::StatusCode;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String
}

#[derive(Debug)]
struct MonitoringData<'a> {
    response_time_ms:i64,
    response_code: StatusCode,
    headers: &'a HeaderMap<HeaderValue>
}

impl<'a> MonitoringData<'a> {
    pub fn new(response_time_ms: i64, response_code: StatusCode, headers: &'a HeaderMap<HeaderValue>) -> MonitoringData {
        MonitoringData{response_time_ms, response_code, headers}
    }
}

fn main() {
    let opt = Opt::from_args();

    let start = Utc::now();
    let response = isahc::head(opt.url).unwrap();
    let end = Utc::now();

    let millis = end.timestamp_millis()-start.timestamp_millis();
    let result = MonitoringData::new(millis, response.status(), response.headers());

    println!("{:?}", result);
}