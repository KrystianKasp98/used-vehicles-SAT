use actix_web::{get, web, Responder, Result};
use serde::{Serialize, Deserialize};
use chrono::Datelike;
use rand::Rng;

struct Car {
    year_of_production: u16,
    fuel_usage_per_100_km: f64,
    model: String
}

#[derive(Deserialize)]
struct QueryParamsDistance{
    yearOfProduction: String,
    fuelUsagePer100Km: String,
}

#[derive(Deserialize)]
struct QueryParamsFail{
    vin: String
}


#[derive(Serialize)]
struct CarInfoFuel {
    fuel_usage: f64
}

#[derive(Serialize)]
struct CarInfoFail {
    fail_probability: String
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

fn get_default_car() -> Car{
    return Car {
        year_of_production: 2022,
        fuel_usage_per_100_km: 10.5, 
        model: "PeopleCar PasWagon C6".to_string()
    };
}

fn convert_fail_probability(car: Car) -> String{
    let mut car_fail_probability = car.fail_probability().to_string();
    car_fail_probability.push_str("%");
    return car_fail_probability;
}


#[get("/calculateDisselUsageForDistance")]
async fn calculate_dissel_usage_for_distance(query_params: web::Query<QueryParamsDistance>) -> Result<impl Responder> {
    let converted_year_of_production = query_params.yearOfProduction.to_string().parse::<u16>().unwrap();
    let converted_fuel_usage_per_100_km = query_params.fuelUsagePer100Km.to_string().parse::<f64>().unwrap();

    let c_6 = Car {
        year_of_production: converted_year_of_production,
        fuel_usage_per_100_km: converted_fuel_usage_per_100_km, 
        model: "PeopleCar PasWagon C6".to_string()
    };
    let car_info_fuel = CarInfoFuel{fuel_usage: c_6.fuel_consumption(1000)};
    Ok(web::Json(car_info_fuel))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probability_of_unit_injector_fail(query_paramas: web::Query<QueryParamsFail>) -> Result<impl Responder> {

    let vin = query_paramas.vin.to_string();
    let c_6_default = get_default_car();

    let car_info_fail = CarInfoFail{fail_probability: convert_fail_probability(c_6_default)};
    
    Ok(web::Json(car_info_fail))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
        .service(calculate_dissel_usage_for_distance)
        .service(probability_of_unit_injector_fail))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
