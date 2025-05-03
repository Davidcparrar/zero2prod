# Zero 2 Production

This is a simple project that demonstrates how to build a REST API using Rust and Actix-web. The project uses PostgreSQL as the database and sqlx for database interactions.

It also includes a Dockerfile and docker-compose file for easy deployment.

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
sqlx prepare --check --all-targets --workspace
```

This command will create the `.sqlx` directory and prepare the database for testing. It will also check for any errors in the SQL queries and migrations. This is useful for ensuring that the database is ready for testing and that there are no errors in the SQL queries.
