# Build Dependencies
FROM rust:1.44 as Build-Deps

WORKDIR /app
COPY Cargo.lock Cargo.toml /app/
RUN mkdir /app/.cargo && mkdir /app/src && echo "fn main() {}" > /app/src/main.rs
RUN cargo vendor > /app/.cargo/config

# Build App
FROM Build-Deps as Build

COPY src /app/src
RUN cargo build --release
RUN cargo install --path . --verbose

# Release
FROM debian:stable-slim

COPY --from=Build /usr/local/cargo/bin/guess-game /bin

ENTRYPOINT ["guess-game"]