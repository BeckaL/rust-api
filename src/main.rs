use rocket::response::content;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> content::RawJson<&'static str> {
    // content::RawJson("{'hi': 'world'}")
    content::RawJson("{'title': 'New Years Day', 'date': '2017-01-02', 'notes': 'Substitute day', 'bunting': true}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}