# axum-api-server8

## Postgres Setup
```
$ docker volume create postgres1-data
$ docker run --name postgres1 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=admin -e POSTGRES_DB=db_name1 -d -p 5431:5432 -v postgres1-data:/var/lib/postgresql/data postgres:latest

$ docker stop postgres1
$ docker rm postgres1

$ docker run --name postgres1 ...
$ docker start postgres1
$ docker ps -a

// Add table inside Docker Postgres one line at a time
$ docker exec -it postgres1 psql -U postgres

postgres=# \dt  ... find all tables
postgres=# CREATE DATABASE "db_name1";
postgres=# \c db_name1  ... to connect

db_name1=# CREATE TABLE users(
id SERIAL PRIMARY KEY,
name VARCHAR(255),
password VARCHAR(255),
occupation VARCHAR(255),
email VARCHAR(255),
phone VARCHAR(20)
);
db_name1=# SELECT * FROM users;
postgres=# \q

$ docker stop postgres1
$ docker rm postgres1
$ docker ps -a
```

## TODO
https://github.com/tokio-rs/axum

https://medium.com/@mikecode/rust-axum-create-user-log-in-user-crypt-password-verify-password-connect-to-database-69c65a3c10b4

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
install terminator

install slumber http client

install just command runner

install Rust 1.87.0 (17067e9ac 2025-05-09)

```
cargo install cargo-watch
```

## Run
Start Terminator: `just term`

Run the server: `just watch`

Start Slumber to make requests: `slumber`