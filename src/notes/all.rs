use hyper::{Body, Request, Response};

pub fn get_all_notes(_: &Request<Body>) -> Response<Body> {
    Response::new("Get all notes".into())
}
