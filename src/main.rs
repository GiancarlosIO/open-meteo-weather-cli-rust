use chrono::NaiveDateTime;
use clap::Parser;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    altitude: f64,
    #[arg(short, long)]
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
    windspeed_80m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    latitude: f64,
    longitude: f64,
    timezone: String,
    elevation: f64,
    hourly: Hourly,
}

impl Forecast {
    async fn get(longitude: f64, altitude: f64) -> Result<Self, ExitFailure> {
        // latitude = -12.04, longitude = -77.03,
        let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,windspeed_80m", longitude, altitude);
        println!("> Making http call to get data...");
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;

        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Args::parse();
    let altitude = args.altitude;
    let longitude = args.longitude;

    println!(
        "\n> Getting weather for the altitude {} and longitude {}",
        altitude, longitude
    );

    let response = Forecast::get(longitude, altitude).await?;

    let times = &response.hourly.time[0..48];
    let temperatures = &response.hourly.temperature_2m[0..48];
    let windspeeds = &response.hourly.windspeed_80m[0..48];
    for i in 0..48 {
        let date = NaiveDateTime::parse_from_str(&times[i], "%Y-%m-%dT%H:%M")?;
        let date_formatted = date.format("%l %p on %b %-d, %C%y").to_string();
        println!("\n{}", date_formatted);
        println!("\t🌡️ Temperature: {}°C", temperatures[i]);
        println!("\t💨 Windspeed: {}", windspeeds[i]);
    }

    Ok(())
}
