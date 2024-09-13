mkdir -p s3local

docker compose up -d --remove-orphans

export DATABASE_BUCKET_NAME=visual-novel-database
export ASSET_BUCKET_NAME=visual-novel-asset
export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123
export AWS_ENDPOINT_URL=http://localhost:9000
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
