# use the default dart image as the build image
FROM rust:1.70 AS builder

# copy the current folder into the build folder
COPY . /app

# set the work directory
WORKDIR /app

# install protoc
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler libprotobuf-dev

# build app
RUN cargo build --release

# use a slim image
FROM debian:bullseye-slim

# copy the runtime files
COPY --from=builder /app/target/release/namada-faucet /app/axum 
WORKDIR /app

# start the dart server
CMD ["./axum"]