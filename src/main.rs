use structopt::StructOpt;
use exitfailure::{ExitFailure};
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
    let resp = Forecast::get(&args.place).await?;
    let temp_cel = kelvin_to_celcius(resp.main.temp);
    let wind_speed = miles_per_sec_to_kmh(resp.wind.speed);
    let wind_direction = degrees_to_compass(resp.wind.deg);
    println!("{}: Clouds: {}  Temp: {:.2}  Humidity: {}%  Wind Speed: {}  Wind Direction: {} ", args.place, resp.weather.details.description, temp_cel, resp.main.humidity, wind_speed, wind_direction  );
    Ok(())
}

fn kelvin_to_celcius(kel: f64) -> f64{
     kel - 273.15
}
fn degrees_to_compass(deg: i32) -> & 'static str {
    match deg {
        00..=21 => return "North",
        22..=43 => return "North Northeast",
        44..=45 => return "North East",
        46..=66 => return "East Northeast",
        67..=111 => return "East",
        112..=133 => return "East Southeast",
        134..=135 => return "Southeast",
        136..=156 => return "South Southeast",
        157..=201 => return "South",
        202..=223 => return "South Southwest",
        224..=225 => return "Southwest",
        226..=246 => return "West Southwest",
        247..=291 => return "West",
        292..=313 => return "West Northwest",
        314..=315 => return "Northwest",
        316..=336 => return "North Northewest",
        337..=360 => return "North",
        _ => return "Error getting wind direction",
    }
}
fn miles_per_sec_to_kmh(inputspeed: f64) -> f64 {
    inputspeed * 3.6
}

impl Forecast {
    async fn get(place: &String) -> Result<Self,ExitFailure>{
        let url = format!("http://127.0.0.1:8445/todayweatherbycity/{}", place);

        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }


}

//    .json::<HashMap<String,String>>()
//         .await?;


