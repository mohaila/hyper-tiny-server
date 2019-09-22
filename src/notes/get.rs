use hyper::{Body, Request, Response};

pub fn get_note(_: &Request<Body>) -> Response<Body> {
    Response::new("Get note by id".into())
}
