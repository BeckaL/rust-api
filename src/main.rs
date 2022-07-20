#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;

use rocket::serde::{Deserialize};

use rocket::response::content;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct BankHoliday {
    title: String,
    // TODO: make date a special date class
    date: String,
    notes: String,
    bunting: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegionalBankHoliday {
    division: String,
    events: Vec<BankHoliday>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
async fn test() -> String  {
    println!("test called");
    let rbh = blah().await;
    dbg!(rbh.division);
    // content::RawJson("[{
    //     'title': 'New Years Day',
    //     'date': '2017-01-02',
    //     'notes': 'Substitute day',
    //     'bunting': true
    //     },
    //     {
    //     'title': 'Good Friday',
    //     'date': '2017-04-14',
    //     'notes': '',
    //     'bunting': false
    //     },
    //     {
    //     'title': 'Easter Monday',
    //     'date': '2017-04-17',
    //     'notes': '',
    //     'bunting': true
    //     }]")
    "hi".to_string()
}

async fn blah() -> RegionalBankHoliday {
    let requestUrl = "https://www.gov.uk/bank-holidays/england-and-wales.json";
    reqwest::get(requestUrl).await.unwrap().json::<RegionalBankHoliday>().await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}