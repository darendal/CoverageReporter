FROM lukemathwalker/cargo-chef:latest-rust-1.65-bullseye AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin coverage-reporter

FROM debian:bullseye-slim as runtime
COPY --from=builder /app/target/release/coverage-reporter /usr/local/bin/coverage-reporter
CMD ["coverage-reporter"]

