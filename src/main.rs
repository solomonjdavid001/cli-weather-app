mod cli;
mod config;
mod models;
mod utils;
mod weather;

use colored::*;
use weather::{display_weather_info, get_weather_info};

fn main() {
    println!("{}", "Welcome to Weather App!".bright_yellow());

    loop {
        let city = cli::get_user_input("Please enter your city name:");
        let country_code = cli::get_user_input("Please enter the country code:");
        let api_key = config::API_KEY;

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        let input =
            cli::get_user_input("Do you want to search for weather in another city? (yes / no)");
        if input.to_lowercase() != "yes" {
            println!("Thank you...");
            break;
        }
    }
}
