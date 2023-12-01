FROM alpine:3.18.5 as certs
RUN apk update && apk add ca-certificates


FROM busybox:1.35.0-musl
COPY --from=certs /etc/ssl/certs /etc/ssl/certs
ARG TARGET
COPY ./target/$TARGET/release/oioi-agent oioi-agent
CMD oioi-agent