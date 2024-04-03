ARG RUST_VERSION=1.75-buster
FROM rust:${RUST_VERSION} as builder

WORKDIR /app

LABEL maintainer="Oto Brglez <otobrglez@gmail.com>"
LABEL "org.opencontainers.image.url"="https://github.com/otobrglez/ella"
LABEL "org.opencontainers.image.source"="https://github.com/otobrglez/ella"

ENV RUST_BACKTRACE=full
ENV CGO_ENABLED=1
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=DontWarn
ENV DEBIAN_FRONTEND=noninteractive

RUN \
    --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update -yyq && \
    apt-get install -yyq \
      software-properties-common build-essential lsb-release wget software-properties-common gnupg g++ clang \
      lld protobuf-compiler pkg-config libclang-dev \
      llvm-dev libssl-dev libclang-dev curl bash cmake && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN \
   wget -q https://go.dev/dl/go1.22.1.linux-amd64.tar.gz && \
   tar -xvf go1.22.1.linux-amd64.tar.gz -C /usr/local
ENV PATH=$PATH:/usr/local/go/bin
ENV GOPATH=/go
ENV GOROOT=/usr/local/go

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

RUN \
    --mount=type=cache,target=/root/.cache \
    --mount=type=cache,target=target \
    --mount=type=cache,target=/usr/local/cargo/registry,id=cargo_registry \
    cargo build --locked --release && \
    cp ./target/release/ella /usr/local/bin/ella

FROM debian:buster-slim as runtime

RUN \
    --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update -yyq && \
    apt-get install -yqq libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 8000
COPY --from=builder /usr/local/bin/ella /usr/local/bin/ella
CMD ["ella"]

