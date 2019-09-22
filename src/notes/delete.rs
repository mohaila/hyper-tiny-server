use hyper::{Body, Request, Response};

pub fn delete_note(_: &Request<Body>) -> Response<Body> {
    Response::new("Delete note by id".into())
}
