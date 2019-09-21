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

fn handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {
    match(req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new("Hello from Rust!".into()))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            future::ok(response)
        }
    }
}
