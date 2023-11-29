FROM rust:1.74.0-bookworm as build-env
WORKDIR /app
COPY . /app
RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/oioi-agent /
CMD ./oioi-agent
