extern crate chrono;
extern crate http;
extern crate isahc;
extern crate structopt;
use chrono::prelude::*;
use isahc::prelude::*;
use http::header::{HeaderMap, HeaderValue};
use http::StatusCode;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
}

use serde::ser::{Serialize, Serializer, SerializeStruct};

struct MonitoringData<'a> {
    response_time_ms: i64,
    response_code: StatusCode,
    response_body_size_bytes: usize,
    headers: &'a HeaderMap<HeaderValue>,
}

impl<'a> Serialize for MonitoringData<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MonitoringData", 3)?;
        state.serialize_field("response_time", &self.response_time_ms)?;
        state.serialize_field("response_code", &self.response_code.as_str())?;
        state.serialize_field("body_size", &self.response_body_size_bytes)?;
        state.end()
    }
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

    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
    Ok(())
}