# 1: Build the executable for npuzzle
FROM rust:latest as builder
WORKDIR /usr/src

# Prepare container
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# Download and compile Rust dependencies
COPY rubik_lib ./rubik_lib
COPY server ./rubik_server
RUN cargo install --target x86_64-unknown-linux-musl --path ./rubik_server

# Copy the exe and extra files if needed to an empty Docker image
FROM scratch
EXPOSE 8080
COPY --from=builder /usr/local/cargo/bin/server .
ENTRYPOINT ["./server"]