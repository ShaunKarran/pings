#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket;
extern crate serde_json;

extern crate libpings;

use diesel::prelude::*;

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
        device_ids.push(device.id)
    }

    // // Return the json representation of an array of device_ids. eg. "["id_1", "id_2"]"
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

fn main() {
    rocket::ignite().mount("/", routes![clear_data, get_devices, ping]).launch();
}
