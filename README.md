# Demo-App in Rust

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
- File > New > Project.. > Rust, set location to where you cloned the repository
- click the play button
- first run takes minutes due to cargo sync and compiling, consecutive runs are faster due to incremental compilation :)
- test app
  - open `http://localhost:8080/users` in Browser
  - or open `src/test/http_requests.http` in RustRover and edit and run requests from the IDE
  - browse db using the IDE's database tab