FROM alpine:3.18.5 as INSTALL

RUN apk update && apk add \
    ca-certificates \
    docker-cli \
    aws-cli

ARG TARGET
COPY ./target/$TARGET/release/oioi-agent oioi-agent
CMD oioi-agent