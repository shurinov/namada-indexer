FROM lukemathwalker/cargo-chef:latest-rust-1.79-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update && apt-get install -y protobuf-compiler build-essential clang-tools-14

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --package seeder

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/seeder /app/seeder

RUN apt-get update && apt-get install -y libpq5

WORKDIR /app

CMD ["./seeder"]
