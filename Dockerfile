ARG BASE_IMAGE=messense/rust-musl-cross:x86_64-musl

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

RUN apt-get install pkg-config -y

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/using-diesel \
    /usr/local/bin/
CMD /usr/local/bin/using-diesel