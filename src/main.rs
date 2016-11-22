mod jwtrs;

extern crate iron;
extern crate router;
extern crate bodyparser;

use jwtrs::auth;

use iron::prelude::*;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.post("/auth", auth::handle, "auth");

    let _server = Iron::new(router).http("localhost:9000").unwrap();
    println!("Starting server on localhost:9000...");
}
