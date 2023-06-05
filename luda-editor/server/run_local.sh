mkdir dynamodblocal
chmod 777 -R dynamodblocal

mkdir s3local
chmod 777 -R s3local

docker-compose up -d --force-recreate --remove-orphans

CARGO_INCREMENTAL=1
RUST_BACKTRACE=1
RUST_LOG=info

cargo run \
  --release \
  --manifest-path ./server-bin/Cargo.toml
  