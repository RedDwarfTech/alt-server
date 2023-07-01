ARG BASE_IMAGE=messense/rust-musl-cross:x86_64-musl

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
WORKDIR /app

COPY . .

RUN sudo apt-get update && apt-get install libssl-dev pkg-config musl-tools -y

# Build our application.
RUN cargo build --release --target=x86_64-unknown-linux-musl