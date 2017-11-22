extern crate hyper;
extern crate reqwest;
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::{Value, from_str};
use std::io;
use std::io::Read;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Timezone {
    name: String,
    timezone: String
}

#[derive(Serialize, Deserialize)]
struct Currency {
    name: String,
    code: String,
    symbol: String
}

fn main() {
    println!("Starting request!");
     let client = reqwest::Client::new();

      let mut res = client
        .get("https://restcountries.eu/rest/v2/all")
        .send()
        .unwrap();
    
    let mut body = String::new();

    res.read_to_string(&mut body).unwrap();

    let body: Value = from_str(&body).unwrap();

    let mut timezones : Vec<Timezone> = vec![];
    let mut currencies : Vec<Currency> = vec![];

    if let Some(country_array) = body.as_array() {
        for country in country_array.iter() {
            if let Some(country_timezones) = country["timezones"].as_array() {
                for timezone in country_timezones.iter() {
                    timezones.push(Timezone{
                        name: String::from(country["name"].as_str().unwrap_or(" ")),
                        timezone: String::from(timezone.as_str().unwrap_or(" "))
                    });
                }
            }

            if let Some(country_currencies) = country["currencies"].as_array() {
                for currency in country_currencies.iter() {
                    currencies.push(Currency{
                        name: String::from(currency["name"].as_str().unwrap_or(" ")),
                        code: String::from(currency["code"].as_str().unwrap_or(" ")),
                        symbol:String::from(currency["symbol"].as_str().unwrap_or(" ")),
                    })
                }
            }
        }
    }

    let timezone_path = Path::new("results/timezones.json");
    let currency_path = Path::new("results/currencies.json");

    let mut timezone_file = match File::create(&timezone_path) {
        Err(why) => panic!("couldn't create timezone file : {}", why.description()),
        Ok(file) => file,
    };

    let mut currency_file = match File::create(&currency_path) {
        Err(why) => panic!("couldn't create currency file : {}", why.description()),
        Ok(file) => file,
    };

    match timezone_file.write_all(serde_json::to_string(&timezones).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to timezone file : {}", why.description()),
        Ok(_) => println!("Successfully wrote file.")
    }

    match currency_file.write_all(serde_json::to_string(&currencies).unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to currency file : {}", why.description()),
        Ok(_) => println!("Successfully wrote file.")
    }

    println!("Finished parsing.");
}
