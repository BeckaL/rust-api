use rocket::response::content;
#[macro_use] extern crate rocket;
#[macro_use] extern crate reqwest;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> content::RawJson<&'static str> {
    blah();


    // match body {
    //     Ok(jsonBody) => content::RawJson(&jsonBody),
    //     Err(_) => content::RawJson("{ 'message': 'uh-oh' }"),
    // }
    //
    content::RawJson("{ 'hello' : 'world' }")
}

fn blah() -> () {
    let requestUrl = "https://www.gov.uk/bank-holidays/england-and-wales.json";
    let body = reqwest::get(requestUrl).await?.text().await?;

    println!("body {:?}", body);
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}