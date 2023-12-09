# cron_api

# Spis treści

- [Description cron_api](#cron-api-cluster)
- [Installation of Rust](#installation-of-rust)
- - [MacOS](#macos)
- - [Linux](#linux)
- [Dependencies](#dependencies)
- [Compilation](#compilation)
- [Run Application](#run-application)
- [Application](#application)
- [Generate Documentation](#generate-documentation)
- [Endpoints](#endpoints)
- - [Post Run](#post-run)
- - [POST harmonogram](#post-harmonogram)

# Cron API Cluster

TODO: This is a Cluster API application for run services. We use it to run list of programs in cluster on independend threads. In the following steps, I will assume that the repository is already cloned and that we are in the project folder `cron_api`.

## Installation of Rust

### MacOS

1. Open Terminal.
2. Install _rustup_, a tool for installing Rust, by typing: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
3. Close and reopen the terminal.
4. Type `rustc --version`. You should see the version of the Rust compiler.

### Linux

1. Open Terminal.
2. Install _rustup_, a tool for installing Rust, by typing: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
3. Close and reopen the terminal.
4. Type `rustc --version`. You should see the version of the Rust compiler.

## Dependencies

After installing Rust, let's download cargo-make for easier application building.

1. Type in the terminal:

```sh
$ cargo install cargo-make
```

## Compilation

1. When cargo-make is installed, we can compile the project:

```sh
$ cargo make build
```

## Run Application

1. Go into application folder and run binary as:

```sh
$ ./cron_api
```

2. After the compilation is finished, it should create a folder named `cron_api/` with our application and `.env` static file. (I specifically did not build them inside the binary).

## Application

We start the application by entering the `cron_api/` folder and typing `./cron_api` in the console. We should get a log like this:

```sh
[2023-07-03T10:29:27Z INFO  cron_api] Starting server in "0.0.0.0:1726" with 10 threads
[2023-07-03T10:29:27Z INFO  actix_server::builder] starting 10 workers
[2023-07-03T10:29:27Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime

```

The number of threads depends on the processor from which we launch the API, i.e., the available `number of threads - 2`. For a 6-core 12-thread processor, this will be `10` threads available for the API handler.

## Generate Documentation

1. For generate and run docuemntation, command in console:

```sh
$ cargo doc --no-deps
```

2. After finished compile docs, go to the folder `target/doc/cron_api/` and open `index.html` file with your browser.

# Endpoints

## POST Run

#### Type [`POST`]: `/run`

1. Endpoint who we use to starting cluster. Incoming object have to be that struct,:

```json
{
    "thread": 2,
    "services": ["service1", ..., "serviceN"],
}
```

- where:
  - thread: `number`
  - services: `string[]`

## POST harmonogram

#### Type [`POST`]: `/harmonogram`

1. Endpoint to handle harmonogram data. It save harmonogram to file and load it to cron, then current harmonogram is sent to Endpoint on other API to save in DB.

```json
{
  "id": "ID",
  "time": "* * * *",
  "thread": 2,
  "scrapers": ["scraper1", "scraper2"]
}
```

- where:
  - id: `string`
  - time: `string`
  - thread: `number`
  - scrapers: `string[]`
