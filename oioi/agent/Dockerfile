FROM busybox:1.35.0-musl
ARG TARGET
COPY ./target/$TARGET/release/oioi-agent oioi-agent
CMD oioi-agent