# syntax=docker/dockerfile:1

FROM docker.io/rust:1.65-alpine

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTFLAGS="-D warnings"

RUN apk upgrade -U -a \
    && apk add --no-cache \
    chromium-chromedriver \
    curl \
    musl-dev

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown

RUN rustup show