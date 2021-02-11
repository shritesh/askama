#[macro_use]
extern crate rocket;

use askama::Template;

use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

#[test]
fn test_rocket() {
    let rocket = rocket::ignite().mount("/", routes![hello]);
    let client = Client::tracked(rocket).unwrap();
    let rsp = client.get("/").dispatch();
    assert_eq!(rsp.status(), Status::Ok);
    assert_eq!(rsp.content_type(), Some(ContentType::HTML));
    assert_eq!(rsp.into_string(), Some("Hello, world!".into()));
}
