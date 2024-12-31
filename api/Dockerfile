FROM rust:latest AS rust-builder

# Copy Cargo files
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release

RUN rm -rf ./src
RUN ls
COPY ./src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release

# copy to alpine
FROM debian:bookworm-slim AS release
RUN apt-get update && apt-get install -y ca-certificates
RUN update-ca-certificates
ENV ROCKET_PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=rust-builder /app/target/release/ /usr/local/bin/
WORKDIR /usr/local/bin
CMD ["api"]