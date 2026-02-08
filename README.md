
# Zero 2 Production

This is a simple project that demonstrates how to build a REST API using Rust and Actix-web. The project uses PostgreSQL as the database and sqlx for database interactions.

It also includes a Dockerfile file for easy local development.

# Getting Started

## Prerequisites

- Rust
- Docker
- Docker Compose
- PostgreSQL
- sqlx-cli
- Actix-web

## Setup

1. Clone the repository:

```bash
git clone github.com/Davidcparrar/zero2prod.git
```

2. Change into the project directory:

```bash
cd zero2prod
```

3. Build the rust project:

```bash
cargo build
```
4. Run the project:

```bash
cargo run
```
5. Run the tests:

```bash
cargo test
```

To check if the docker container is running and the DB is ready:

```bash
docker inspect postgres-test | grep .State -A 40
```

# Database SQL 

This project uses sqlx to manage the database schema and migrations.

## migrations
To create a new migration, run the following command:

```bash
sqlx migrate add <migration_name>
```

This will create a new migration file in the `migrations` directory with the name `<migration_name>`. The file will contain a template for the migration.
To apply the migrations, run the following command:

```bash
sqlx migrate run
```

### Migrations Script

The `init_db.sh` script is used to create the database and apply the migrations. It uses docker to start a PostgreSQL container and then runs the migrations using sqlx.

You can run the script without the SKIP_DOCKER variable to create the database and apply the migrations in a docker container:

```bash
./scripts/init_db.sh
```

SKIP_DOCKER=true is used to avoid trying to run the docker container again if it is already running. This is useful if you want to run new migrations without having to start the container every time.

```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```

## Interacting with the Database

You can docker exec into the PostgreSQL container and run SQL commands directly:

```bash
docker exec -it postgres-test psql -U postgres
```

This will open a PostgreSQL shell where you can run SQL commands.


# Development

It is recommended to use cargo watch to automatically check the code for changes and run tests when files are modified. This can be done by running the following command in a separate terminal:

```bash
 cargo watch -x check -x test
```
This will watch for changes in the code and run `cargo check` and `cargo test` whenever a file is modified. This is useful for quickly checking for errors and running tests without having to manually run the commands every time.

# Testing
## Unit tests
Unit tests are located in the `tests` directory. They are run using the `cargo test` command. The tests use the `reqwest` test client to make requests to the API and check the responses.

## sqlx

sqlx is used to interact with the database, howwver, for the CI to work properly (clippy I am looking at you) we need the .sqlx directory to be present in the root of the project. This is done by running the following command:

```bash
 cargo sqlx prepare --workspace --check -- --all-targets
```

This command will create the `.sqlx` directory and prepare the database for testing. It will also check for any errors in the SQL queries and migrations. This is useful for ensuring that the database is ready for testing and that there are no errors in the SQL queries.

# Telemetry

This project uses the `tracing` crate for logging and telemetry. It is used to log events and errors in the application. The traces are sent to a `tracing-subscriber` which can be configured to send the logs to different outputs, such as a file or a remote server. The `tracing` crate is used to create spans and events in the code, which can be used to track the flow of the application and debug issues.

For testing purposes the TEST_LOG=true environment variable is used to enable logging in the tests. This is useful for debugging and checking the logs during testing. The logs are printed to the console and can be used to check for errors and events in the application.

Traces are json formatted, in local development bunyan can be used to format the logs and make them more readable. This can be done by running the following command:

```bash
 TEST_LOG=true cargo test health_check_works | bunyan
```

bunyan is an NPM package but cargo can be used to install a rust-port:

```bash
cargo install bunyan
```


## Useful commands

```bash
curl -i -X POST -d 'email=thomas_mann7@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions -v
```

Cargo expand
```bash
cargo +nightly rustc --profile=check -- -Zunpretty=expanded
```

# Digital Ocean

```
sudo snap install doctl
doctl auth init
doctl apps create --spec spec.yaml
doctl apps update <ID> --spec=spec.yaml
```
