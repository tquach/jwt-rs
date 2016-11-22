//! Auth module handles HTTP requests for the `/auth` endpoint.
use iron::prelude::*;
use iron::status;
use bodyparser;

/// Handle POST requests that include a JSON body.
pub fn handle(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => println!("OK: {:?}", json_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {}", err),
    }

    Ok(Response::with(status::Ok))
}
