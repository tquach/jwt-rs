//! Home module handles HTTP requests for the `/` endpoint.
use iron::prelude::*;
use iron::status;

/// index handles requests to the root path.
pub fn index(_: &mut Request) -> IronResult<Response> {
    let content_type = mime!(Application / Json);
    Ok(Response::with((content_type, status::Ok, "{\"status\": \"OK\"}")))
}
