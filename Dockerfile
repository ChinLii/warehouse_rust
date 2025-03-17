FROM rust:lastest

WORKDIR /app

COPY Cargo.lock Cargo.toml ./ 
COPY src ./src

RUN cargo build --release
RUN cargo run --release
