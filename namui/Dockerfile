# syntax=docker/dockerfile:1

FROM rust:1.58-alpine as base

RUN apk upgrade -U -a \
    && apk add \
    firefox-esr \
    curl \
    musl-dev
RUN apk add -X http://dl-cdn.alpinelinux.org/alpine/edge/testing geckodriver

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown

WORKDIR /namui

COPY .cargo/ ./.cargo
COPY Cargo.toml ./
COPY src ./src

FROM base as test
RUN wasm-pack test --headless --firefox
