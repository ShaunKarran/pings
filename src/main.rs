extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

// fn handler(req: &mut Request) -> IronResult<Response> {
//     let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
//     Ok(Response::with((status::Ok, *query)))
// }

fn clear_data(req: &mut Request) -> IronResult<Response> {
    println!("Clearing data...");
    Ok(Response::with(status::Ok))
}

fn get_devices(req: &mut Request) -> IronResult<Response> {
    println!("Getting devices...");
    // let test_devices = Vec::new("5581db36-57e7-4274-a36d-0c105c70fbfa", "5225a416-3394-4e9f-9d97-e371d7615197");
    // TODO: Response body should be plain text in JSON format.
    // eg. ["5581db36-57e7-4274-a36d-0c105c70fbfa","5225a416-3394-4e9f-9d97-e371d7615197"]
    let test_devices = "[\"5581db36-57e7-4274-a36d-0c105c70fbfa\",\"5225a416-3394-4e9f-9d97-e371d7615197\"]";
    Ok(Response::with((status::Ok, test_devices)))
}

fn main() {
    let mut router = Router::new();

    router.post("/clear_data", clear_data, "clear_data");
    router.get("/devices", get_devices, "devices");

    Iron::new(router).http("localhost:3000").unwrap();
}

// ----------- Hyper stuff.
// extern crate hyper;

// use hyper::method::Method::{Get, Post};
// use hyper::server::{Server, Request, Response};
// use hyper::status::StatusCode;

// fn clear_data() {
//     println!("Clearing data...");
// }

// fn get_devices() -> Vec<String> {
//     println!("Getting devices...");

//     Vec::new()
// }

// fn handler(req: Request, mut res: Response) {
//     // let (sock_addr, method, headers, request_uri, http_version, http_reader) = req.deconstruct();

//     match (req.method, &req.uri.to_string()[..]) {
//         (Post, "/clear_data") => clear_data(),
//         (Get, "/devices") => {
//             get_devices()
//         },
//         _ => println!("You must never go here, Simba.")
//     }

//     *res.status_mut() = StatusCode::Ok;
// }

// fn main() {
//     Server::http("localhost:3000").unwrap().handle(handler).unwrap();
// }
