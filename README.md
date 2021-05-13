# ![RealWorld Example App](logo.png)

> ### [Actix](https://actix.rs/) codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

❗ (2021/05/13) This codebase is currently unmaintained, and I am not interested in maintaining it. This relies on an old version of Actix -- developers who want to learn Actix should probably read the latest docs at the [Actix website](https://actix.rs/).

This codebase was created to demonstrate a fully fledged fullstack application built with [Actix](https://actix.rs/) including CRUD operations, authentication, routing, pagination, and more. CORS, however, is not yet added.

This implementation is not reviewed. See the [Contributing](#contributing) section below.

For more information on how this works with other frontends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

## How it works

This is an application written in [Rust](https://www.rust-lang.org/) that utilizes [Actix](https://actix.rs/) for developing the backend web service that powers the RealWorld application.

You can view a full list of crates being used in [Cargo.toml](./Cargo.toml), but here are some of the main ones of note:

* [Actix](https://actix.rs/) - a powerful Actor framework
* [Chrono](https://github.com/chronotope/chrono) - a Date and Time library for Rust
* [Failure](https://rust-lang-nursery.github.io/failure/) - a system for creating and managing errors in Rust
* [Futures](https://docs.rs/futures/0.1.25/futures/) - Zero-cost Futures in Rust
* [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - Create and parses JWT (JSON Web Tokens)
* [libreauth](https://github.com/breard-r/libreauth) - a collection of tools for user authentication
* [Serde](https://serde.rs/) - a framework for serializing and deserializing Rust data structures efficiently and generically
* [Uuid](https://github.com/uuid-rs/uuid) - Generate and parse UUIDs
* [validator](https://github.com/Keats/validator) - Simple validation for Rust structs

## Getting started

* Install [Rust](https://www.rust-lang.org/)
* Install [PostgreSQL](https://www.postgresql.org/) if you don't have it already.
* Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.
* Clone this repo to a folder on your computer.
* Copy (`cp`) [.env.example](./.env.example) to `.env` within this directory, and change the environment variables accordingly to your system.
* Setup your database by running `diesel database setup`. Make sure it has completed successfully.
* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.
* Run with `cargo run`.
* The API URL will be whatever the `BIND_ADDRESS` value is in `.env` with the `/api` path included e.g. `https://127.0.0.1:3000/api`. Set it as such in your REST client ([Postman](https://www.getpostman.com/), [Insomnia](https://insomnia.rest/), etc.), import the [postman collection](https://github.com/gothinkster/realworld/blob/master/api/Conduit.postman_collection.json) and start testing it out!

## Contributing

Feel free to take a look at the current issues in this repo for anything that currently needs to be worked on.

You are also welcome to open a new issue if you see something is missing or could be improved upon.
