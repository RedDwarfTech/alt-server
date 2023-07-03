ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

FROM gcr.io/distroless/static-debian11
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
#
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/alt-server /app/
CMD ["./alt-server"]
