# syntax=docker/dockerfile:1

FROM docker.io/rust:1-alpine

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTFLAGS="-D warnings"

RUN apk add --no-cache \
    chromium-chromedriver \
    curl \
    musl-dev \
    sccache \
    perl \
    make

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown

RUN rustup show

ENV RUSTC_WRAPPER=sccache
ENV CARGO_INCREMENTAL=0
