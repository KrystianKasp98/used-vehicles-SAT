use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use chrono::Datelike;
use rand::Rng;

#[derive(Serialize)]
struct Car {
    year_of_production: u16,
    fuel_usage_per_100_km: f64,
    model: String
}

trait Calculate {
    fn fuel_consumption(&self, distance:u32) -> f64;
    fn fail_probability(&self) -> u8;
}

impl Calculate for Car {
    fn fuel_consumption(&self, distance:u32) -> f64 {
        let converted_distance = distance as f64;

        let converted_year = self.year_of_production as f64;
        let converted_current_year = get_year() as f64;
        let motor_efficiency = 100.0 - (converted_current_year - converted_year) * 0.75;

        let result:f64 = self.fuel_usage_per_100_km / 100.00 * converted_distance * (100.0 / motor_efficiency);
        return result
    }

    fn fail_probability(&self) -> u8 {
        let generate_fail_probability:u8 = rand::thread_rng().gen_range(1..=101);
        return generate_fail_probability;
    }
}

fn get_year() -> i32{
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    return year;
}



fn main() {
    let c_6 = Car {
        year_of_production: 2020,
        fuel_usage_per_100_km: 11.5, 
        model: "PeopleCar PasWagon C6".to_string()
    };

    println!("fuel consumption = {}", c_6.fuel_consumption(100));
    println!("fail probability = {}", c_6.fail_probability());

}
