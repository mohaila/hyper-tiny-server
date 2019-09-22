use hyper::{Body, Request, Response};

pub fn update_note(_: &Request<Body>) -> Response<Body> {
    Response::new("Update note by id".into())
}
