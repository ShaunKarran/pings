extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod utils {
    use std::env;

    use chrono::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use dotenv::dotenv;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    /// Parse an ISO formated date into a timestamp (epoch time).
    ///
    /// # Panics
    ///
    /// Will panic! if the passed string does not match the format `"%Y-%m-%d"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libpings::utils::parse_iso;
    ///
    /// let timestamp = parse_iso("2017-04-08");
    /// ```
    pub fn parse_iso(iso: &str) -> i64 {
        let mut iso_string = iso.to_string();
        iso_string.push_str(" 00:00:00"); // Cannot parse without time.

        let date_time = UTC.datetime_from_str(&iso_string, "%Y-%m-%d %H:%M:%S").unwrap();

        date_time.timestamp()
    }
}

pub mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

pub mod models {
    use super::schema::{devices, pings};

    #[derive(Debug, Insertable, Queryable)]
    #[table_name="devices"]
    pub struct Device {
        pub id: String
    }

    #[derive(Debug, Insertable, Queryable)]
    #[table_name="pings"]
    pub struct Ping {
        pub epoch_time: i64,
        pub device_id: String
    }
}
