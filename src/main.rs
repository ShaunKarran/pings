#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate diesel;
extern crate rocket;
extern crate serde_json;
extern crate time;

extern crate libpings;

use chrono::prelude::*;
use diesel::prelude::*;
use time::Duration;

use libpings::utils::establish_connection;
use libpings::models::{Device, Ping};

#[post("/clear_data")]
fn clear_data() {
    // Get the variables that refer to the tables.
    use libpings::schema::{devices, pings};

    let db_connection = establish_connection();

    // Delete all pings first because they refer to devices.
    diesel::delete(pings::table).execute(&db_connection).unwrap();
    diesel::delete(devices::table).execute(&db_connection).unwrap();
}

#[get("/devices")]
fn get_devices() -> String {
    // Get the variables that refer to the tables.
    use libpings::schema::devices;

    let db_connection = establish_connection();

    // Get all devices from the table.
    let results = devices::table
        .load::<Device>(&db_connection)
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
    // Get the variables that refer to the tables.
    use libpings::schema::{devices, pings};

    let db_connection = establish_connection();

    // Create new objects to be inserted into the database.
    let new_device = Device {
        id: device_id.to_string()
    };
    let new_ping = Ping {
        epoch_time: epoch_time,
        device_id: device_id.to_string()
    };

    // Insert into database.
    diesel::insert(&new_device).into(devices::table)
        .execute(&db_connection);
        // Currently ignoring this for the case where the device already exists. TODO: Catch other errors.
        // .expect("Error saving device.");
    diesel::insert(&new_ping).into(pings::table)
        .execute(&db_connection)
        .expect("Error saving ping.");
}

#[get("/<device_id>/<date>")]
fn get_pings_on_date(device_id: &str, date: &str) -> String {
    // Get the variables that refer to the tables.
    use libpings::schema::{devices, pings};

    let db_connection = establish_connection();

    let mut date_string = date.to_string();
    date_string.push_str(" 00:00:00"); // Cannot parse without time.

    // Create the `from` and `to` datetimes.
    let from_date_time = UTC.datetime_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();
    let to_date_time = from_date_time + Duration::days(1);

    // Convert to epoch for easy comparisons.
    let from_timestamp = from_date_time.timestamp();
    let to_timestamp = to_date_time.timestamp();

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

#[get("/<device_id>/<from>/<to>")]
fn get_pings_between(device_id: &str, from: &str, to: &str) -> String {
    unimplemented!();
}

#[get("/all/<date>")]
fn get_all_on_date(date: &str) -> String {
    unimplemented!();
}

#[get("/all/<from>/<to>")]
fn get_all_between(from: &str, to: &str) -> String {
    unimplemented!();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![clear_data, get_devices, ping, get_pings_on_date, get_pings_between])
        .launch();
}
