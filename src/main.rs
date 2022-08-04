#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;

use chrono::{Date, Utc, DateTime, NaiveDate};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use serde_json::Result;
use rocket::response::content;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct RegionalBankHoliday {
    division: String,
    events: Vec<BankHoliday>
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(PartialEq, Debug)]
struct BankHoliday {
    title: String,
    date: NaiveDate,
    notes: String,
    bunting: bool,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/show_first_bank_holiday")]
async fn show_first_bank_holiday() -> String  {
    let regional_bank_holidays = fetch_bank_holiday_data().await;
    
    let first_bank_holiday = regional_bank_holidays.events.first().unwrap();
    match serde_json::to_string(&first_bank_holiday) {
        Ok(bank_holiday_string) => bank_holiday_string,
        Err(e) => e.to_string(),
    }
}

#[get("/show_all_bank_holidays")]
async fn show_all_bank_holidays() -> String  {
    let regional_bank_holidays = fetch_bank_holiday_data().await;

    match serde_json::to_string(&regional_bank_holidays) {
        Ok(bank_holiday_string) => bank_holiday_string,
        Err(e)  => e.to_string(),
    }
}

async fn fetch_bank_holiday_data() -> RegionalBankHoliday {
    let request_url = "https://www.gov.uk/bank-holidays/england-and-wales.json";
    reqwest::get(request_url).await.unwrap().json::<RegionalBankHoliday>().await.unwrap()
}

fn next_bank_holiday(date: NaiveDate, bank_holiday_data: &RegionalBankHoliday) -> Option<&BankHoliday> {
    let mut iter = bank_holiday_data.events.iter();
    iter.find(|&event| event.date == date)
}

// fn create_date_from_string(string: String) -> Date<Utc> {
//     NaiveDate
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, show_first_bank_holiday, show_all_bank_holidays])
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::{BankHoliday, next_bank_holiday, RegionalBankHoliday};

    #[test]
    fn returns_next_bank_holiday_when_one_is_found() {
        let date: NaiveDate = NaiveDate::from_ymd(2022, 8, 4);
        let bank_holiday_on_date: BankHoliday = BankHoliday{
            title: String::from("a bank holiday"),
            date: date,
            notes: String::from(" "),
            bunting: false,
        };
        let rbh: RegionalBankHoliday = RegionalBankHoliday {
            division: String::from("whatever"),
            events: vec!(bank_holiday_on_date),
        };
        // TODO: be able to reference original bank holiday
        assert_eq!(next_bank_holiday(date, &rbh), Some(&BankHoliday{
            title: String::from("a bank holiday"),
            date: date,
            notes: String::from(" "),
            bunting: false,
        }))
    }
}