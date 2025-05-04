 FROM rust:1.85.0
 WORKDIR /app
 # Install the required system dependencies for our linking configuration
 RUN apt update && apt install lld clang -y
 # Copy all files from our working environment to our Docker image
 COPY . .

 RUN cargo build --release
 ENTRYPOINT ["./target/release/zero2prod"]
