#![allow(non_snake_case)]
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
extern crate prettytable;

use reqwest::Error;
use prettytable::{Table, Row, Cell, Attr, color};
use std::env;

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
struct LocationData {
    name: String,
    data: Result<Vec<WeatherData>, Error>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut allResults = Vec::new();

    // TODO: remove the first item of args in a cleaner way.
    for (i, location) in args.iter().enumerate() {
        if i == 0 { continue };

        allResults.push(LocationData {
           name: location.to_string(),
           data: get_forecasts(&get_location_id(location))
        });
    }

    print(allResults);
}

fn get_location_id(location: &String) -> i32 {
    match &location[..] {
        "porto" => 275317,
        "albufeira" => 273200,
        "colombo" => 311399,
        "barcelona" => 307297,
        "nysa" => 265168,
        _ => 0
    }
}

fn print(location_data: Vec<LocationData>) {
    let mut table = Table::new();
    let mut first_row = vec![Cell::new("").with_style(Attr::Bold)];

    // Create first row
    for forecast in &location_data {
        first_row.push(Cell::new(&forecast.name).with_style(Attr::Bold));
    }
    table.add_row(Row::new(first_row));

    // Create rest of the rows
    for forecast in location_data {
        match forecast.data {
            Ok(res) => print_row(res, &mut table),
            Err(e) => {
                println!("Error happened: {}", e);
                print_empty_row(&mut table);
            },
        };
    }
        
    table.printstd();
}

fn print_row(data: Vec<WeatherData>, table: &mut prettytable::Table) {
    for (i, item) in data.iter().enumerate() {
        let date = format_date(&item.Date);
        let temp = f_to_c(item.Temperature.Maximum.Value).to_string() + "/" + &f_to_c(item.Temperature.Minimum.Value).to_string();
        // +1 because of header row
        let current_row_index = i + 1;
        let current_row = table.get_row(current_row_index);

        match current_row {
            None => {
                table.add_row(Row::new(vec![Cell::new(&date).with_style(Attr::Bold), Cell::new(&temp)]));
            },
            Some(_row) => {
                table[current_row_index].add_cell(Cell::new(&temp));
            },
        }
    }
}

fn print_empty_row(table: &mut prettytable::Table) {
    for i in 1..table.len() {
        table[i].add_cell(Cell::new("N/A").with_style(Attr::ForegroundColor(color::RED)));
    }
}

fn get_forecasts(id: &i32) -> Result<Vec<WeatherData>, Error> {
    let request_url = format!("http://dataservice.accuweather.com/forecasts/v1/daily/5day/{location_id}?apikey={apikey}",
        location_id = id,
        apikey = "z6em40OIbyDIxJKnVLydnBndRkGNNtvN");
    //let request_url = format!("https://my-json-server.typicode.com/gpawlik/weather-rust/{}", &id);
    println!("Request: {}", request_url);

    let mut response = reqwest::get(&request_url)?;

    let data: Data = response.json()?;
    let forecasts: Vec<WeatherData> = data.DailyForecasts;
    
    Ok(forecasts)
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 30.0) / 2.0
}

fn format_date(date_string: &String) -> String {
    let split = date_string[..10].split("-"); 
    let split_vec = split.collect::<Vec<&str>>();

    split_vec[2].to_owned() + "/" + split_vec[1]
}