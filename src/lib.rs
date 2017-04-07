#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod utils {
    use std::env;

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
