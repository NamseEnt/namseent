FROM alpine:3.18.5 as certs
RUN apk update && \
    apk add ca-certificates docker
ARG TARGET
COPY ./target/$TARGET/release/oioi-agent oioi-agent
CMD oioi-agent