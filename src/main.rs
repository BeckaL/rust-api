#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde_json::Result;
use rocket::response::content;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct BankHoliday {
    title: String,
    date: String,
    notes: String,
    bunting: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegionalBankHoliday {
    division: String,
    events: Vec<BankHoliday>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/show_first_bank_holiday")]
async fn show_first_bank_holiday() -> String  {
    let regional_bank_holidays = fetch_bank_holiday_data().await;
    
    let first_bank_holiday = regional_bank_holidays.events.first().unwrap();
    serde_json::to_string(&first_bank_holiday).unwrap()
}

#[get("/show_all_bank_holidays")]
async fn show_all_bank_holidays() -> String  {
    let regional_bank_holidays = fetch_bank_holiday_data().await;
    
    serde_json::to_string(&regional_bank_holidays).unwrap()
}

async fn fetch_bank_holiday_data() -> RegionalBankHoliday {
    let request_url = "https://www.gov.uk/bank-holidays/england-and-wales.json";
    reqwest::get(request_url).await.unwrap().json::<RegionalBankHoliday>().await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, show_first_bank_holiday, show_all_bank_holidays])
}