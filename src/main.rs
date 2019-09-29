extern crate chrono;
extern crate http;
extern crate isahc;
extern crate structopt;

use chrono::prelude::*;
use isahc::prelude::*;
use structopt::StructOpt;
mod monitoring_data;
use monitoring_data::*;

#[derive(Debug)]
enum HttpVerb {
    Get, 
    Head
}
use std::str::FromStr;

impl FromStr for HttpVerb {
    type Err = std::convert::Infallible;
    fn from_str(verb: &str) -> Result<Self, Self::Err> {
        match verb {
            "get" => Ok(HttpVerb::Get),
            "head" => Ok(HttpVerb::Head),
            _ => Ok(HttpVerb::Head),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
    #[structopt(long = "verb", default_value = "get")]
    verb: HttpVerb,
}

fn fetch_response(url: &str, verb: &HttpVerb) -> Result<Response<Body>, isahc::Error> {
    match verb {
        HttpVerb::Head => isahc::head(url),
        HttpVerb::Get => isahc::get(url)
    }
}

fn main() -> Result<(), isahc::Error> {
    let opt = Opt::from_args();

    let start = Utc::now();
    let mut response = fetch_response(&opt.url, &opt.verb)?;
    let end = Utc::now();

    let result = MonitoringData::new(
        end.timestamp_millis() - start.timestamp_millis(),
        response.status(),
        response.text()?.len(),
        response.headers(),
    );

    println!("{}", serde_json::to_string(&result).unwrap());
    Ok(())
}
