FROM rust:slim-buster AS builder
RUN rustup default nightly-2019-12-19
RUN apt-get update && apt-get install -y libpq-dev postgresql wait-for-it
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

ENV APP_PATH /app
RUN mkdir $APP_PATH
COPY . $APP_PATH
WORKDIR $APP_PATH

COPY Cargo.lock Cargo.toml ./
COPY src/ src/
COPY migrations/ migrations/

EXPOSE 8000
RUN cargo build --release

