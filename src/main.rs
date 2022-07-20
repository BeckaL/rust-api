#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;

use rocket::response::content;
use std::collections::HashMap;
use std::error::Error;

struct BankHoliday {
    title: String,
    // TODO: make date a special date class
    date: String,
    notes: String,
    bunting: bool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> content::RawJson<&'static str> {
    content::RawJson("[{
        'title': 'New Years Day',
        'date': '2017-01-02',
        'notes': 'Substitute day',
        'bunting': true
        },
        {
        'title': 'Good Friday',
        'date': '2017-04-14',
        'notes': '',
        'bunting': false
        },
        {
        'title': 'Easter Monday',
        'date': '2017-04-17',
        'notes': '',
        'bunting': true
        }]")
}

// fn blah() -> () {
//     let requestUrl = "https://www.gov.uk/bank-holidays/england-and-wales.json";
//     let body = reqwest::get(requestUrl).await?.text().await?;

//     println!("body {:?}", body);
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}