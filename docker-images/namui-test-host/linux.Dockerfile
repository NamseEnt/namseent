# syntax=docker/dockerfile:1

FROM docker.io/rust:1.70-alpine

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTFLAGS="-D warnings"

RUN apk add --no-cache \
    chromium-chromedriver \
    curl \
    musl-dev \
    sccache \
    perl \
    make

RUN cargo install --version 0.11.1 wasm-pack

RUN rustup target add wasm32-unknown-unknown

RUN rustup show

ENV RUSTC_WRAPPER=sccache
ENV CARGO_INCREMENTAL=0
