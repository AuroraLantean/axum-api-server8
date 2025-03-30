# axum-api-server8

## TODO
Tutorials:
https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials

Using Rust, Axum, PostgreSQL, and Tokio to build a Blog
https://spacedimp.com/blog/using-rust-axum-postgresql-and-tokio-to-build-a-blog/

Rust Axum Full Course: 
https://www.youtube.com/watch?v=XZtlD_m59sM

Brook Build: Updating the Axum course to v0.8.1
https://www.youtube.com/watch?v=JCPKGYD3vJk&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS&index=64

Deployment with Shuttle
https://docs.shuttle.dev/examples/axum

## Installation
rustc 1.85.1 (4eb161250 2025-03-15)

```
cargo add axum tokio --features tokio/full
cargo install cargo-watch
```

## Run
Start Terminator: `just term`

Run the server: `just watch`

Start Slumber to make requests: `just httpclient`