(cd $(dirname "$0");

    mkdir dynamodblocal
    chmod 777 -R dynamodblocal

    mkdir s3local
    chmod 777 -R s3local

    docker-compose up -d --force-recreate --remove-orphans

    RUST_BACKTRACE=1
    RUST_LOG=info
    MANIFEST_PATH=./server-bin/Cargo.toml

    cargo test --manifest-path $MANIFEST_PATH
)
