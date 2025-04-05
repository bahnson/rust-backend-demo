# rust-backend-demo

Small Demo showcasing api server development using [Rust](https://www.rust-lang.org/) and [RustRover IDE](https://www.jetbrains.com/de-de/rust/).

The code uses:
- [tokio](https://github.com/tokio-rs/tokio) and [axum](https://github.com/tokio-rs/axum) for API server
- [serde](https://github.com/serde-rs/serde) for JSON de-/serialization
- [sqlx](https://github.com/launchbadge/sqlx) for accessing a local postgres db


## Requirements
- Bash / GitBash
- Docker for running postgres db
- [RustRover IDE](https://www.jetbrains.com/de-de/rust/) (comes with Rust, cargo installation)

## Setup
- `git clone https://github.com/bahnson/rust-backend-demo.git`
-  in a bash, in working directory `local-db`, run `./start-db.sh`
    - starts postgres db in a container 
    - `Dockerfile` copies `init.sql` in container image for initializing db

## Run

- open RustRover
- File > New > Project.. > Rust, set location to where you cloned the repository,  confirm to create from existing sources
- wait for cargo project to synchronize, can take minutes, see progress in build tab
- edit the run configuration `Run rust-backend-demo`, set environment variables to `DATABASE_URL=postgres://postgres:password@localhost/postgres`
- click the play button
- first run takes minutes for compiling, consecutive runs are much faster
- test app
  - open `http://localhost:8080/users` in Browser
  - or open `test/http_requests.http` in RustRover and edit and run requests from the IDE
  - browse db using the IDE's database tab
    - URL: `jdbc:postgresql://localhost:5432/postgres`
    - password `password`