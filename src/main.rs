#![allow(non_snake_case)]
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Data {
    DailyForecasts: Vec<WeatherData>,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    Date: String,
    Temperature: Temperature,
}
#[derive(Deserialize, Debug)]
struct Temperature {
    Minimum: TempData,
    Maximum: TempData,
}
#[derive(Deserialize, Debug)]
struct TempData {
    Value: f32,
    Unit: String,
}


fn main() {
    let forecasts = get_forecasts();
    
    match forecasts {
        Ok(res) => show_forecast(res),
        Err(e) => println!("Error happened: {}", e),
    };
}

fn get_forecasts() -> Result<Vec<WeatherData>, Error> {
    // let request_url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{location_id}?apikey={apikey}",
    //     location_id = "275317",
    //     apikey = "z6em40OIbyDIxJKnVLydnBndRkGNNtvN");
    let request_url = format!("https://my-json-server.typicode.com/gpawlik/weather-rust/db");
    println!("Request: {}", request_url);

    let mut response = reqwest::get(&request_url)?;

    let data: Data = response.json()?;
    let forecasts: Vec<WeatherData> = data.DailyForecasts;
    
    Ok(forecasts)
}

fn show_forecast(data: Vec<WeatherData>) {
    for item in data {
        println!("Date: {:?}", format_date(&item.Date));
        println!("Max: {:?}C", f_to_c(item.Temperature.Maximum.Value));
        println!("Min: {:?}C", f_to_c(item.Temperature.Minimum.Value));
    }
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 30.0) / 2.0
}

fn format_date(date_string: &String) -> String {
    let mut split = date_string[..10].split("-"); 
    let split_vec = split.collect::<Vec<&str>>();

    split_vec[2].to_owned() + "/" + split_vec[1]
}