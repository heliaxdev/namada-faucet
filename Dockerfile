FROM lukemathwalker/cargo-chef:latest-rust-1.78-bookworm AS chef
RUN apt-get update && apt-get install -y protobuf-compiler build-essential clang-tools-14 

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release /app/server

CMD ["./server"]