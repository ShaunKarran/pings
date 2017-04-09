extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

/// Utility functions for the pings application.
pub mod utils {
    use std::env;

    use chrono::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use dotenv::dotenv;

    /// Establish a connection to the database specified in the `"DATABASE_URL"` env variable.
    ///
    /// # Panics
    ///
    /// Will panic! if the `"DATABASE_URL"` variable is not set or a connection cannot be established.
    ///
    /// # Examples
    ///
    /// ```
    /// use libpings::utils::establish_connection;
    ///
    /// let db_connection = establish_connection();
    /// ```
    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
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
    /// use libpings::utils::parse_iso_date;
    ///
    /// let timestamp = parse_iso_date("2017-04-08".to_string());
    /// ```
    pub fn parse_iso_date(mut iso_date: String) -> i64 {
        iso_date.push_str(" 00:00:00"); // Cannot parse without time.
        let date_time = UTC.datetime_from_str(&iso_date, "%Y-%m-%d %H:%M:%S").unwrap();

        date_time.timestamp()
    }
}

/// Infers the schema of the connected database.
///
/// # Examples
///
/// ```
/// use libpings::schema::pings;
/// ```
pub mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

pub mod models {
    use super::schema::{devices, pings};

    #[derive(Debug, Insertable, Queryable)]
    #[table_name="devices"]
    pub struct Device {
        pub id: String,
    }

    #[derive(Debug, Insertable, Queryable)]
    #[table_name="pings"]
    pub struct Ping {
        pub epoch_time: i64,
        pub device_id: String,
    }
}
