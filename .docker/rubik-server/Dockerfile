# 1: Build the executable for npuzzle
FROM rust:latest as builder
WORKDIR /usr/src

# Prepare container
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# Download and compile Rust dependencies
# RUN USER=root cargo new rubik_server
COPY rubik_lib ./rubik_lib
COPY server ./rubik_server
RUN cargo install --target x86_64-unknown-linux-musl --path ./rubik_server

# Build the executable using the source code
# WORKDIR /usr/src/rubik_server
# WORKDIR /usr/src/rubik_server

# COPY rubik_lib ./rubik_lib
# COPY server .
# RUN cargo install --target x86_64-unknown-linux-musl --path .

# 2: Copy the exe and extra files if needed to an empty Docker image
FROM scratch
EXPOSE 8080
COPY --from=builder /usr/local/cargo/bin/server .
COPY ./server/src/public ./public
USER 1000
ENTRYPOINT ["./server"]