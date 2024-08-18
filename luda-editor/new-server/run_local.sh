mkdir -p s3local

docker compose up -d --remove-orphans

export BUCKET_NAME=visual-novel
export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123
export AWS_ENDPOINT_URL=http://localhost:9000
export AWS_REGION=ap-northeast-2
# export RUST_BACKTRACE=1
# export RUST_LOG=debug

cargo watch --why -C ./server -w .. -x run
