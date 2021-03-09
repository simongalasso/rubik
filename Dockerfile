# 1: Build the executable for npuzzle
FROM rust:latest as builder
WORKDIR /usr/src

# Prepare container
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# Download and compile Rust dependencies
RUN USER=root cargo new rubik_server
# WORKDIR /usr/src/rubik_server
COPY rubik_lib ./rubik_lib
COPY server/Cargo.toml ./rubik_server
RUN cargo install --target x86_64-unknown-linux-musl --path ./rubik_server

# Build the executable using the source code
COPY ./server ./rubik_lib
RUN cargo install --target x86_64-unknown-linux-musl --path ./rubik_server

# 2: Copy the exe and extra files if needed to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/rubik_server .
COPY ./server/public ./public
USER 1000
ENTRYPOINT ["./rubik_server"]