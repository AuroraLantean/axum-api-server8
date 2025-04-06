# axum-api-server8

## TODO
https://github.com/tokio-rs/axum

Build a CRUD REST API with Rust Axum | Tutorial
https://www.youtube.com/watch?v=NJsTgmayHZY&t=21s

Rust Axum Full Course - Web Development (GitHub repo updated to Axum 0.7)
https://www.youtube.com/watch?v=XZtlD_m59sM

Brook Build: Updating the Axum course to v0.8.1
https://www.youtube.com/watch?v=JCPKGYD3vJk&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS&index=64

Tutorials:
https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials

Using Rust, Axum, PostgreSQL, and Tokio to build a Blog
https://spacedimp.com/blog/using-rust-axum-postgresql-and-tokio-to-build-a-blog/

Rust Axum Full Course: 
https://www.youtube.com/watch?v=XZtlD_m59sM

Tonic makes gRPC in Rust stupidly simple
https://www.youtube.com/watch?v=kerKXChDmsE

Socket.io Socketioxide
https://www.youtube.com/watch?v=HEhhWL1oUTM

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