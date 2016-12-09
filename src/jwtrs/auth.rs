//! Auth module handles HTTP requests for the `/auth` endpoint.
use iron::prelude::*;
use iron::status;
use jwtrs::jwt::Jwt;

use rustc_serialize::json;

use bodyparser;

/// Handle POST requests that include a JSON body.
pub fn handle(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Struct<Jwt>>();
    let content_type = mime!(Application / Json);
    let result = Jwt {
        secret_key: "".to_string(),
        token: "".to_string(),
    };

    let payload = json::encode(&result).unwrap();

    match json_body {
        Ok(Some(json_body)) => {
            println!("OK: {:?}", json_body);
        }
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {}", err),
    }
    Ok(Response::with((content_type, status::Ok, payload)))
}
