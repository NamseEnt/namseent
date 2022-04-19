# syntax=docker/dockerfile:1

FROM rust:1.58-alpine

ENV CARGO_HOME=/usr/local/cargo

RUN apk upgrade -U -a \
    && apk add --update-cache \
    chromium-chromedriver \
    curl \
    musl-dev \
    nodejs \
    npm

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown
