extern crate chrono;
extern crate http;
extern crate isahc;
extern crate structopt;

use chrono::prelude::*;
use isahc::prelude::*;
use structopt::StructOpt;
mod monitoring_data;
use monitoring_data::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
}

fn main() -> Result<(), isahc::Error> {
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

    println!("{}", serde_json::to_string(&result).unwrap());
    Ok(())
}
