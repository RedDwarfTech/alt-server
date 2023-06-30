# build stage
FROM rust:1.54-bullseye as builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-musl
RUN RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
# RUN cargo build
# do not use slim image, will block when query database
FROM gcr.io/distroless/static-debian11
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
COPY --from=builder /app/.env /app
COPY --from=builder /app/settings.toml /app
#
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/alt-server /app/
RUN chmod +x /app/alt-server
CMD ["/app/alt-server"]



