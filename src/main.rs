#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
extern crate rocket;
extern crate serde_json;
extern crate time;

extern crate libpings;

use std::collections::HashMap;

use chrono::prelude::*;
use diesel::prelude::*;
use time::Duration;

use libpings::models::{Device, Ping};
use libpings::schema::{devices, pings};
use libpings::utils::{establish_connection, parse_iso};

#[post("/clear_data")]
fn clear_data() {
    let db_connection = establish_connection();

    // Delete all pings first because they refer to devices.
    diesel::delete(pings::table).execute(&db_connection).unwrap();
    diesel::delete(devices::table).execute(&db_connection).unwrap();
}

#[get("/devices")]
fn get_devices() -> String {
    let db_connection = establish_connection();

    // Get all devices from the table.
    let results = devices::table.load::<Device>(&db_connection)
        .expect("Error getting devices.");

    // Collect all the device_ids into a vector.
    let mut device_ids = Vec::new();
    for device in results {
        device_ids.push(device.id);
    }

    // Return the json representation of an array of device_ids. eg. "["id_1", "id_2"]"
    serde_json::to_string(&device_ids).unwrap()
}

#[post("/<device_id>/<epoch_time>")]
fn ping(device_id: &str, epoch_time: i64) {
    let db_connection = establish_connection();

    // Create new objects to be inserted into the database.
    let new_device = Device { id: device_id.to_string() };
    let new_ping = Ping {
        epoch_time: epoch_time,
        device_id: device_id.to_string(),
    };

    // Insert into database.
    diesel::insert(&new_device)
        .into(devices::table)
        .execute(&db_connection);
    // Currently ignoring this for the case where the device already exists. TODO: Catch other errors.
    // .expect("Error saving device.");
    diesel::insert(&new_ping)
        .into(pings::table)
        .execute(&db_connection)
        .expect("Error saving ping.");
}

#[get("/<device_id>/<date>", rank = 2)]
fn get_pings_on_date(device_id: &str, date: &str) -> String {
    let db_connection = establish_connection();

    let mut date_string = date.to_string();
    date_string.push_str(" 00:00:00"); // Cannot parse without time.

    // Create the `from` and `to` datetimes.
    let from_date_time = UTC.datetime_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();
    let to_date_time = from_date_time + Duration::days(1);

    // Convert to epoch for easy comparisons.
    let from_timestamp = from_date_time.timestamp();
    let to_timestamp = to_date_time.timestamp();

    // Query the database.
    let results = pings::table
        .filter(pings::dsl::device_id.eq(device_id))
        .filter(pings::dsl::epoch_time.ge(from_timestamp)) // >=
        .filter(pings::dsl::epoch_time.lt(to_timestamp)) // <
        .load::<Ping>(&db_connection)
        .expect("Error getting pings.");

    // Collect all the ping timestamps into a vector.
    let mut ping_epochs = Vec::new();
    for ping in results {
        ping_epochs.push(ping.epoch_time);
    }

    // Return the json representation of an array of timestamps. eg. "[1234, 4321]"
    serde_json::to_string(&ping_epochs).unwrap()
}

#[get("/<device_id>/<from>/<to>", rank = 2)]
fn get_pings_between(device_id: &str, from: &str, to: &str) -> String {
    let db_connection = establish_connection();

    // Attempt to parse as epoch_time, on failure parse as ISO format.
    let from_timestamp = match from.parse::<i64>() {
        Ok(value) => value,
        Err(_) => parse_iso(from),
    };
    let to_timestamp = match to.parse::<i64>() {
        Ok(value) => value,
        Err(_) => parse_iso(to),
    };

    // Query the database.
    let results = pings::table
        .filter(pings::dsl::device_id.eq(device_id))
        .filter(pings::dsl::epoch_time.ge(from_timestamp)) // >=
        .filter(pings::dsl::epoch_time.lt(to_timestamp)) // <
        .load::<Ping>(&db_connection)
        .expect("Error getting pings.");

    // Collect all the ping timestamps into a vector.
    let mut ping_epochs = Vec::new();
    for ping in results {
        ping_epochs.push(ping.epoch_time);
    }

    // Return the json representation of an array of timestamps. eg. "[1234, 4321]"
    serde_json::to_string(&ping_epochs).unwrap()
}

#[get("/<date>", rank = 1)]
fn get_all_on_date(date: &str) -> String {
    let db_connection = establish_connection();

    let mut date_string = date.to_string();
    date_string.push_str(" 00:00:00"); // Cannot parse without time.

    // Create the `from` and `to` datetimes.
    let from_date_time = UTC.datetime_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();
    let to_date_time = from_date_time + Duration::days(1);

    // Convert to epoch for easy comparisons.
    let from_timestamp = from_date_time.timestamp();
    let to_timestamp = to_date_time.timestamp();

    // Query the database.
    let results = pings::table
        .filter(pings::dsl::epoch_time.ge(from_timestamp)) // >=
        .filter(pings::dsl::epoch_time.lt(to_timestamp)) // <
        .load::<Ping>(&db_connection)
        .expect("Error getting pings.");

    // Collect all the ping timestamps into a vector.
    let mut ping_epochs: HashMap<String, Vec<i64>> = HashMap::new();
    for ping in results {
        // If a device_id doesnt exist yet, insert it with a new empty vector before pushing the new epoch time.
        ping_epochs.entry(ping.device_id).or_insert(Vec::new()).push(ping.epoch_time);
    }

    // Return the json representation of an hash of device_id: timestamps. eg. "{"qwe": [124, 431], "ewq": [124, 432]}"
    serde_json::to_string(&ping_epochs).unwrap()
}

#[get("/<from>/<to>", rank = 1)]
fn get_all_between(from: &str, to: &str) -> String {
    let db_connection = establish_connection();

    // Attempt to parse as epoch_time, on failure parse as ISO format.
    let from_timestamp = match from.parse::<i64>() {
        Ok(value) => value,
        Err(_) => parse_iso(from),
    };
    let to_timestamp = match to.parse::<i64>() {
        Ok(value) => value,
        Err(_) => parse_iso(to),
    };

    // Query the database.
    let results = pings::table
        .filter(pings::dsl::epoch_time.ge(from_timestamp)) // >=
        .filter(pings::dsl::epoch_time.lt(to_timestamp)) // <
        .load::<Ping>(&db_connection)
        .expect("Error getting pings.");

    // Collect all the ping timestamps into a vector.
    let mut ping_epochs: HashMap<String, Vec<i64>> = HashMap::new();
    for ping in results {
        // If a device_id doesnt exist yet, insert it with a new empty vector before pushing the new epoch time.
        ping_epochs.entry(ping.device_id).or_insert(Vec::new()).push(ping.epoch_time);
    }

    // Return the json representation of an hash of device_id: timestamps. eg. "{"qwe": [124, 431], "ewq": [124, 432]}"
    serde_json::to_string(&ping_epochs).unwrap()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![clear_data, get_devices, ping, get_pings_on_date, get_pings_between])
        .mount("/all", routes![get_all_on_date, get_all_between])
        .launch();
}
