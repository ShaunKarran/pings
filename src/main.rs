#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

// TODO: Replace this json file with database.
fn read_json_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect(&format!("could not open {}", file_name));
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

#[post("/clear_data")]
fn clear_data() {
    let mut file = File::create("pings.json").expect(&format!("could not open {}", "pings.json"));
    file.write_all("{}\n".as_bytes()).unwrap();
}

#[get("/devices")]
fn devices() -> String {
    // Read the whole file into a string.
    let data = read_json_file("pings.json");

    // Deserialise into object.
    let json: HashMap<String, Vec<u64>> = serde_json::from_str(&data).unwrap();

    // Collect all the device_ids into a vector.
    let device_ids = json.keys().collect::<Vec<_>>();

    // Return the json representation of an array of device_ids. eg. "["id_1", "id_2"]"
    serde_json::to_string(&device_ids).unwrap()
}

#[post("/<device_id>/<epoch_time>")]
fn ping(device_id: &str, epoch_time: u64) {
    // Read the whole file into a string.
    let data = read_json_file("pings.json");

    // Deserialise into object.
    let mut json: HashMap<String, Vec<u64>> = serde_json::from_str(&data).unwrap();

    // If a device_id doesnt exist yet, insert it with a new empty vector before pushing the new epoch time.
    json.entry(device_id.to_string()).or_insert(Vec::new()).push(epoch_time);

    // Serialise the updated json object.
    let new_data = serde_json::to_string(&json).unwrap();

    let mut file = File::create("pings.json").expect(&format!("could not open {}", "pings.json"));
    file.write_all(new_data.as_bytes()).unwrap();
}

fn main() {
    rocket::ignite().mount("/", routes![clear_data, devices, ping]).launch();
}

// ----------- Iron stuff.
// extern crate iron;
// extern crate router;

// use iron::prelude::*;
// use iron::status;
// use router::Router;

// fn clear_data(_: &mut Request) -> IronResult<Response> {
//     println!("Clearing data...");
//     Ok(Response::with(status::Ok))
// }

// fn get_devices(_: &mut Request) -> IronResult<Response> {
//     println!("Getting devices...");
//     let devices = vec!("5581db36-57e7-4274-a36d-0c105c70fbfa", "5225a416-3394-4e9f-9d97-e371d7615197");
//     let devices_json = serde_json::to_string(&devices).unwrap();
//     Ok(Response::with((status::Ok, devices_json)))
// }

// fn main() {
//     let mut router = Router::new();

//     router.post("/clear_data", clear_data, "clear_data");
//     router.get("/devices", get_devices, "devices");

//     Iron::new(router).http("localhost:3000").unwrap();
// }

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
