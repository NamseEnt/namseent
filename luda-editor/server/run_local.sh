mkdir -p dynamodblocal
mkdir -p s3local

docker-compose up -d --force-recreate --remove-orphans

RUST_BACKTRACE=1 RUST_LOG=info cargo run --manifest-path ./server-bin/Cargo.toml
