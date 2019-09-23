extern crate isahc;
extern crate chrono;
extern crate http;
extern crate structopt;
use chrono::prelude::*;
use http::header::{HeaderMap, HeaderValue};
use http::StatusCode;
use structopt::StructOpt;
use std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String
}

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

impl<'a> fmt::Debug for MonitoringData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ response_time: {:?},\nresponse_code: {}\nheaders: {:#?} }}", self.response_time_ms, self.response_code.as_u16(), self.headers)
    }
}

fn main() {
    let opt = Opt::from_args();

    let start = Utc::now();
    let response = isahc::head(opt.url).unwrap();
    let end = Utc::now();

    let millis = end.timestamp_millis()-start.timestamp_millis();
    let result = MonitoringData::new(millis, response.status(), response.headers());

    /*let serialized = serde_json::to_string(&point).unwrap();*/
    println!("{:#?}", result);
}