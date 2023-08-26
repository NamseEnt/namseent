mkdir -p dynamodblocal
mkdir -p s3local

docker-compose up -d --force-recreate --remove-orphans

RUST_BACKTRACE=1 RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run --manifest-path ./server-bin/Cargo.toml --release
