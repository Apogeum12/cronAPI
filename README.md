# cron_api

# Table of Contents

- [Description cron_api](#cron-api-cluster)
- [Dependencies](#dependencies)
- [Compilation](#compilation)
- [Run Application](#run-application)
- [Application](#application)
- [Generate Documentation](#generate-documentation)
- [Endpoints](#endpoints)
- - [POST Run](#post-run)
- - [POST harmonogram](#post-harmonogram)

# Cron API Cluster

This is a Cluster API application for run services. We use it to run list of programs in cluster on independend threads.

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

1. Go into "application/" folder and run binary as:

```sh
$ sudo ./cron_api
```

2. After the compilation is finished, it should create a folder named `application/` with our application and `.env` static file. (I specifically did not build them inside the binary). We need run as root, because we'll create, and modify crontab.

## Application

We start the application by entering the `application/` folder and typing `./cron_api` in the console. We should get a log like this:

```sh
[2023-12-10T22:59:47Z INFO  cron_api] Cron service is already running as root.
[2023-12-10T22:59:47Z INFO  cron_api] Starting server in "0.0.0.0:1726" with 10 threads
[2023-12-10T22:59:47Z INFO  actix_server::builder] starting 10 workers
[2023-12-10T22:59:47Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime

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
    "services": ["service1", "...", "serviceN"],
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
  "time": "* * * *",
  "thread": 2,
  "scrapers": ["service1", "service2"]
}
```

- where:
  - time: `string`
  - thread: `number`
  - scrapers: `string[]`
