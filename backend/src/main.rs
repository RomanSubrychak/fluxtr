#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/hello")]
fn hello() -> &'static str {
    "Hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
}
