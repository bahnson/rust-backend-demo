# Demo-App in Rust

Small Demo 
- created with [RustRover IDE](https://www.jetbrains.com/de-de/rust/)
- using:
  - [tokio](https://github.com/tokio-rs/tokio) and [axum](https://github.com/tokio-rs/axum) for API server
  - [serde](https://github.com/serde-rs/serde) for JSON de-/serialization
  - [sqlx](https://github.com/launchbadge/sqlx) for accessing local postgres db

## Setup
- Windows Subsystem Linux (WSL), Ubuntu-24.04 in my case
- Docker Desktop with WSL integration enabled

## Run
- in a WSL-Terminal, run:
  - `sudo apt install postgresql-client-16`
  - `cd scripts && ./start-db.sh`
- in RustRover:
  - build and start app
  - open `src/test/test_requests.http` and run requests