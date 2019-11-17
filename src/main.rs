#![allow(non_snake_case)]
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
#[macro_use] extern crate prettytable;

use reqwest::Error;
use prettytable::{Table, Row, Cell};

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

struct Location {
    id: i32,
    name: String,
}

fn main() {
    // let table = table!(["ABC", "DEFG", "HIJKLMN"],
    //                    ["foobar", "bar", "foo"],
    //                    ["foobar2", "bar2", "foo2"]);

    // table.printstd();

    let locations = vec![
        Location { id: 275317, name: String::from("Porto") },
        Location { id: 273200, name: String::from("Albufeira") }
    ];

    for location in locations {
       let forecasts = get_forecasts(&location.id);
       println!("{}", location.name);
    
        match forecasts {
            Ok(res) => show_forecast(res),
            Err(e) => println!("Error happened: {}", e),
        }; 
    } 
}

fn get_forecasts(id: &i32) -> Result<Vec<WeatherData>, Error> {
    // let request_url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{location_id}?apikey={apikey}",
    //     location_id = id,
    //     apikey = "z6em40OIbyDIxJKnVLydnBndRkGNNtvN");
    let request_url = format!("https://my-json-server.typicode.com/gpawlik/weather-rust/{}", &id);
    println!("Request: {}", request_url);

    let mut response = reqwest::get(&request_url)?;

    let data: Data = response.json()?;
    let forecasts: Vec<WeatherData> = data.DailyForecasts;
    
    Ok(forecasts)
}

fn show_forecast(data: Vec<WeatherData>) {
    let mut table = Table::new();
    table.add_row(row!["Date", "Porto"]);

    for item in data {
        table.add_row(row![format_date(&item.Date), f_to_c(item.Temperature.Maximum.Value).to_string() + "/" + &f_to_c(item.Temperature.Minimum.Value).to_string()]);
    }

    table.printstd();
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 30.0) / 2.0
}

fn format_date(date_string: &String) -> String {
    let split = date_string[..10].split("-"); 
    let split_vec = split.collect::<Vec<&str>>();

    split_vec[2].to_owned() + "/" + split_vec[1]
}