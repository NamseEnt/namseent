FROM alpine:3.18.5 as INSTALL

RUN apk update && apk add ca-certificates docker-cli

######################

FROM busybox:1.35.0-musl

COPY --from=INSTALL /etc/ssl/certs /etc/ssl/certs
COPY --from=INSTALL /usr/bin/docker /usr/bin/docker

ARG TARGET
COPY ./target/$TARGET/release/oioi-agent oioi-agent
CMD oioi-agent