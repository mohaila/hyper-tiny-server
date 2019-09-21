use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

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
                (&Method::GET, Some(_)) => Response::new("Get note by id".into()),
                // GET /notes
                (&Method::GET, None) => Response::new("Get all notes".into()),
                // POST /notes
                (&Method::POST, None) => Response::new("Create note".into()),
                // PUT /notes/1234
                (&Method::PUT, Some(_)) => Response::new("Update note by id".into()),
                // DELETE /notes/1234
                (&Method::DELETE, Some(_)) => Response::new("delete note by id".into()),
                // Other verbs
                _ => response_with_code(StatusCode::METHOD_NOT_ALLOWED),
            }
        }
        _ => response_with_code(StatusCode::NOT_FOUND),
    };
    future::ok(response)
}

const NOTES: &str = "/notes/";
