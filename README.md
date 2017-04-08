# pings
My implementation of the [pings challenge](https://github.com/TandaHQ/work-samples/tree/master/pings%20(backend)) for Tanda.

### Dependencies
- Latest nightly release of the [Rust Programming Language](https://www.rust-lang.org/en-US/install.html).
- [PostgreSQL](https://www.postgresql.org) installed and running.

### Instructions
- Install the Diesel CLI with `cargo install diesel_cli`
- Update the username and password in the `.env` file.
- Run `diesel setup` to get Diesel to create the database.
- Run migrations with `diesel migration run`
- Run pings using `cargo run`.
