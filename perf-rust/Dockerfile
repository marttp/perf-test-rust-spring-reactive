FROM rust:1.68.0 as builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release
RUN rm src/*.rs

# The final base image
FROM debian:buster-slim
COPY --from=builder /app/target/release/perf-rust /usr/src/app
CMD ["/usr/src/app"]