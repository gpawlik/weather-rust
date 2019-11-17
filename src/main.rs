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
struct LocationData {
    name: String,
    data: Result<Vec<WeatherData>, Error>,
}

struct Location {
    id: i32,
    name: String,
}

fn main() {
    let locations = vec![
        Location { id: 275317, name: String::from("Porto") },
        Location { id: 273200, name: String::from("Albufeira") },
        Location { id: 311399, name: String::from("Colombo") },
    ];

    let mut allResults = Vec::new();

    for location in locations {
        allResults.push(LocationData {
           name: location.name,
           data: get_forecasts(&location.id)
        });
    }

    print(allResults);
}

fn print(location_data: Vec<LocationData>) {
    let mut table = Table::new();
    let mut first_row = vec![Cell::new("Data")];

    // Create first row
    for forecast in &location_data {
        first_row.push(Cell::new(&forecast.name));
    }
    table.add_row(Row::new(first_row));

    // Create rest of the rows
    for forecast in location_data {
        match forecast.data {
            Ok(res) => print_row(res, &mut table),
            Err(e) => println!("Error happened: {}", e),
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
                table.add_row(row![date, temp]);
            },
            Some(_row) => {
                table[current_row_index].add_cell(Cell::new(&temp));
            },
        }
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