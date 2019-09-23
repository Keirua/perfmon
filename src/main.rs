extern crate isahc;
extern crate chrono;
extern crate http;
extern crate structopt;
use chrono::prelude::*;
use http::header::HeaderValue;
use http::StatusCode;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String
}

#[derive(Debug)]
struct MonitoringData {
    response_time_ms:i64,
    response_code: StatusCode
}

impl MonitoringData {
    pub fn new(response_time_ms: i64, response_code: StatusCode) -> MonitoringData {
        MonitoringData{response_time_ms, response_code}
    }
}

fn main() {
    let opt = Opt::from_args();

    let start = Utc::now();
    let response = isahc::head(opt.url).unwrap();
    let end = Utc::now();
    let millis = end.timestamp_millis()-start.timestamp_millis();
    let result = MonitoringData::new(millis, response.status());

    println!("{:?}", result);
    /*let millis_str = format!("{}", millis);
    response.headers_mut().insert("X-MONITHOR-TIME-ms", HeaderValue::from_str(&millis_str).unwrap());
    response.headers_mut().insert("X-MONITHOR-TIME-ms", HeaderValue::from_str(&millis_str).unwrap());
    println!("{:#?}", response.headers());*/
}