use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

mod notes;
use notes::*;

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

fn handler(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = Error> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::new("Hello from Rust!".into()),
        (method, path) if path.starts_with(NOTES) => {
            let id = path
                .trim_start_matches(NOTES)
                .parse::<u64>()
                .ok()
                .map(|i| i as u64);
            match (method, id) {
                // GET /notes/1234
                (&Method::GET, Some(_)) => get_note(&req),
                // GET /notes/
                (&Method::GET, None) => get_all_notes(&req),
                // POST /notes/
                (&Method::POST, None) => create_note(&req),
                // PUT /notes/1234
                (&Method::PUT, Some(_)) => update_note(&req),
                // DELETE /notes/1234
                (&Method::DELETE, Some(_)) => delete_note(&req),
                // Other verbs
                _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED),
            }
        }
        _ => response_with_code(StatusCode::NOT_FOUND),
    };
    future::ok(response)
}
