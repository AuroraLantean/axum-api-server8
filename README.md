# axum-api-server8

## Why Axum
Go with axum, i's a lot easier to get into and the performance difference is negligible.

If you’re doing anything that involves a database, that timing is going to dominate your API performance.

Other consideration:
- Number of production apps using each library
- Maintenance story: who is fixing bugs, improving the library and how frequently?
- API ergonomics

Actix uses a pool of single thread tokio runtimes and axum just uses the multithread runtime. The multithread runtime does add a litte big overhead but in situations where the endpoints in the backend do generate not even loads when executing that can be better then a bunch of single thread runtimes. 

Using actix-web adds complications because it is using its own actix-rt runtime. It is based on tokio but it does its own thing with threads which may cause some incompatibilities with other projects. Libraries like sqlx and sea-orm have feature flags to use this runtime but most other projects typically just support tokio only. You can run actix-web under the tokio runtime but then you lose support for actix actors and websockets.

## Postgres Setup
### Docker Compose automatic setup
```
$ docker compose -f db/dbContainer.yaml up
... check there is no error running the seed file

$ docker exec -it postgres1 psql -U postgres
postgres=# \l   ... find list of databases
... confirm db_name1 exists

postgres=# \c db_name1
postgres=# \dt  ... find all tables
... confirm the table generated from your seed file
postgres=# SELECT * FROM users;
```


### Manual setup via CLI
```
$ docker volume create postgres1-data
$ docker run --name postgres1 -e POSTGRES_PASSWORD=password -e POSTGRES_USER=admin -e POSTGRES_DB=db_name1 -d -p 5431:5432 -v postgres1-data:/var/lib/postgresql/data postgres:latest

// Add table inside Docker Postgres one line at a time
$ docker exec -it postgres1 psql -U postgres

postgres=# CREATE DATABASE "db_name1";
postgres=# \l   ... find list of databases
... confirm db_name1 exists

postgres=# \c db_name1  ... to connect
db_name1=# CREATE TABLE users(
id SERIAL PRIMARY KEY,
name VARCHAR(255) UNIQUE NOT NULL,
password VARCHAR(255) NOT NULL,
occupation VARCHAR(255),
email VARCHAR(255) UNIQUE,
phone VARCHAR(32) UNIQUE,
priority INT,
balance Numeric(26, 18),
updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
db_name1=# SELECT * FROM users;

postgres=# \dt  ... find all tables
... confirm the table generated from your seed file

postgres=# \dt  ... find all tables
postgres=# \q
```

## Generate SeaORM Entity from Database
```
$ cargo install sea-orm-cli
$ sea-orm-cli generate entity \
    -u [YOUR POSTGRES SQLX URL] \
    -o src/entities
```
in the Entity file, add `use serde::{Deserialize, Serialize};` to that Model; And implement `IntoResponse`


## Docker Container Management
```
$ docker stop postgres1
$ docker start postgres1
$ docker ps -a

$ docker stop postgres1
$ docker rm postgres1
$ docker ps -a
```

## Add Rust Dependencies
async-trait: Type erasure for async trait methods 
axum and axum-extra: web framework built with Tokio, Tower, and Hyper; Extra utilities for Axum

rust_decimal: included in SeaORM
chrono: for DB timestamps ... included in SeaORM
//time: to compliment chronos

dotenvy: A well-maintained fork of the dotenv
jsonwebtoken: to make json web token
lettre: sending emails
serde and serde_json: to serialize and deserialize

sqlx: An async, pure Rust SQL toolkit featuring compile-time checked queries without a DSL. Supports PostgreSQL, MySQL, and SQLite.

tokio: async runtime from timer to networking
tower: networking clients and servers
tracing-subscriber: for collecting structured event logs
uuid: for unique identifier
validator: validation for data models

## Reference
SeaORM https://github.com/SeaQL/sea-orm
Tokio https://github.com/tokio-rs/axum


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

## TODO
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

consume-body-in-extractor-or-middleware

print-request-response
tracing-aka-logging
tls-graceful-shutdown

Rust Postgres Driver
https://github.com/sfackler/rust-postgres

diesel-postgrestodo, diesel-async-postgres
tokio-postgres,
sqlx-postgres

testing
dependency-injection

Tutorials
https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials

Neon Postgres https://neon.tech/docs/guides/rust
Shuttle Postgres https://docs.shuttle.dev/resources/shuttle-shared-db
Render https://render.com/docs/deploy-rocket-rust

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
