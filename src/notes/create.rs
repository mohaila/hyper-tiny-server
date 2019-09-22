use hyper::{Body, Request, Response};

pub fn create_note(_: &Request<Body>) -> Response<Body> {
    Response::new("Create note".into())
}
