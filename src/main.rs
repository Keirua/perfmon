extern crate chrono;
extern crate http;
extern crate isahc;
extern crate structopt;
use chrono::prelude::*;
use isahc::prelude::*;
use http::header::{HeaderMap, HeaderValue};
use http::StatusCode;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
}

struct MonitoringData<'a> {
    response_time_ms: i64,
    response_code: StatusCode,
    response_body_size_bytes: usize,
    headers: &'a HeaderMap<HeaderValue>,
}

impl<'a> MonitoringData<'a> {
    pub fn new(
        response_time_ms: i64,
        response_code: StatusCode,
        response_body_size_bytes: usize,
        headers: &'a HeaderMap<HeaderValue>,
    ) -> MonitoringData {
        MonitoringData {
            response_time_ms,
            response_code,
            response_body_size_bytes,
            headers,
        }
    }
}

impl<'a> fmt::Debug for MonitoringData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \"response_time\": {:?},\n\"response_code\": {} }}",
            self.response_time_ms,
            self.response_code.as_u16()
        )
    }
}

fn main() -> Result<(), isahc::Error>{
    let opt = Opt::from_args();

    let start = Utc::now();
    let mut response = isahc::head(opt.url)?;
    let end = Utc::now();

    let result = MonitoringData::new(
        end.timestamp_millis() - start.timestamp_millis(),
        response.status(),
        response.text()?.len(),
        response.headers(),
    );

    println!("{:#?}", result);
    Ok(())
}