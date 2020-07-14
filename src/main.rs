use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::{ExitFailure};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    place: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let url = format!("http://127.0.0.1:8445/todayweatherbycity/{}", args.place);

    let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
        .await?
         /*   .text()
            .await?;*/
            .json::<Forecast>()
            .await?;

    println!("{:?}", resp);
    Ok(())
}

//    .json::<HashMap<String,String>>()
//         .await?;


