# axum-api-server8

## TODO
https://github.com/tokio-rs/axum

query-params-with-empty-strings
customize-path-rejection
get header request-id
stream_reqwest_response(reqwest-response)
parse-body-based-on-content-type(application/json or application/x-www-form-urlencoded)
routes-and-handlers-close-together

//anyhow-error-response
customize-extractor-error
handle-head-request
global-404-handler
error-handling

form, cors, jwt
consume-body-in-extractor-or-middleware

print-request-response
tracing-aka-logging
tls-graceful-shutdown

PostgreSQL
https://github.com/sfackler/rust-postgres
https://medium.com/@mikecode/rust-how-to-connect-to-postgresql-f39ba1497b2a
diesel-postgrestodo, diesel-async-postgres
tokio-postgres,
sqlx-postgres

auto-reload
testing
dependency-injection

Tutorials
https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials

Neon Postgres https://neon.tech/docs/guides/rust
Shuttle Postgres https://docs.shuttle.dev/resources/shuttle-shared-db
Render https://render.com/docs/deploy-rocket-rust

Other POstgres Hosts
https://medium.com/@nile.bits/top-10-affordable-options-to-host-your-postgresql-database-6cf103fe40b4


JWT, Socket, gRPC
Socketioxide: Socket.io implementation in Rust
I never thought I'd use Socket.io ever again https://www.youtube.com/watch?v=HEhhWL1oUTM

Websockets | Rust | long lived Connections 
https://www.youtube.com/watch?v=u4gV9nYEi_I
Real-Time P2P File Transfer App with Rust(Axum), WebSockets & Next.js 
https://www.youtube.com/watch?v=EYwqo3CMGsU

Tonic makes gRPC in Rust stupidly simple
https://www.youtube.com/watch?v=kerKXChDmsE

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