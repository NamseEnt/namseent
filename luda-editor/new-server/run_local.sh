#!/bin/bash

set -e

mkdir -p s3local

docker compose up -d --remove-orphans

echo "Waiting for localstack to be ready..."
while [ $(curl -s localhost:4566/_localstack/init/ready | grep -c '"completed": true') -eq 0 ]; do
    sleep 1
done

export DATABASE_BUCKET_NAME=visual-novel-database
export ASSET_BUCKET_NAME=visual-novel-asset
export AWS_ACCESS_KEY_ID=random
export AWS_SECRET_ACCESS_KEY=random
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_REGION=ap-northeast-2
# export RUST_LOG=debug

# https://github.com/watchexec/cargo-watch

cargo watch --why \
    --workdir ./server \
    --watch .. \
    --ignore "*/*.{sqlite,sqlite-wal}" \
    --ignore "server/backup" \
    -B 1 \
    --exec run
