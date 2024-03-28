# read: https://github.com/tikv/grpc-rs/issues/477#issuecomment-1354094205

ARG RUST_VERSION=1.75-buster

FROM rust:${RUST_VERSION} as builder

WORKDIR /app

ENV RUST_BACKTRACE=full
ENV CGO_ENABLED=1
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=DontWarn
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -yyq && apt-get install -yyq \
  build-essential lsb-release wget software-properties-common gnupg g++ clang \
  lld protobuf-compiler pkg-config libclang-dev \
  llvm-dev libssl-dev libclang-dev curl bash cmake golang-go 

# Go stuff
COPY ./libprom2json libprom2json
COPY ./go.mod go.mod
COPY ./go.sum go.sum

# Rust stuff
COPY ./src src
COPY ./build.rs build.rs
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN go build \
  -v \
  --ldflags '-linkmode external -extldflags=-static' \
  -o libprom2json/prom2json.a \
  -buildmode=c-archive \
  libprom2json/main.go

RUN cargo build --locked --release && cargo install --path . && \
  cp ./target/release/ella /usr/local/bin/ella

FROM debian:buster-slim as runtime
EXPOSE 8000
COPY --from=builder /app/target/release/ella /usr/local/bin/ella
CMD ["ella"]

