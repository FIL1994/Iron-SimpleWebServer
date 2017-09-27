extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

use mount::Mount;
use router::Router;
use staticfile::Static;

use std::path::Path;

///Main function
fn main() {
    let mut router = Router::new();
    router
        .get("/hello", say_hello, "hello")
        .get("/", handler, "index")
        .get("/:query", handler, "query");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/docs/", Static::new(Path::new("target/doc")));

    Iron::new(mount).http("localhost:3000").unwrap();
}

fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn handler(req:&mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap()
        .find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}