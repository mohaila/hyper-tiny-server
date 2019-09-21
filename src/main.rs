use hyper::service::service_fn;
use hyper::{Body, Response, Server, Request, Error, Method, StatusCode};
use futures::{future, Future};

fn main() {
    let addr = ([0, 0, 0, 0], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| service_fn(handler));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

fn response_with_code(code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(code)
        .body(Body::empty())
        .unwrap()
}

fn handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {
    let response = 
        match(req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                Response::new("Hello from Rust!".into())
            },
            _ => {
                response_with_code(StatusCode::NOT_FOUND)
            }
        };
    future::ok(response)
}
