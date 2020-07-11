use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::{ExitFailure};
use serde::{Deserialize};
use std::collections::HashMap;
use reqwest::Url;

#[derive(StructOpt)]
struct Cli {
    place: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?id={}&appid={{}}", args.place);
    let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String,String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}


