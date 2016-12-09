mod jwtrs;

#[macro_use]
extern crate mime;
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate rustc_serialize;
extern crate serde;

use jwtrs::auth;
use jwtrs::home;

use iron::prelude::*;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", home::index, "home");
    router.post("/auth", auth::handle, "auth");

    let _server = Iron::new(router).http("localhost:9000").unwrap();
    println!("Starting server on localhost:9000...");
}
