ARG BASE_IMAGE=messense/rust-musl-cross:x86_64-musl

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN sudo apt-get update && apt-get install libssl-dev pkg-config musl-tools libpq5 -y

# Build our application.
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian11
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
#
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/alt-server /app/
CMD ["./alt-server"]
