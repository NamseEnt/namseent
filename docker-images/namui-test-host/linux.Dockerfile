FROM alpine:3.17

ENV RUSTFLAGS="-D warnings"

RUN apk upgrade -U -a \
    && apk add --no-cache \
    chromium-chromedriver \
    curl \
    musl-dev \
    nodejs \
    npm \
    build-base

ENV PATH=/home/user/.cargo/bin:$PATH

RUN addgroup -g 1000 -S user \
    && adduser -u 1000 -S user -G user
USER user
WORKDIR /home/user

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile=minimal --default-toolchain=1.65

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown

RUN rustup show

RUN node -v
