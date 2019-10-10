# rust-fullstack-example
Example of Rust-Actix server with Rust-Seed frontend.

* Seed Frontend from MartinKavik (https://github.com/MartinKavik/seed-rs-realworld)
* Actix Server from fairingrey (https://github.com/fairingrey/actix-realworld-example-app)

Both built to conform to RealWorld Example (https://github.com/gothinkster/realworld)

> Note: MartinKavik and fairingrey have put together a couple of great examples. My purpose for this repo was to simplify an example for those getting started. The Seed frontend didn't appear in the RealWorld example and the Actix didn't have a frontend... Voila!

The only edit to code is to set the API_URLs to the correct local destinations and to allow the server to receive requests.

Directions for the initial setup/getting started for each can be found at the links above.
They are also copied below as they appeared when pulled and in the order to run.

## Server

### Getting started

* Install [Rust](https://www.rust-lang.org/)
* Install [PostgreSQL](https://www.postgresql.org/) if you don't have it already.
* Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.
* Clone this repo to a folder on your computer.
* Copy (`cp`) [.env.example](./.env.example) to `.env` within this directory, and change the environment variables accordingly to your system.
* Setup your database by running `diesel database setup`. Make sure it has completed successfully.
* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.
* Run with `cargo run`.
* The API URL will be whatever the `BIND_ADDRESS` value is in `.env` with the `/api` path included e.g. `https://127.0.0.1:3000/api`. Set it as such in your REST client ([Postman](https://www.getpostman.com/), [Insomnia](https://insomnia.rest/), etc.), import the [postman collection](https://github.com/gothinkster/realworld/blob/master/api/Conduit.postman_collection.json) and start testing it out!

## Frontend

### Getting started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Update Rust: `$ rustup update`
3. Install WASM target: `$ rustup target add wasm32-unknown-unknown`
4. Install [cargo-make](https://sagiegurari.github.io/cargo-make/): `$ cargo install --force cargo-make`
5. Build project from its root: `$ cargo make all` or `$ cargo make watch`
   - _Note:_ You need some dependencies like `pkg-config` on Linux - just follow recommendations after compilation errors.
6. Start local server: `$ cargo make serve`
   - _Note:_ You have to open a new terminal tab/window if you used `$ cargo make watch`.
7. Open in you browser [localhost:8000](http://localhost:8000/)
8. NOTE (not in original): You should see an 'Error loading feed' in body as your server has no data at this point... Go to seed-rs-realworld/src/request.rs and look at line 20. Change the local address for BASE_API_URL to the external conduit address. Then rerun 5 & 6 above.
